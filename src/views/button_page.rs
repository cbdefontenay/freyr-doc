use crate::components::CodeBlock;
use dioxus::prelude::*;
use dioxus_i18n::t;
use freyr::prelude::*;

#[component]
pub fn ButtonPage() -> Element {
    let routing_label = t!("button_with_routing_label");
    let success_label = t!("button_colors_success_label");
    let no_routing_label = t!("button_without_routing_label");

    let mut count = use_signal(|| 0);
    let increment = move |_| count += 1;

    rsx! {
            div { class: "w-full min-h-screen p-4 sm:p-6 md:p-8 flex flex-col items-center",
                div { class: "w-full max-w-4xl flex flex-col gap-6 md:gap-8",

                    // Header section
                    div { class: "flex flex-col items-center gap-4 text-center",
                        h1 { class: "text-2xl sm:text-3xl md:text-4xl font-bold text-slate-100",
                            {t!("freyr_button_page_header")}
                        }
                        p { class: "text-slate-100 max-w-md md:max-w-lg text-sm sm:text-base",
                            {t!("freyr_button_text")}
                        }
                    }

                    // Main content
                    div { class: "w-full flex flex-col gap-8 md:gap-10",

                        // Section 1: Button with routing
                        div { class: "flex flex-col items-center gap-4 p-6 bg-white rounded-xl shadow-sm",
                            div { class: "flex flex-col items-center gap-2 text-center",
                                h2 { class: "text-lg sm:text-xl font-semibold text-slate-800",
                                    {t!("button_with_routing_title")}
                                }
                                p { class: "text-slate-500 text-sm",
                                    {t!("button_with_routing_description")}
                                }
                            }

                            BasicButton {
                                color: ButtonColor::Freyr,
                                label: routing_label,
                                link: Some(ButtonUrl { url: "/".to_string() }),
                            }

                            div { class: "w-full mt-4",
                                p { class: "text-slate-500 text-sm mb-2", "Implementation in code:" }
                                CodeBlock {
                                    code: r#"BasicButton {
    color: ButtonColor::Freyr,
    label: "Go to Home",
    link: Some(ButtonUrl {
        url: "/".to_string()
    })
};"#.to_string(),
                                }
                            }
                        }

                        // Color variants section
                        div { class: "flex flex-col items-center gap-4 p-6 bg-white rounded-xl shadow-sm",
                            div { class: "flex flex-col items-center gap-2 text-center",
                                p { class: "italic text-slate-700",
                                    {t!("button_colors_title")}
                                }
                            }
                            BasicButton {
                                color: ButtonColor::Success,
                                label: success_label,
                                link: Some(ButtonUrl { url: "/".to_string() }),
                            }
                        }

                        // Section 2: Button without routing
                        div { class: "flex flex-col items-center gap-4 p-6 bg-white rounded-xl shadow-sm",
                            div { class: "flex flex-col items-center gap-2 text-center",
                                h2 { class: "text-lg sm:text-xl font-semibold text-slate-800",
                                    {t!("button_without_routing_title")}
                                }
                                p { class: "text-slate-500 text-sm",
                                    {t!("button_without_routing_description")}
                                }
                            }
                            BasicButton {
                                color: ButtonColor::Freyr,
                                label: no_routing_label,
                                link: None,
                            }
                        }

                        // Section 3: Button with event
                        div { class: "flex flex-col items-center gap-4 p-6 bg-white rounded-xl shadow-sm",
                            div { class: "flex flex-col items-center gap-2 text-center",
                                h2 { class: "text-lg sm:text-xl font-semibold text-slate-800",
                                    {t!("button_with_event_title")}
                                }
                                p { class: "text-slate-500 text-sm",
                                    {t!("button_with_event_description")}
                                }
                            }
                            div { class: "flex flex-col items-center gap-3",
                                EventButton {
                                    color: ButtonColor::Black,
                                    onclick: increment,
                                    label: String::from({ t!("button_with_event_label") }),
                                }
                                div { class: "text-slate-700 font-medium", "Count: {count}" }
                            }
                           div { class: "w-full mt-4",
        CodeBlock {
            code: r#"let mut count = use_signal(|| 0);
let increment = move |_| count += 1;

EventButton {
    color: ButtonColor::Black,
    onclick: increment,
    label: "Click me".to_string(),
}"#.to_string(),
        }
    }
                        }
                    }
                }
            }
        }
}
