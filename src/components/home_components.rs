use dioxus::prelude::*;

#[component]
pub fn HomeComponents(title: String) -> Element {
    rsx! {
        div { class: "
                bg-slate-100
                rounded-lg
                shadow-md
                p-4
                transition-transform
                transform
                hover:scale-105
                hover:shadow-lg
                w-80
                h-40
                max-w-full
                sm:w-96
                sm:h-48
            ",
            h2 { class: "text-lg font-bold text-gray-800", "{title}" }
        }
    }
}
