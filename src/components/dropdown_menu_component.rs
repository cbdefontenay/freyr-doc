use crate::components::CodeBlock;
use dioxus::prelude::*;
use dioxus_i18n::t;
use freyr::prelude::*;

#[component]
pub fn DropdownMenuComponent() -> Element {
    let mut counter = use_signal(|| 0);
    let dropdown_items = vec![t!("dropdown_increment"), t!("dropdown_decrement")];

    let increment = move |_| counter += 1;
    let decrement = move |_| counter -= 1;

    let onclick_handlers: Vec<EventHandler<MouseEvent>> =
        vec![EventHandler::new(increment), EventHandler::new(decrement)];

    let config_dropdown = DropdownButtonConfig {
        title: t!("dropdown_counter"),
        labels: dropdown_items,
        onclick: onclick_handlers,
        background_color: DropdownColorScheme::Dark,
        title_color: DropdownTitleColor::Light,
        labels_color: DropdownLabelsColor::Light,
        hover_color: DropdownHoverColor::Custom("#03346E"),
    };
    
    rsx! {
                div{class:"flex flex-col w-full mt-10",
                    h2{class:"text-lg font-bold justify-start text-blue-300 mb-5", {t!("dropdown_second_dropdown")} }
                     div{ class:"flex flex-row w-full mb-5 items-center",
                        DropdownMenuButton { config_dropdown }
                                span{class:"text-yellow-600 ml-10",
                                {t!("dropdown_count")} " {counter}"}
                        }
                }

                h3{ class:"mt-10 text-slate-100 font-semibold", 
                    {t!("dialog_implementation")} 
                }
                
                div{
                    class:"mt-5",
                    CodeBlock {
                    code: r#"let dropdown_items = vec![\"Increment\".to_string(), \"Decrement\".to_string()];

    let increment = move |_| counter += 1;
    let decrement = move |_| counter -= 1;

    let onclick_handlers: Vec<EventHandler<MouseEvent>> =
        vec![EventHandler::new(increment), EventHandler::new(decrement)];

    let config_dropdown = DropdownButtonConfig {
        title: \"Counter\".to_string(),
        labels: dropdown_items,
        onclick: onclick_handlers,
        background_color: DropdownColorScheme::Dark,
        title_color: DropdownTitleColor::Light,
        labels_color: DropdownLabelsColor::Light,
        hover_color: DropdownHoverColor::Custom(\"custom_color\"),
    };

    rsx! {
        DropdownMenuButton { config_dropdown }
    }"#.to_string(),
                    }
                }
    }
}
