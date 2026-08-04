// SPDX-License-Identifier: MPL-2.0

use std::time::Duration;

use crate::config::{Config, Flags};
use crate::leds::LedState;
use crate::{fl, icons};
use cosmic::iced::platform_specific::shell::wayland::commands::popup::{destroy_popup, get_popup};
use cosmic::iced::window::Id;
use cosmic::iced::{Limits, Subscription};
use cosmic::prelude::*;
use cosmic::widget;

/// The application model stores app-specific state used to describe its interface and
/// drive its logic.
pub(crate) struct AppModel {
    /// Application state which is managed by the COSMIC runtime.
    core: cosmic::Core,
    /// The popup id.
    popup: Option<Id>,
    /// Configuration data that persists between application runs.
    config: Config,
    /// Handler used to persist configuration changes.
    config_handler: Option<cosmic::cosmic_config::Config>,
    /// Current keyboard lock indicators.
    leds: LedState,
}

/// Messages emitted by the application and its widgets.
#[derive(Debug, Clone)]
pub enum Message {
    TogglePopup,
    PopupClosed(Id),
    Tick,
    ToggleShowCapsLock(bool),
    ToggleShowNumLock(bool),
    ToggleShowScrollLock(bool),
}

/// Create a COSMIC application from the app model
impl cosmic::Application for AppModel {
    /// The async executor that will be used to run your application's commands.
    type Executor = cosmic::SingleThreadExecutor;

    /// Data that your application receives to its init method.
    type Flags = Flags;

    /// Messages which the application and its widgets will emit.
    type Message = Message;

    /// Unique identifier in RDNN (reverse domain name notation) format.
    const APP_ID: &'static str = crate::config::APP_ID;

    fn core(&self) -> &cosmic::Core {
        &self.core
    }

    fn core_mut(&mut self) -> &mut cosmic::Core {
        &mut self.core
    }

    /// Initializes the application with any given flags and startup commands.
    fn init(core: cosmic::Core, flags: Self::Flags) -> (Self, Task<cosmic::Action<Self::Message>>) {
        let app = AppModel {
            core,
            popup: None,
            config: flags.config,
            config_handler: flags.config_handler,
            leds: LedState::read(),
        };

        (app, Task::none())
    }

    fn on_close_requested(&self, id: Id) -> Option<Message> {
        Some(Message::PopupClosed(id))
    }

    /// Describes the interface based on the current state of the application model.
    ///
    /// The applet's button in the panel will be drawn using the main view method.
    /// This view should emit messages to toggle the applet's popup window, which will
    /// be drawn using the `view_window` method.
    fn view(&self) -> Element<'_, Self::Message> {
        let mut chips = widget::row::with_capacity(4).spacing(2);
        if self.config.show_caps {
            chips = chips.push(lock_chip(icons::BLOQ_MAYUS, self.leds.caps == Some(true)));
        }
        if self.config.show_num {
            chips = chips.push(lock_chip(icons::BLOQ_NUM, self.leds.num == Some(true)));
        }
        if self.config.show_scroll {
            chips = chips.push(lock_chip(icons::BLOQ_DESPL, self.leds.scroll == Some(true)));
        }
        if !self.config.show_caps && !self.config.show_num && !self.config.show_scroll {
            chips = chips.push(lock_chip(icons::DEFAULT, false));
        }

        let button = widget::button::custom(chips)
            .class(cosmic::theme::Button::AppletIcon)
            .on_press_down(Message::TogglePopup);

        self.core
            .applet
            .autosize_window(button)
            .limits(cosmic::iced::Limits::NONE)
            .into()
    }

    /// The applet's popup window will be drawn using this view method.
    fn view_window(&self, _id: Id) -> Element<'_, Self::Message> {
        let content_list = widget::list_column()
            .add(cosmic::applet::padded_control(toggle_row(
                fl!("caps-lock"),
                self.config.show_caps,
                Message::ToggleShowCapsLock,
            )))
            .add(cosmic::applet::padded_control(toggle_row(
                fl!("num-lock"),
                self.config.show_num,
                Message::ToggleShowNumLock,
            )))
            .add(cosmic::applet::padded_control(toggle_row(
                fl!("scroll-lock"),
                self.config.show_scroll,
                Message::ToggleShowScrollLock,
            )));

        self.core.applet.popup_container(content_list).into()
    }

    /// Register subscriptions for this application.
    ///
    /// Subscriptions are long-lived async tasks running in the background which
    /// emit messages to the application through a channel.
    fn subscription(&self) -> Subscription<Self::Message> {
        cosmic::iced::time::every(Duration::from_millis(200)).map(|_| Message::Tick)
    }

    /// Handles messages emitted by the application and its widgets.
    fn update(&mut self, message: Self::Message) -> Task<cosmic::Action<Self::Message>> {
        match message {
            Message::Tick => {
                let state = LedState::read();
                if state != self.leds {
                    self.leds = state;
                }
            }
            Message::ToggleShowCapsLock(value) => self.set_show_caps(value),
            Message::ToggleShowNumLock(value) => self.set_show_num(value),
            Message::ToggleShowScrollLock(value) => self.set_show_scroll(value),
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
        }
        Task::none()
    }

    fn style(&self) -> Option<cosmic::iced::theme::Style> {
        Some(cosmic::applet::style())
    }
}

impl AppModel {
    fn set_show_caps(&mut self, value: bool) {
        if let Some(handler) = &self.config_handler
            && let Err(error) = self.config.set_show_caps(handler, value)
        {
            eprintln!("error setting show_caps: {error}");
        }
    }

    fn set_show_num(&mut self, value: bool) {
        if let Some(handler) = &self.config_handler
            && let Err(error) = self.config.set_show_num(handler, value)
        {
            eprintln!("error setting show_num: {error}");
        }
    }

    fn set_show_scroll(&mut self, value: bool) {
        if let Some(handler) = &self.config_handler
            && let Err(error) = self.config.set_show_scroll(handler, value)
        {
            eprintln!("error setting show_scroll: {error}");
        }
    }
}

/// Una pastilla indicadora compacta con icono SVG.
fn lock_chip(icon: &'static [u8], active: bool) -> Element<'static, Message> {
    let handle = cosmic::widget::svg::Handle::from_memory(icon);
    let icon = widget::svg(handle).width(12).height(12).symbolic(true);
    widget::container(icon)
        .padding([2, 5])
        .class(if active {
            cosmic::theme::Container::Primary
        } else {
            cosmic::theme::Container::custom(|theme| {
                let cosmic = theme.cosmic();
                let on_bg = cosmic.on_bg_color();
                let bg = cosmic.bg_color();
                let mix = |t: f32| {
                    cosmic::iced::Color::from(cosmic::cosmic_theme::palette::Srgba::new(
                        on_bg.red * t + bg.red * (1.0 - t),
                        on_bg.green * t + bg.green * (1.0 - t),
                        on_bg.blue * t + bg.blue * (1.0 - t),
                        1.0,
                    ))
                };
                cosmic::iced::widget::container::Style {
                    background: Some(cosmic::iced::Background::Color(mix(0.2))),
                    text_color: Some(mix(0.6)),
                    icon_color: Some(mix(0.6)),
                    border: cosmic::iced::Border {
                        radius: cosmic.corner_radii.radius_s.into(),
                        ..Default::default()
                    },
                    ..Default::default()
                }
            })
        })
        .into()
}

/// A popup row with a label and a toggler.
fn toggle_row<'a>(
    label: String,
    value: bool,
    on_toggle: fn(bool) -> Message,
) -> Element<'a, Message> {
    widget::row::with_capacity(3)
        .push(widget::text(label))
        .push(widget::space::horizontal())
        .push(widget::toggler(value).on_toggle(on_toggle))
        .into()
}
