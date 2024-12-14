use crate::Route;
use dioxus::prelude::*;
use freyr::prelude::*;

#[component]
pub fn Navigation() -> Element {
    const LOGO: Asset = asset!("/assets/header.svg");

    let navbar_logo_config = NavbarWithLogoConfig {
        background_color: ColorScheme::Freyr,
        nav_items: vec!["Home".to_string(), "About".to_string(), "Components".to_string()],
        nav_links: vec!["/".to_string(), "/about".to_string(), "/components".to_string()],
        nav_item_color: NavItemsColor::Light,
        icon_color: IconColor::White,
        logo_url: String::from("/"),
        logo_src: LOGO,
        logo_alt: String::from("logo"),
    };

    rsx! {
        div {
            class:"h-20",
            NavbarWithLogo { navbar_logo_config }
        }
        Outlet::<Route> {}
    }
}
