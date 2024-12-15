use dioxus::prelude::*;
use dioxus_i18n::prelude::*;
use dioxus_i18n::t;
use dioxus_i18n::unic_langid::langid;

#[component]
pub fn TranslationButtons() -> Element {
    let mut i18n = i18n();

    let change_to_english = move |_| i18n.set_language(langid!("en-UK"));
    let change_to_french = move |_| i18n.set_language(langid!("fr-FR"));

    rsx! {
        div { class: "gap-4 md:gap-6 flex flex-row mt-10 w-full justify-center items-center",
            button {
                class: "bg-blue-900 text-slate-100 px-4 py-2 rounded-lg cursor-pointer hover:bg-slate-900 hover:text-slate-200",
                onclick: change_to_english,
                "English"
            }
            button {
                class: "bg-blue-900 text-slate-100 px-4 py-2 rounded-lg cursor-pointer hover:bg-slate-900 hover:text-slate-200",
                onclick: change_to_french,
                "Français"
            }
        }
    }
}
