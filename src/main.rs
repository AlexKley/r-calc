mod app;
mod config;
mod i18n;

fn main() -> cosmic::iced::Result {
    // 1. Pega os idiomas do sistema e inicializa a tradução
    let requested_languages = i18n_embed::DesktopLanguageRequester::requested_languages();
    i18n::init(&requested_languages);

    // 2. Trava a janela definindo os limites Mínimo e Máximo com os mesmos valores
    let limites = cosmic::iced::Limits::NONE
        .min_width(380.0)
        .max_width(380.0)
        .min_height(580.0)
        .max_height(580.0);

    // 3. Aplica os limites nas configurações
    let settings = cosmic::app::Settings::default().size_limits(limites);

    // 4. Inicia o aplicativo passando as configurações
    cosmic::app::run::<app::AppModel>(settings, ())
}