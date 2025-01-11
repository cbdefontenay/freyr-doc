use dioxus::prelude::*;

#[component]
pub fn NavbarPage() -> Element {
    rsx! {
        div { class: "flex flex-col items-center justify-center h-screen",
            button { "Click me" }
        }
    }
}