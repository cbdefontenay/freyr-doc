use dioxus::prelude::*;

#[component]
pub fn HomeComponents(title: String) -> Element {
    rsx! {
        div { class: "
                rounded-lg shadow-md p-6 transition-transform transform hover:scale-105 hover:shadow-xl w-full h-40
                sm:w-72 lg:w-80 flex items-center justify-center bg-[#3795BD]",
            h2 { class: "text-xl font-semibold text-slate-100 text-center", "{title}" }
        }
    }
}