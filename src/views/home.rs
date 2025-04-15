use crate::components::HomeComponents;
use crate::Route;
use dioxus::prelude::*;
use dioxus_i18n::t;

#[component]
pub fn Home() -> Element {
    rsx! {
        div { class: "h-full flex items-center justify-center py-10 px-4",
            div { class: "max-w-7xl w-full",
                div { class: "text-center mb-10",
                    h1 { class: "text-4xl font-extrabold text-slate-100 font-mono mb-20 md:text-5xl",
                        {t!("header_home")}
                    }
                }
                div { class: "grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-6 place-items-center",
                    Link { to: Route::ButtonPage {}, class: "w-full",
                        HomeComponents { title: {t!("button_component_card")} }
                    }
                    Link { to: Route::NavbarPage {}, class: "w-full",
                        HomeComponents { title: {t!("navbar_component_card")} }
                    }
                    Link { to: Route::TabsPage {}, class: "w-full",
                        HomeComponents { title: {t!("tabs_component_card")} }
                    }
                }
            }
        }
    }
}
