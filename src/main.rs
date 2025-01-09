use components::Navigation;
use dioxus::prelude::*;
use dioxus_i18n::prelude::{i18n, use_init_i18n, I18nConfig, Locale};
use dioxus_i18n::t;
use dioxus_i18n::unic_langid::langid;
use views::About;
use views::Home;
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
}

const FAVICON: Asset = asset!("/assets/favicon.ico");
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

fn main() {
    launch(App);
}

#[component]
fn App() -> Element {
    let i18 = use_init_i18n(|| {
        I18nConfig::new(langid!("en-US"))
            .with_locale(Locale::new_static(
                langid!("en-US"),
                include_str!("translations/en-US.ftl"),
            ))
            .with_locale(Locale::new_static(
                langid!("fr-FR"),
                include_str!("./translations/fr-FR.ftl"),
            ))
    });

    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }
        document::Title { "Freyr" }

        Router::<Route> {}
    }
}
