// SPDX-License-Identifier: MPL-2.0

//! Application entry point for R Calc on the COSMIC desktop.

mod app;
mod config;
mod i18n;

fn main() -> cosmic::iced::Result {
    // 1. Detect system language preferences and initialize localization
    let requested_languages = i18n_embed::DesktopLanguageRequester::requested_languages();
    i18n::init(&requested_languages);

    // 2. Enforce fixed window dimensions (340x600) for a consistent calculator layout
    let limits = cosmic::iced::Limits::NONE
        .min_width(340.0)
        .max_width(340.0)
        .min_height(600.0)
        .max_height(600.0);

    // 3. Configure application settings with fixed size limits
    let settings = cosmic::app::Settings::default().size_limits(limits);

    // 4. Launch the COSMIC application runtime with the AppModel
    cosmic::app::run::<app::AppModel>(settings, ())
}