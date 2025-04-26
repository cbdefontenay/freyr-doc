use crate::components::CodeBlock;
use dioxus::prelude::*;
use dioxus_i18n::t;
use freyr::prelude::*;

#[component]
pub fn NavbarPage() -> Element {
    let navbar_config = NavbarConfig {
        background_color: ColorScheme::Freyr,
        nav_header: Some(String::from("Freyr")),
        orientation: Some(Orientation::Right),
        header_color: HeaderColor::Light,
        nav_items: vec![
            "Home".to_string(),
            "About".to_string(),
            "Contact".to_string(),
        ],
        nav_links: vec![
            "/".to_string(),
            "/about".to_string(),
            "/contact".to_string(),
        ],
        nav_item_color: NavItemsColor::Light,
        icon_color: IconColor::White,
    };

    let dropdown_items = vec![
        DropdownItem {
            label: "Home".to_string(),
            url: Some("/".to_string()),
        },
        DropdownItem {
            label: "About".to_string(),
            url: Some("/about".to_string()),
        },
        DropdownItem::without_url("A Label without route"),
        DropdownItem {
            label: "Contact".to_string(),
            url: None,
        },
    ];

    let _config_dropdown = DropdownConfig {
        title: "My dropdown".to_string(),
        label: dropdown_items,
        background_color: DropdownColorScheme::Dark,
        title_color: DropdownTitleColor::Light,
        labels_color: DropdownLabelsColor::Light,
        hover_color: DropdownHoverColor::Custom("#47453e"),
    };

    rsx! {
        div { class: "min-h-screen py-20 px-4",
            div { class: "max-w-4xl mx-auto flex flex-col items-center",
                // Code Block Section
                div { class: "w-full bg-white p-6 rounded-xl shadow-md mb-8",
                    h3 { class: "text-xl font-bold text-gray-800 mb-4", {t!("navbar_implementation_title")} }
                    CodeBlock {
                        code: r#"// Define tab names and content
let navbar_config = NavbarConfig {
    background_color: ColorScheme::Freyr,
    nav_header: Some(String::from("Freyr")),
    orientation: Some(Orientation::Right),
    header_color: HeaderColor::Light,
    nav_items: vec!["Home".to_string(), "About".to_string(), "Contact".to_string()],
    nav_links: vec!["/".to_string(), "/about".to_string(), "/contact".to_string()],
    nav_item_color: NavItemsColor::Light,
    icon_color: IconColor::White,
};

rsx! {
    Navbar { navbar_config }
};"#.to_string(),
                    }
                }

                }
                // Navbar Preview Section
                div { class: "w-full bg-white p-6 rounded-xl shadow-md",
                    h3 { class: "text-xl font-bold text-gray-800 mb-4", {t!("navbar_preview_title")} }
                    span { class: "text-sm italic text-gray-600 mb-5", {t!("navbar_preview_subtitle")} }
                    div { class: "border border-gray-200 rounded-lg p-4 z-0",
                        Navbar { navbar_config }
                    }
            }
        }
    }
}
