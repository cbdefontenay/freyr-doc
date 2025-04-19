use crate::components::HomeComponents;
use crate::Route;
use dioxus::prelude::*;
use dioxus_i18n::t;

#[component]
pub fn Home() -> Element {
    rsx! {
        div { class: "h-full w-full flex items-center justify-center py-10 px-4",
            div { class: "max-w-7xl w-full",
                div { class: "text-center mb-10 md:mb-20",
                    div {
                        h1 { class: "text-3xl sm:text-4xl md:text-5xl font-extrabold text-white tracking-tight mb-5",
                            {t!("welcome_header")}
                        }
                        h4 { class: "text-base sm:text-lg md:text-xl text-slate-300 italic mb-14",
                            {t!("welcome_subtitle")}
                        }
                    }
                    h1 { class: "text-xl sm:text-2xl md:text-3xl font-mono font-bold text-indigo-400 mb-20",
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
                    Link { to: Route::CarouselPage {}, class: "w-full",
                        HomeComponents { title: "Carousel" }
                    }
                    Link { to: Route::AccordionPage {}, class: "w-full",
                        HomeComponents { title: "Accordion" }
                    }
                }
            }
        }
    }
}