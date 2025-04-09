use dioxus::prelude::*;
use dioxus_i18n::t;
use freyr::prelude::*;

const IMAGE: Asset = asset!("/assets/images/one.jpg");

#[component]
pub fn TabsPage() -> Element {
    let tabs_names = vec![t!("tabs_luebeck"), t!("tabs_otto"), t!("tabs_barbarossa")];

    let custom_text = vec![
        rsx! {
            div {
                class: "flex flex-col items-center w-full max-w-2xl p-6 bg-white rounded-lg shadow-lg",
                h1 {
                    class: "text-3xl font-bold text-gray-800 mb-6 text-center font-serif",
                    {t!("tabs_luebeck_title")}
                }
                img {
                    src: IMAGE,
                    class: "w-full max-w-md h-64 object-cover rounded-lg shadow-md border-4 border-white transition-all duration-300 hover:scale-105",
                    alt: "Lübeck"
                }
                p {
                    class: "mt-6 text-gray-700 text-lg leading-relaxed text-center px-4",
                    {t!("tabs_luebeck_description")}
                }
            }
        },
        rsx! {
            div {
                class: "flex flex-col w-full max-w-2xl p-8 bg-white rounded-lg shadow-lg",
                h1 {
                    class: "text-3xl font-bold text-gray-800 mb-6 font-serif",
                    {t!("tabs_otto_title")}
                }
                p {
                    class: "text-gray-700 text-lg leading-relaxed text-justify",
                    {t!("tabs_otto_description")}
                }
            }
        },
        rsx! {
            div {
                class: "flex flex-col w-full max-w-2xl p-8 bg-white rounded-lg shadow-lg",
                h1 {
                    class: "text-3xl font-bold text-gray-800 mb-6 font-serif",
                    {t!("tabs_barbarossa_title")}
                }
                p {
                    class: "text-gray-700 text-lg leading-relaxed",
                    {t!("tabs_barbarossa_description")}
                }
            }
        },
    ];

    rsx! {
        div {
            class: "min-h-screen bg-gradient-to-br from-gray-50 to-gray-100 py-20 px-4",
            div {
                class: "max-w-4xl mx-auto flex flex-col items-center",
                h2 {
                    class: "text-2xl font-bold text-gray-800 mb-2 font-serif",
                    {t!("tabs_page_header")}
                }
                div {
                    class: "w-full border-b-2 border-gray-200 max-w-xs mb-10"
                }
                div {
                    class: "mt-10 w-full flex justify-center",
                    Tabs {
                        tabs_names,
                        custom_texts: Some(custom_text),
                        custom_color: TabsColor::Freyr,
                    }
                }
            }
        }
    }
}