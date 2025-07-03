use dioxus::prelude::*;
use crate::components::DropdownComponent;

#[component]
pub fn DropdownPage()->Element {
    rsx!{
        div { class: "flex flex-col gap-2", DropdownComponent {} }
    }
}