use crate::components::{HomeComponents, TranslationButtons};
use dioxus::prelude::*;

#[component]
pub fn Home() -> Element {
    rsx! {
        div { class: "",
            TranslationButtons {}
            div { class: "flex flex-wrap gap-4 mt-10 w-fullt",
                HomeComponents { title: "Card 1" }
                HomeComponents { title: "Card 2" }
                HomeComponents { title: "Card 3" }
            }
        }
    }
}
