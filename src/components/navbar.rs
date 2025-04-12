use dioxus::prelude::*;
use dioxus_i18n::prelude::*;
use dioxus_i18n::t;
use dioxus_i18n::unic_langid::langid;
use freyr::prelude::*;
use crate::Route;

#[component]
pub fn Navigation() -> Element {
    let mut i18n = i18n();

    let change_to_english = move |_| i18n.set_language(langid!("en-US"));
    let change_to_french = move |_| i18n.set_language(langid!("fr-FR"));
    let change_to_german = move |_| i18n.set_language(langid!("de-DE"));

    let dropdown_items = vec!["English".to_string(), "Français".to_string(), "Deutsch".to_string()];

    let onclick_handlers: Vec<EventHandler<MouseEvent>> = vec![
        EventHandler::new(change_to_english),
        EventHandler::new(change_to_french),
        EventHandler::new(change_to_german),
    ];

    let config_dropdown = DropdownButtonConfig {
        title: t!("language"),
        labels: dropdown_items,
        onclick: onclick_handlers,
        background_color: DropdownColorScheme::Freyr,
        title_color: DropdownTitleColor::Light,
        labels_color: DropdownLabelsColor::Light,
        hover_color: DropdownHoverColor::Custom("#03346E"),
    };

    let navbar_config = NavbarConfig {
        background_color: ColorScheme::Freyr,
        nav_header: String::from("Freyr"),
        nav_items: vec![
            t!("home"),
            t!("about"),
        ],
        nav_links: vec![
            "/".to_string(),
            "/about".to_string(),
        ],
        nav_item_color: NavItemsColor::Light,
        icon_color: IconColor::White,
    };


    rsx! {
        NavbarDropdownButtons { navbar_config, config_dropdown }
        Outlet::<Route> {}
    }
}
