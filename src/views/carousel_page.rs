use crate::components::CodeBlock;
use dioxus::prelude::*;
use dioxus_i18n::t;
use freyr::prelude::*;

#[component]

pub fn CarouselPage() -> Element {
    const MARS: Asset = asset!("/assets/images/mars.jpg", ImageAssetOptions::new().with_webp());
    const PLUTO: Asset = asset!("/assets/images/pluto.jpg", ImageAssetOptions::new().with_webp());
    const SATELLITE: Asset = asset!("/assets/images/satellite.jpg", ImageAssetOptions::new().with_webp());
    const MOON: Asset = asset!("./assets/images/moon.jpg", ImageAssetOptions::new().with_webp());

    let carousel_items = vec![
        CarouselItem::new(MARS, String::from("Mars")),
        CarouselItem::new(PLUTO, String::from("Pluto")),
        CarouselItem::new(SATELLITE, String::from("A satellite in orbit")),
        CarouselItem::new(MOON, String::from("The Moon")),
    ];

    let alt = carousel_items.clone();

    rsx! {
        div { class: "w-full h-full flex flex-col items-center px-4 py-10",
            h1 {
                class: "text-2xl sm:text-3xl md:text-4xl font-bold text-white text-center",
                {t!("header_carousel")}
            }
            span {
                class: "text-slate-300 text-base sm:text-lg italic mt-4 text-center max-w-xl",
                {t!("carousel_intro_text")}
            }

            div {
                class: "bg-slate-800 rounded-xl mt-12 sm:mt-16 w-full max-w-4xl px-4 sm:px-10 py-8 shadow-lg",
                h2 {
                    class: "text-white text-lg sm:text-xl font-semibold text-center mb-8",
                    {t!("carousel_basic_title")}
                }
                div {
                    class: "w-full flex justify-center mb-10",
                    CarouselSimple {
                        items: carousel_items.clone(),
                        alt: alt.clone(),
                        class: Some(String::from("w-full md:w-96 h-full md:h-96 flex justify-center rounded-lg")),
                    }
                }
                h3 {
                    class: "text-white text-lg font-medium mb-4",
                    {t!("carousel_code_label")}
                }
                CodeBlock {
                    code: r#"const BIRD: Asset = asset!("./assets/one.jpg");
const FOX: Asset = asset!("./assets/two.jpg");
const DOG: Asset = asset!("./assets/three.jpg");
let items = vec![
    CarouselItem::new(BIRD, String::from("Image 1")),
    CarouselItem::new(FOX, String::from("Image 2")),
    CarouselItem::new(DOG, String::from("Image 3")),
];

let alt = items.clone(); // Alternate text can be reused if identical.
rsx! {
    CarouselSimple {
        items: items,
        alt: alt,
        class: Some(String::from("w-full h-full flex justify-center rounded-lg")),
    }
}"#.to_string(),
                }
            }
            div {
                class: "bg-slate-800 rounded-xl mt-12 sm:mt-16 w-full max-w-4xl px-4 sm:px-10 py-8 shadow-lg",
                h2 {
                    class: "text-white text-lg sm:text-xl font-semibold text-center mb-8",
                    {t!("carousel_numbers_title")}
                }
                div {
                    class: "w-full flex justify-center mb-10",
                    CarouselWithNumbers {
                        items: carousel_items.clone(),
                        alt: alt.clone(),
                        class: Some(String::from("w-full md:w-[600px] h-full md:h-[600px] flex justify-center rounded-lg")),
                    }
                }
                h3 {
                    class: "text-white text-lg font-medium mb-4",
                    {t!("carousel_code_label")}
                }
                CodeBlock {
                    code: r#"const BIRD: Asset = asset!("./assets/one.jpg");
const FOX: Asset = asset!("./assets/two.jpg");
const DOG: Asset = asset!("./assets/three.jpg");

let items = vec![
    CarouselItem::new(BIRD, String::from("Image 1")),
    CarouselItem::new(FOX, String::from("Image 2")),
    CarouselItem::new(DOG, String::from("Image 3")),
];
let alt = items.clone(); // Alternate text can be reused if identical.

rsx! {
    CarouselWithNumbers {
        items: items,
        alt: alt,
        class: Some(String::from("w-full h-full flex justify-center rounded-lg")),
    }
}"#.to_string(),
                }
            }
            div {
                class: "bg-slate-800 rounded-xl mt-12 sm:mt-16 w-full max-w-4xl px-4 sm:px-10 py-8 shadow-lg",
                h2 {
                    class: "text-white text-lg sm:text-xl font-semibold text-center mb-8",
                    {t!("carousel_timer_title")}
                }
                div {
                    class: "w-full flex justify-center mb-10",
                    CarouselWithTimer {
                        items: carousel_items.clone(),
                        alt: alt.clone(),
                        timer_seconds: 5,
                        class: Some(String::from("w-full md:w-96 h-full md:h-96 flex justify-center rounded-lg")),
                    }
                }
                h3 {
                    class: "text-white text-lg font-medium mb-4",
                    {t!("carousel_code_label")}
                }
                CodeBlock {
                    code: r#"const BIRD: Asset = asset!("./assets/one.jpg");
const FOX: Asset = asset!("./assets/two.jpg");
const DOG: Asset = asset!("./assets/three.jpg");

let items = vec![
    CarouselItem::new(BIRD, String::from("Image 1")),
    CarouselItem::new(FOX, String::from("Image 2")),
    CarouselItem::new(DOG, String::from("Image 3")),
];

let alt = items.clone(); // Alternate text can be reused if identical.

rsx! {
    CarouselWithTimer {
        items: items,
        alt: alt,
        timer_seconds: 5,
        class: Some(String::from("w-full h-full flex justify-center rounded-lg")),
    }
}"#.to_string(),
                }
            }
        }
    }
}