use dioxus::prelude::*;
use dioxus_i18n::t;

#[component]
pub fn About() -> Element {
    rsx!{
        div {
            p { {t!("about_page")} }
        }
    }
}