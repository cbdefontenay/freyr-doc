use dioxus::prelude::*;
use crate::components::HomeComponents;

#[component]
pub fn Home() -> Element {
    rsx! {
        div { class: "",
            div { class: "flex flex-wrap gap-4 mt-10",
                HomeComponents { title: "Card 1" }
                HomeComponents { title: "Card 2" }
                HomeComponents { title: "Card 3" }
            }
        }
    }
}
