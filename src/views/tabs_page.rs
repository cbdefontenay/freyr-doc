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
                   class: "flex flex-col items-center w-96",
                     h1 {
                class: "text-2xl font-bold text-gray-800 my-4",
                {t!("tabs_luebeck_title")}
            }
            img {
                src: IMAGE,
                class: "w-64 h-64 object-cover",
                alt: "Lübeck"
            }
            p {
                class: "mt-4 text-gray-600 text-center",
                {t!("tabs_luebeck_description")}
            }
        }
            },
        rsx! {
            div {
                class: "flex flex-col items-start w-96",
                p { class: "mt-4 text-gray-600",
                {t!("tabs_otto_description")}
                }
            }
        },
        rsx! {
            div {
                class: "flex flex-col items-start w-96",
                p { class: "mt-4 text-gray-600",
                    {t!("tabs_barbarossa_description")}
                }
            }
        },
    ];

    rsx! {
        div { class: "flex flex-col items-center mt-20 w-full py-10",
            h2 { class: "", {t!("tabs_page_header")} }
            div { class: "mt-20 flex flex-col items-center justify-center",
                Tabs {
                    tabs_names,
                    custom_texts: Some(custom_text),
                    custom_color: TabsColor::Freyr,
                }
            }
        }
    }
}
