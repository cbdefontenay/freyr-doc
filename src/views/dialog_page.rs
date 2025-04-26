use dioxus::prelude::*;
use dioxus_i18n::t;
use freyr::prelude::*;

#[component]
pub fn DialogPage() -> Element {
    rsx! {
        div { class: "w-full min-h-screen flex flex-col items-center p-4 bg-gray-50 dark:bg-slate-900",
            div { class: "w-full max-w-4xl bg-slate-800 rounded-xl mt-12 sm:mt-16 p-4 sm:p-6 shadow-lg",
                // h1 { class: "text-slate-100 text-xl sm:text-2xl font-bold mb-5 text-center",
                //     {t!("dialog_header")}
                // }
                // div { class: "flex my-10 justify-center items-center", DialogComponent {} }
                div { class: "flex my-10 justify-center items-center text-slate-100 italic",  {t!("dialog_code_error")} }
                div { class: "text-lg text-slate-200 font-semibold", {t!("dialog_code_show")} }
                div { class: "flex items-center flex-shrink-0 justify-center my-10",
                    BasicButton {
                        color: ButtonColor::Primary,
                        label: String::from(t!("freyr_dialog_doc")),
                        link: ButtonUrl {
                            url: "https://docs.rs/freyr/latest/freyr/fn.Dialog.html".to_string(),
                        },
                    }
                }
            }
        }
    }
}
