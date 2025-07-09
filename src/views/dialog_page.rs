use dioxus::prelude::*;
use dioxus_i18n::t;
use freyr::prelude::*;
use crate::components::DialogComponent;

#[component]
pub fn DialogPage() -> Element {
    rsx! {
        div { class: "w-full min-h-screen flex flex-col items-center p-4 bg-gray-50 dark:bg-slate-800",
            div { class: "w-full max-w-4xl bg-slate-700 rounded-xl mt-12 sm:mt-16 p-4 sm:p-6 shadow-lg",
                h1 { class: "text-slate-100 text-xl sm:text-2xl font-bold mb-5 text-center",
                    {t!("dialog_title")}
                }
                div { class: "flex flex-col my-10 justify-center items-center", DialogComponent {} }
            }
        }
    }
}
