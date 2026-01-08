// SPDX-License-Identifier: GPL-3.0-only

use crate::config::Config;
use crate::trash_status::TrashStatus;
use crate::{file_manager, ui_panel_button, ui_popup};
use cosmic::cosmic_config::{self, CosmicConfigEntry};
use cosmic::iced::{Limits, Subscription, window::Id};
use cosmic::iced_futures::stream;
use cosmic::iced_winit::commands::popup::{destroy_popup, get_popup};
use cosmic::prelude::*;
use notify_debouncer_full::{DebounceEventResult, new_debouncer, notify};
use std::any::TypeId;
use std::time::Duration;

/// `AppModel`: Application state and message orchestrator
pub struct AppModel {
    core: cosmic::Core,
    popup: Option<Id>,
    config: Config,

    // Trash state (reactive)
    trash_status: TrashStatus,
}

/// Applet messages
#[derive(Debug, Clone)]
#[allow(dead_code)] // Some variants will be used in future phases
pub enum Message {
    // Popup
    TogglePopup,
    PopupClosed(Id),

    // Configuration
    UpdateConfig(Config),

    // Trash (Backend)
    TrashStatusChanged(TrashStatus),
    EmptyTrash,
    RestoreItems,
    OpenTrashFolder,

    // [PHASE 2+] Drag &amp; Drop (foundation)
    DndUriReceived(String),
    DndOfferAccepted,
    DndOfferRejected,

    // [PHASE 3+] Disk Eject
    EjectDrive(String),

    // [PHASE 4+] App Uninstall
    UninstallApp(String),
}

impl cosmic::Application for AppModel {
    type Executor = cosmic::executor::Default;
    type Flags = ();
    type Message = Message;
    const APP_ID: &'static str = "com.github.thiagocys.CosmicAppletTrash";

    fn core(&self) -> &cosmic::Core {
        &self.core
    }

    fn core_mut(&mut self) -> &mut cosmic::Core {
        &mut self.core
    }

    fn init(
        core: cosmic::Core,
        _flags: Self::Flags,
    ) -> (Self, Task<cosmic::Action<Self::Message>>) {
        // 🔥 ESTADO INICIAL REATIVO: Verificar status da lixeira no init()
        let trash_status = TrashStatus::check();

        let config = cosmic_config::Config::new(Self::APP_ID, Config::VERSION)
            .map(|context| match Config::get_entry(&context) {
                Ok(config) | Err((_, config)) => config,
            })
            .unwrap_or_default();

        let app = AppModel {
            core,
            popup: None,
            config,
            trash_status,
        };

        (app, Task::none())
    }

    fn on_close_requested(&self, id: Id) -> Option<Message> {
        Some(Message::PopupClosed(id))
    }

    /// Ícone do painel (estado reativo)
    fn view(&self) -> Element<'_, Self::Message> {
        ui_panel_button::view(&self.trash_status, &self.core)
    }

    /// Popup window
    fn view_window(&self, _id: Id) -> Element<'_, Self::Message> {
        ui_popup::view(&self.trash_status, &self.core)
    }

    fn subscription(&self) -> Subscription<Self::Message> {
        struct TrashWatcherSubscription;

        let watcher_subscription = Subscription::run_with_id(
            TypeId::of::<TrashWatcherSubscription>(),
            stream::channel(1, |mut output| {
                #[allow(clippy::semicolon_if_nothing_returned)]
                async move {
                    let watcher_res = new_debouncer(
                        Duration::from_millis(250),
                        Some(Duration::from_millis(250)),
                        move |event_res: DebounceEventResult| match event_res {
                            Ok(events) => {
                                let should_rescan =
                                    events.iter().any(|event| !event.kind.is_access());

                                if should_rescan {
                                    let new_status = TrashStatus::check();
                                    if let Err(e) =
                                        output.try_send(Message::TrashStatusChanged(new_status))
                                    {
                                        eprintln!("Failed to send trash status update: {e:?}");
                                    }
                                }
                            }
                            Err(e) => {
                                eprintln!("Failed to watch trash: {e:?}");
                            }
                        },
                    );

                    #[cfg(unix)]
                    match (watcher_res, trash::os_limited::trash_folders()) {
                        (Ok(mut watcher), Ok(trash_bins)) => {
                            let trash_paths = trash_bins
                                .into_iter()
                                .flat_map(|path| [path.join("files"), path]);

                            for path in trash_paths {
                                if let Err(e) =
                                    watcher.watch(&path, notify::RecursiveMode::NonRecursive)
                                {
                                    eprintln!("Failed to watch {}: {:?}", path.display(), e);
                                }
                            }

                            std::future::pending().await
                        }
                        (Err(e), _) => {
                            eprintln!("Failed to create trash watcher: {e:?}");
                        }
                        (_, Err(e)) => {
                            eprintln!("Failed to find trash folders: {e:?}");
                        }
                    }

                    std::future::pending().await
                }
            }),
        );

        Subscription::batch(vec![
            self.core()
                .watch_config::<Config>(Self::APP_ID)
                .map(|update| Message::UpdateConfig(update.config)),
            watcher_subscription,
        ])
    }

    fn update(&mut self, message: Self::Message) -> Task<cosmic::Action<Self::Message>> {
        match message {
            Message::UpdateConfig(config) => {
                self.config = config;
            }

            Message::TrashStatusChanged(status) => {
                self.trash_status = status;
            }

            Message::OpenTrashFolder => {
                // Open trash using cosmic-files --trash
                file_manager::open_trash_folder();
            }

            Message::TogglePopup => {
                return if let Some(p) = self.popup.take() {
                    destroy_popup(p)
                } else {
                    let new_id = Id::unique();
                    self.popup.replace(new_id);
                    let mut popup_settings = self.core.applet.get_popup_settings(
                        self.core.main_window_id().unwrap(),
                        new_id,
                        None,
                        None,
                        None,
                    );
                    popup_settings.positioner.size_limits = Limits::NONE
                        .max_width(372.0)
                        .min_width(300.0)
                        .min_height(200.0)
                        .max_height(1080.0);
                    get_popup(popup_settings)
                };
            }

            Message::PopupClosed(id) => {
                if self.popup.as_ref() == Some(&id) {
                    self.popup = None;
                }
            }

            // [FUTURE PHASES] - Placeholders
            Message::EmptyTrash
            | Message::RestoreItems
            | Message::DndUriReceived(_)
            | Message::DndOfferAccepted
            | Message::DndOfferRejected
            | Message::EjectDrive(_)
            | Message::UninstallApp(_) => {
                // Will be implemented in future phases
            }
        }
        Task::none()
    }

    fn style(&self) -> Option<cosmic::iced_runtime::Appearance> {
        Some(cosmic::applet::style())
    }
}
