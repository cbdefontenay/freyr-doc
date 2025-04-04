use components::Navigation;
use dioxus::prelude::*;
use dioxus_i18n::prelude::{i18n, use_init_i18n, I18nConfig, Locale};
use dioxus_i18n::t;
use dioxus_i18n::unic_langid::langid;
use views::About;
use views::ButtonPage;
use views::Home;
use views::NavbarPage;
mod components;
mod views;

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
enum Route {
    #[layout(Navigation)]
    #[route("/")]
    Home {},
    #[route("/about")]
    About {},
    #[route("/buttons")]
    ButtonPage {},
    #[route("/navbar-component")]
    NavbarPage {},
}

const FAVICON: Asset = asset!("/assets/favicon.ico");
const TAILWIND_CSS: Asset = asset!("/assets/output.css");
const MAIN: Asset = asset!("/assets/main.css");

fn main() {
    launch(App);
}

#[component]
fn App() -> Element {
    use_init_i18n(|| {
        I18nConfig::new(langid!("en-US"))
            .with_locale(Locale::new_static(
                langid!("en-US"),
                include_str!("./translations/en-US.ftl"),
            ))
            .with_locale(Locale::new_static(
                langid!("fr-FR"),
                include_str!("./translations/fr-FR.ftl"),
            ))
    });

    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }
        document::Link { rel: "stylesheet", href: MAIN }
        document::Title { "Freyr" }
        Router::<Route> {}
    }
}
