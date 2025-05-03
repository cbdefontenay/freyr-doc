use dioxus::prelude::*;

#[component]
pub fn CodeBlock(code: String) -> Element {
    rsx! {
        div { class: "w-full bg-gray-800 rounded-lg overflow-hidden",
            div { class: "px-4 py-2 bg-gray-700 flex items-center",
                div { class: "flex gap-2",
                    div { class: "w-3 h-3 rounded-full bg-red-500" }
                    div { class: "w-3 h-3 rounded-full bg-yellow-500" }
                    div { class: "w-3 h-3 rounded-full bg-green-500" }
                }
            }
            pre { class: "p-4 overflow-x-auto",
                code { class: "text-gray-100 font-mono text-sm sm:text-base", {code} }
            }
        }
    }
}
