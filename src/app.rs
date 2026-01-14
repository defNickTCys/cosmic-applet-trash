// SPDX-License-Identifier: GPL-3.0-only

use crate::config::Config;
use crate::trash_item_metadata::EnrichedTrashItem;
use crate::trash_status::TrashStatus;
use crate::{file_manager, trash_operations, ui_panel_button, ui_popup};
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
    trash_items: Vec<EnrichedTrashItem>,
    sort_ascending: bool, // true = A-Z, false = Z-A (folders always first)

    // Operation state
    empty_in_progress: bool,
    operation_error: Option<String>,
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
    TrashItemsLoaded(Vec<trash::TrashItem>),

    EmptyTrash,
    EmptyTrashComplete(Result<(), String>),

    RestoreItem(EnrichedTrashItem),
    RestoreComplete(Result<std::path::PathBuf, String>),

    DeleteItem(EnrichedTrashItem),
    DeleteComplete(Result<(), String>),

    OpenTrashFolder,
    ToggleSortOrder,                  // Toggle sort order A-Z ↔ Z-A
    Surface(cosmic::surface::Action), // For applet_tooltip

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
        let mut commands = Vec::new();

        // 🔥 ESTADO INICIAL REATIVO: Verificar status da lixeira no init()
        let trash_status = TrashStatus::check();

        // Load trash items immediately if not empty
        if !trash_status.is_empty {
            commands.push(Task::perform(trash_operations::list_items(), |result| {
                cosmic::Action::App(Message::TrashItemsLoaded(result.unwrap_or_default()))
            }));
        }

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
            trash_items: Vec::new(),
            sort_ascending: true, // Default A-Z ascending order
            empty_in_progress: false,
            operation_error: None,
        };

        (app, Task::batch(commands))
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
        ui_popup::view(
            &self.trash_status,
            &self.trash_items,
            self.sort_ascending,
            &self.core,
        )
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

    #[allow(clippy::too_many_lines)]
    fn update(&mut self, message: Self::Message) -> Task<cosmic::Action<Self::Message>> {
        match message {
            Message::UpdateConfig(config) => {
                self.config = config;
            }

            Message::TrashStatusChanged(status) => {
                self.trash_status = status;

                // Always reload list to ensure correct metadata/icons/ordering
                // This fixes: wrong icons, missing sizes, incorrect folder sorting for new items
                return Task::perform(trash_operations::list_items(), |result| {
                    Message::TrashItemsLoaded(result.unwrap_or_default())
                })
                .map(cosmic::Action::App);
            }

            Message::TrashItemsLoaded(items) => {
                // Enrich items with pre-computed metadata (size, icon)
                let mut enriched_items: Vec<EnrichedTrashItem> = items
                    .into_iter()
                    .map(EnrichedTrashItem::from_trash_item)
                    .collect();

                // Sort: folders first (alphabetical), then files (alphabetical)
                EnrichedTrashItem::sort_items(&mut enriched_items, self.sort_ascending);

                self.trash_items = enriched_items;
            }

            Message::OpenTrashFolder => {
                // Open trash using cosmic-files --trash
                file_manager::open_trash_folder();
            }
            Message::ToggleSortOrder => {
                // Toggle sort order (folders always stay first)
                self.sort_ascending = !self.sort_ascending;
                EnrichedTrashItem::sort_items(&mut self.trash_items, self.sort_ascending);
            }
            Message::Surface(action) => {
                return cosmic::task::message(cosmic::Action::Cosmic(
                    cosmic::app::Action::Surface(action),
                ));
            }

            Message::EmptyTrash => {
                if self.empty_in_progress {
                    return Task::none(); // Prevent multiple clicks
                }

                self.empty_in_progress = true;
                self.operation_error = None;

                return Task::perform(trash_operations::empty_trash(), |result| {
                    Message::EmptyTrashComplete(result.map_err(|e| e.to_string()))
                })
                .map(cosmic::Action::App);
            }

            Message::EmptyTrashComplete(result) => {
                self.empty_in_progress = false;

                match result {
                    Ok(()) => {
                        self.trash_items.clear();
                        // TrashStatusChanged will be sent by watcher
                    }
                    Err(e) => {
                        self.operation_error = Some(format!("Failed to empty trash: {e}"));
                        eprintln!("Empty trash error: {e}");
                    }
                }
            }

            Message::RestoreItem(enriched_item) => {
                return Task::perform(
                    trash_operations::restore_item(enriched_item.item),
                    |result| Message::RestoreComplete(result.map_err(|e| e.to_string())),
                )
                .map(cosmic::Action::App);
            }

            Message::RestoreComplete(result) => {
                match result {
                    Ok(path) => {
                        eprintln!("✅ Restored to: {}", path.display());
                        // Watcher will auto-reload list via TrashStatusChanged
                    }
                    Err(e) => {
                        eprintln!("❌ Restore failed: {e}");
                        self.operation_error = Some(format!("Failed to restore: {e}"));
                    }
                }
            }

            Message::DeleteItem(enriched_item) => {
                return Task::perform(
                    trash_operations::delete_item(enriched_item.item),
                    |result| Message::DeleteComplete(result.map_err(|e| e.to_string())),
                )
                .map(cosmic::Action::App);
            }

            Message::DeleteComplete(result) => {
                match result {
                    Ok(_) => {
                        eprintln!("✅ Item permanently deleted");
                        // Watcher will auto-reload list via TrashStatusChanged
                    }
                    Err(e) => {
                        eprintln!("❌ Delete failed: {e}");
                        self.operation_error = Some(format!("Failed to delete: {e}"));
                    }
                }
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
            Message::DndUriReceived(_)
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
