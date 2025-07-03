use crate::components::CodeBlock;
use dioxus::prelude::*;
use dioxus_i18n::t;
use freyr::prelude::*;
use crate::components::dropdown_menu_component::DropdownMenuComponent;

#[component]
pub fn DropdownComponent() -> Element {
    let dropdown_items = vec![
        DropdownItem {
            label: "Home".to_string(),
            url: Some("/".to_string()),
        },
        DropdownItem {
            label: "About".to_string(),
            url: Some("/about".to_string()),
        },
        DropdownItem::without_url("A Label without any route"),
        DropdownItem {
            label: "No url at all".to_string(),
            url: None,
        },
    ];

    let config_dropdown = DropdownConfig {
        title: "My cool dropdown".to_string(),
        label: dropdown_items,
        background_color: DropdownColorScheme::Freyr,
        title_color: DropdownTitleColor::Light,
        labels_color: DropdownLabelsColor::Light,
        hover_color: DropdownHoverColor::Custom("#03346E"),
    };

    rsx! {
        div { class: "w-full min-h-screen flex flex-col items-center p-4 bg-gray-50 dark:bg-slate-900",
            div { class: "w-full max-w-4xl bg-slate-800 rounded-xl mt-12 sm:mt-16 p-4 sm:p-6 shadow-lg",
                h1 { class: "text-slate-100 text-xl sm:text-2xl font-bold mb-5 text-center mb-5", {t!("dropdown_header")} }
                div{class:"flex flex-col w-full mt-10",
                    h2{class:"text-lg font-bold justify-start mb-5 text-blue-300", {t!("dropdown_first_dropdown")} }
                    DropdownMenu { config_dropdown }
                }

                h3{ class:"mt-10 text-slate-100 font-semibold", 
                    {t!("dialog_implementation")} 
                }
                
                div{
                    class:"mt-5",
                    CodeBlock {
                    code: r#"let dropdown_items = vec![
        DropdownItem { label: \"Home\".to_string(), url: Some(\"/\".to_string()) },
        DropdownItem { label: \"About\".to_string(), url: Some(\"/about\".to_string()) },
        // without routing
        DropdownItem::without_url(\"A Label without route\"),
        // Or like that
        DropdownItem { label: \"Contact\".to_string(), url: None },
    ];

    let config_dropdown = DropdownConfig {
        title: \"My cool dropdown\".to_string(),
        label: dropdown_items,
        background_color: DropdownColorScheme::Freyr,
        title_color: DropdownTitleColor::Light,
        labels_color: DropdownLabelsColor::Light,
        hover_color: DropdownHoverColor::Custom(\"your_custom_color\"),
    };

    rsx! {
        DropdownMenu { config_dropdown }
    }"#.to_string(),
                    }
                }
                    DropdownMenuComponent {}
            }
        }
    }
}
