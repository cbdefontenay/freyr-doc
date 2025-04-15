use crate::components::CodeBlock;
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
                class: "flex flex-col items-center w-full max-w-2xl p-6 rounded-lg shadow-lg bg-slate-100",
                h1 {
                    class: "text-3xl font-bold text-slate-800 mb-6 text-center font-serif",
                    {t!("tabs_luebeck_title")}
                }
                img {
                    src: IMAGE,
                    class: "w-full max-w-md h-64 object-cover rounded-lg shadow-md border-4 border-white transition-all duration-300 hover:scale-105",
                    alt: "Lübeck"
                }
                p {
                    class: "mt-6 text-slate-700 text-md md:text-lg leading-relaxed text-center px-4",
                    {t!("tabs_luebeck_description")}
                }
            }
        },
        rsx! {
            div {
                class: "flex flex-col w-full max-w-2xl p-8 bg-white rounded-lg shadow-lg bg-slate-100",
                h1 {
                    class: "text-md md:text-lg font-bold text-slate-800 mb-6 font-serif",
                    {t!("tabs_otto_title")}
                }
                p {
                    class: "text-slate-700 text-lg leading-relaxed text-justify",
                    {t!("tabs_otto_description")}
                }
            }
        },
        rsx! {
            div {
                class: "flex flex-col w-full max-w-2xl p-8 bg-white rounded-lg shadow-lg",
                h1 {
                    class: "text-md md:text-lg font-bold text-slate-800 mb-6 font-serif",
                    {t!("tabs_barbarossa_title")}
                }
                p {
                    class: "text-slate-700 text-lg leading-relaxed text-justify",
                    {t!("tabs_barbarossa_description")}
                }
            }
        },
    ];

    rsx! {
        div { class: "min-h-screen py-20 px-4",
            div { class: "max-w-4xl mx-auto flex flex-col items-center",

                div { class: "w-full mt-12 bg-white p-6 rounded-xl shadow-md",
                    h3 { class: "text-xl font-bold text-slate-800 mb-4", {t!("tabs_implementation_title")} }
                    CodeBlock {
                        code: r#"// Define tab names and content
const IMAGE: Asset = asset!("/assets/city.jpg");
let tabs = vec!["Puffin", "Otto I", "Barbarossa"];

let content = vec![
    rsx! {
        img { src: IMAGE, class: "w-64 h-64" }
        p { "Lübeck description..." }
    },
    rsx! { p { "Otto I description..." } },
    rsx! { p { "Barbarossa description..." } }
];

// Render the tabs
rsx! {
    Tabs {
        tabs_names: tabs,
        custom_texts: Some(content),
        custom_color: TabsColor::Freyr
    }
}"#.to_string(),
                    }
                }

                div { class: "mt-10 flex justify-center bg-slate-200",
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
