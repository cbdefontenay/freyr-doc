use crate::components::CodeBlock;
use dioxus::prelude::*;
use dioxus_i18n::t;
use freyr::prelude::*;

const MOON: Asset = asset!("/assets/images/moon.jpg", ImageAssetOptions::new().with_webp());

#[component]
pub fn AccordionPage() -> Element {
    let title_one = t!("accordion_title_one");
    let first_text = t!("accordion_first_text");
    let subtitle = t!("accordion_subtitle");
    let title_two = t!("accordion_title_two");

    rsx! {
        div { class: "w-full min-h-screen flex flex-col items-center p-4 bg-gray-50 dark:bg-slate-900",
            div { class: "w-full max-w-4xl bg-slate-800 rounded-xl mt-12 sm:mt-16 p-4 sm:p-6 shadow-lg",
                h1 { class: "text-slate-100 text-xl sm:text-2xl font-bold mb-5 text-center", {t!("standard_accordion")} }

                Accordion {
                    title: title_one,
                    accordion_text: first_text,
                    optional_text: Some(subtitle),
                    icon_color: AccordionIconColor::Dark,
                    class: Some(String::from("w-full")),
                    title_class: Some(String::from("text-lg sm:text-xl font-bold text-center")),
                    accordion_wrapper: Some(String::from("bg-orange-200 rounded-t-lg")),
                    accordion_content: Some(String::from("rounded-b-lg bg-orange-100 w-full h-full text-orange-800 text-justify")),
                }

                h3 {
                    class: "text-white text-base sm:text-lg font-medium mb-4 mt-10 text-center",
                    {t!("carousel_code_label")}
                }

                CodeBlock {
                    code: r#"Accordion {
        title: title_one.clone(),
        accordion_text: first_text.clone(),
        optional_text: Some(String::from("Optional text displayed here. ignore me if you want!")),
        icon_color: AccordionIconColor::Dark,
        class: Some(String::from("w-full")),
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
                    class: "mt-10",
                    h1 { class: "text-slate-100 text-xl sm:text-2xl font-bold mb-5 text-center", {t!("custom_accordion")} }

                    AccordionCustom {
                        title: title_two,
                        accordion_text: rsx! {
                            h1 { class: "text-lg sm:text-xl font-medium text-center mb-4", {t!("moon_surface_text")} }
                            img {
                                class: "w-full sm:w-1/2 h-auto rounded-lg shadow-lg mx-auto mb-4",
                                src: "{MOON}",
                                alt: "image",
                            }
                        },
                        icon_color: AccordionIconColor::Light,
                        class: Some(String::from("w-full")),
                        title_class: Some(String::from("text-slate-200 text-xl font-bold text-center")),
                        accordion_wrapper: Some(String::from("bg-[#2d5ead] rounded-t-lg")),
                        accordion_content: Some(String::from("rounded-b-lg bg-[#7faaf0] w-full h-full text-slate-800 text-justify")),
                    }

                    h3 {
                        class: "text-white text-base sm:text-lg font-medium mb-4 mt-10 text-center",
                        {t!("carousel_code_label")}
                    }

                    CodeBlock {
                        code: r#"AccordionCustom {
                title: title_two,
                accordion_text: rsx! {
                    h1 { "Here is a custom text" }
                    img {
                        class: "w-full sm:w-1/2 h-auto rounded-lg shadow-lg m-4",
                        src: "{IMAGE}",
                        alt: "image",
                    }
                },
                icon_color: AccordionIconColor::Light,
                class: Some(String::from("w-full")),
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
