use crate::components::CodeBlock;
use dioxus::prelude::*;
use dioxus_i18n::t;
use freyr::prelude::*;

const MOON: Asset = asset!("./assets/images/moon.jpg");

#[component]
pub fn AccordionPage() -> Element {
    let title_one = t!("accordion_title_one");
    let first_text = t!("accordion_first_text");
    let subtitle = t!("accordion_subtitle");
    let title_two = t!("accordion_title_two");

    rsx! {
        div { class: "w-full h-full flex flex-col items-center p-4",
            div { class: "bg-slate-800 rounded-xl mt-12 sm:mt-16 md:w-[900px] p-4 shadow-lg",
                h1{ class: "text-slate-100 text-lg font-bold mb-5", {t!("standard_accordion")} }
                Accordion {
                    title: title_one,
                    accordion_text: first_text,
                    optional_text: Some(subtitle),
                    icon_color: AccordionIconColor::Dark,
                    class: None,
                    title_class: Some(String::from("text-xl font-bold text-center")),
                    accordion_wrapper: Some(String::from("bg-orange-200 rounded-t-lg")),
                    accordion_content: Some(
                        String::from(
                            "rounded-b-lg bg-orange-100 w-full h-full text-orange-800 text-justify",
                        ),
                    ),
                }
                h3 {
                    class: "text-white text-lg font-medium mb-4 mt-10",
                    {t!("carousel_code_label")}
                }
                CodeBlock {
                    code: r#"Accordion {
        title: title_one.clone(),
        accordion_text: first_text.clone(),
        optional_text: Some(String::from("Optional text displayed here. ignore me if you want!")),
        icon_color: AccordionIconColor::Dark,
        class: Some(String::from("w-96")),
        title_class: Some(String::from("text-xl font-bold text-center")),
        accordion_wrapper: Some(String::from("bg-orange-200 rounded-t-lg")),
        accordion_content: Some(
        String::from(
        "rounded-b-lg bg-orange-100 w-full h-full text-orange-800 text-justify",
            ),
       ),
}"#.to_string(),
                }

                div {
                    class:"mt-10",
                    h1{ class: "text-slate-100 text-lg font-bold mb-5", {t!("custom_accordion")} }
                    AccordionCustom {
                        title: title_two,
                        accordion_text: rsx! {
                            h1 { {t!("moon_surface_text")} }
                            img {
                                class: "w-1/2 h-1/2 rounded-lg shadow-lg m-4",
                                src: "{MOON}",
                                alt: "image",
                            }
                        },
                        icon_color: AccordionIconColor::Light,
                        class: Some(String::from("w-full")),
                        title_class: Some(String::from("text-slate-200 text-xl font-bold text-center")),
                        accordion_wrapper: Some(String::from("bg-[#2d5ead] rounded-t-lg")),
                        accordion_content: Some(
                            String::from(
                                "rounded-b-lg bg-[#7faaf0] w-full h-full text-slate-800 text-justify",
                            ),
                        ),
                    }
                    h3 {
                        class: "text-white text-lg font-medium mb-4 mt-10",
                        {t!("carousel_code_label")}
                    }
                    CodeBlock {
                        code: r#"AccordionCustom {
                title: title_two,
                accordion_text: rsx! {
                    h1 { "Here is a custom text" }
                    img {
                        class: "w-1/2 h-1/2 rounded-lg shadow-lg m-4",
                        src: "{IMAGE}",
                        alt: "image",
                    }
                },
                icon_color: AccordionIconColor::Light,
                class: Some(String::from("w-96")),
                title_class: Some(String::from("text-slate-100 text-xl font-bold text-center")),
                accordion_wrapper: Some(String::from("bg-orange-200 rounded-t-lg")),
                accordion_content: Some(
                    String::from(
                        "rounded-b-lg bg-orange-100 w-full h-full text-orange-800 text-justify",
                    ),
                ),
            }"#.to_string(),
                    }
                }
            }
        }
    }
}