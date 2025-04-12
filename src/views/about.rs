use dioxus::prelude::*;
use dioxus_i18n::t;
use freyr::prelude::*;

#[component]
pub fn About() -> Element {
    rsx! {
        div { class: "min-h-screen bg-white text-gray-800 px-6 py-16",
            div { class: "max-w-3xl mx-auto",
                h1 { class: "text-4xl font-bold mb-4", {t!("freyr_title")} }
                h2 { class: "text-lg text-gray-600 mb-8", {t!("freyr_subtitle")} }

                p { class: "text-base text-gray-700 mb-6",
                    {t!("freyr_about_intro")}
                }

                p { class: "text-base text-gray-700 mb-6",
                    {t!("freyr_about_what_is_it")}
                }

                p { class: "text-base text-gray-700 italic",
                    {t!("freyr_about_why_built")}
                }
                div { class:"mt-10",
                    BasicButton { color: ButtonColor::Freyr, label: String::from({t!("freyr_about_doc")}), link: ButtonUrl { url: "https://docs.rs/freyr/latest/freyr/#functions".to_string() } }
                }
            }
        }
    }
}
