use crate::components::CodeBlock;
use dioxus::prelude::*;
use dioxus_i18n::t;
use freyr::prelude::*;

const BREIZH: Asset = asset!(
    "/assets/images/breizh.png",
    ImageAssetOptions::new().with_webp()
);
const OCCITANIE: Asset = asset!(
    "/assets/images/occitanie.jpg",
    ImageAssetOptions::new().with_webp()
);

#[component]
pub fn TabsSecondaryComponent() -> Element {
    let tabs_names = vec![
        t!("tabs_secondary_header_one"),
        t!("tabs_secondary_header_two"),
    ];
    let custom_text = vec![
        rsx! {
            div {
                class: "flex flex-col justify-center",
                div {
                    class:"flex items-center justify-center w-96 h-96",
                    img { class:"rounded-lg", src: BREIZH, alt: "An image of Brittany"}
                }
                p {{t!("tabs_secondary_text_one")}}
            }
        },
        rsx! {
            div {
            class: "flex flex-col",
                 div {
                    class:"flex items-center justify-center w-96 h-96",
                    img { class:"rounded-lg", src: OCCITANIE, alt: "An image of Occitanie"}
                }
            span {{t!("tabs_secondary_text_two")}}
        }},
    ];

    rsx! {
        div { class: "max-w-2xl mx-auto flex flex-col items-center",
            h1 { class: "mb-10 text-slate-200 text-lg md:text-2xl", "Tabs Secondary:" }
            div { class: "flex flex-col mb-10 flex justify-center",
                TabsSecondary {
                    tabs_names,
                    custom_texts: Some(custom_text),
                    tab_radius: Some(String::from("1em 1em 1em 1em")),
                    tab_max_width: Some(String::from("800px")),
                    tab_header_hover: Some(String::from("#2e5a75")),
                    header_bg_color: Some("#3795BD".to_string()),
                    header_text_color: Some("#e8d7d5".to_string()),
                    active_bg_color: Some("#2fa2d4".to_string()),
                    active_text_color: Some("#e6ebed".to_string()),
                    tab_shadow: None,
                }
            }
        }
        div { class: "w-full mx-4 md:w-[850px]",
            h1 { class: "text-xl text-slate-100 mt-10 mb-5", {t!("tabs_implementation_title")} }
            CodeBlock {
                code: "
                                TabsSecondary {
                                      tabs_names, // Use a Vec<String>
                                      custom_texts: Some(custom_text), // Use a Vec<Element>
                                      tab_radius: Some(\"1em 1em 1em 1em\".to_string()),
                                      tab_max_width: Some(\"800px\".to_string()),
                                      tab_header_hover: Some(\"#2e5a75\".to_string()),
                                      header_bg_color: Some(\"#3795BD\".to_string()),
                                      header_text_color: Some(\"#e8d7d5\".to_string()),
                                      active_bg_color: Some(\"#2fa2d4\".to_string()),
                                      active_text_color: Some(\"#e6ebed\".to_string()),
                                      tab_shadow: None,
                                }"
                    .to_string(),
            }
        }
    }
}
