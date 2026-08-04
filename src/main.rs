// SPDX-License-Identifier: MPL-2.0

mod app;
mod config;
mod i18n;
mod icons;
mod leds;

fn main() -> cosmic::iced::Result {
    // Get the system's preferred languages.
    let requested_languages = i18n_embed::DesktopLanguageRequester::requested_languages();

    // Enable localizations to be applied.
    i18n::init(&requested_languages);

    // Starts the applet's event loop with the app configuration as flags.
    cosmic::applet::run::<app::AppModel>(config::Flags::new())
}
