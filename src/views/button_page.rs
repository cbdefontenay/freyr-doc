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
        div { class: "w-full h-full p-6 flex flex-col items-center justify-center gap-8",

            h1 { class: "text-3xl md:text-4xl font-bold text-center",
                {t!("freyr_button_page_header")}
            }

            p { class: "text-center max-w-md text-gray-600", {t!("freyr_button_text")} }

            div { class: "w-full max-w-xl mt-6 flex flex-col gap-8",

                // Section 1: Button with routing
                div { class: "flex flex-col items-center gap-2 text-center",

                    h2 { class: "text-xl font-semibold", {t!("button_with_routing_title")} }

                    p { class: "text-gray-600 text-sm", {t!("button_with_routing_description")} }

                    BasicButton {
                        color: ButtonColor::Freyr,
                        label: routing_label,
                        link: Some(ButtonUrl { url: "/".to_string() }),
                    }
                }

                div { class: "flex flex-col items-center gap-2 text-center",

                    p { class: "italic", {t!("button_colors_title")} }

                    BasicButton {
                        color: ButtonColor::Success,
                        label: success_label,
                        link: Some(ButtonUrl { url: "/".to_string() }),
                    }
                }

                // Section 2: Button without routing
                div { class: "flex flex-col items-center gap-2 text-center",

                    h2 { class: "text-xl font-semibold", {t!("button_without_routing_title")} }

                    p { class: "text-gray-600 text-sm", {t!("button_without_routing_description")} }

                    BasicButton {
                        color: ButtonColor::Freyr,
                        label: no_routing_label,
                        link: None,
                    }
                }

                // Section 3: Button with event
                div { class: "flex flex-col items-center gap-2 text-center",

                    h2 { class: "text-xl font-semibold", {t!("button_with_event_title")} }

                    p { class: "text-gray-600 text-sm", {t!("button_with_event_description")} }

                    EventButton {
                        color: ButtonColor::Black,
                        onclick: increment,
                        label: String::from({ t!("button_with_event_label") }),
                    }
                    div { class: "ml-4", "{count}" }
                }
            }
        }
    }
}
