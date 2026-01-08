// SPDX-License-Identifier: GPL-3.0-only

use crate::config::Config;
use crate::trash_status::TrashStatus;
use crate::{dbus_file_manager, ui_panel_button, ui_popup};
use cosmic::cosmic_config::{self, CosmicConfigEntry};
use cosmic::iced::{Limits, Subscription, window::Id};
use cosmic::iced_winit::commands::popup::{destroy_popup, get_popup};
use cosmic::prelude::*;

/// AppModel: Orquestrador de estado e mensagens
pub struct AppModel {
    core: cosmic::Core,
    popup: Option<Id>,
    config: Config,

    // Estado da lixeira (reativo)
    trash_status: TrashStatus,
}

/// Mensagens do applet
#[derive(Debug, Clone)]
#[allow(dead_code)] // Algumas variantes serão usadas em fases futuras
pub enum Message {
    // Popup
    TogglePopup,
    PopupClosed(Id),

    // Configuração
    UpdateConfig(Config),

    // Lixeira (Backend)
    TrashStatusChanged(TrashStatus),
    EmptyTrash,
    RestoreItems,
    OpenTrashFolder,

    // [FASE 2+] Drag & Drop (fundação)
    DndUriReceived(String),
    DndOfferAccepted,
    DndOfferRejected,

    // [FASE 3+] Ejeção de discos
    EjectDrive(String),

    // [FASE 4+] Desinstalação
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
                Ok(config) => config,
                Err((_, config)) => config,
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
        // [FASE 1] Adicionar monitoramento inotify aqui
        Subscription::batch(vec![
            self.core()
                .watch_config::<Config>(Self::APP_ID)
                .map(|update| Message::UpdateConfig(update.config)),
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

            Message::EmptyTrash => {
                // [FASE 1] Implementar
            }

            Message::RestoreItems => {
                // [FASE 1] Implementar
            }

            Message::OpenTrashFolder => {
                // Abrir lixeira usando cosmic-files --trash
                dbus_file_manager::open_trash_folder();
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

            // [FASES FUTURAS]
            Message::DndUriReceived(_)
            | Message::DndOfferAccepted
            | Message::DndOfferRejected
            | Message::EjectDrive(_)
            | Message::UninstallApp(_) => {
                // Placeholder para extensões futuras
            }
        }
        Task::none()
    }

    fn style(&self) -> Option<cosmic::iced_runtime::Appearance> {
        Some(cosmic::applet::style())
    }
}
