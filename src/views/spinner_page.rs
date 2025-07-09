use dioxus::prelude::*;
use dioxus_i18n::t;
use freyr::prelude::*;
use crate::components::CodeBlock;

#[component]
pub fn SpinnerPage() -> Element {
    rsx! {
        div { class: "w-full min-h-screen flex flex-col items-center p-4 bg-gray-100 dark:bg-slate-800",
            div { class: "w-full max-w-4xl bg-slate-600 rounded-xl mt-12 sm:mt-16 p-4 sm:p-6 shadow-lg",
                h1 { class: "text-slate-100 text-xl sm:text-2xl font-bold mb-5 text-center mb-8",
                    {t!("spinner_header")}
                }
                     div { class:"w-full flex justify-center items-center",
                        Spinner {
                            height: "70".to_string(),
                            width: "70".to_string(),
                            spinner_color: Some("#3795BD".to_string()),
                            spinner_bg_color: None
                    }
                }
                h3{ class:"mt-10 text-slate-100 font-semibold",
                    {t!("dialog_implementation")}
                }
                                div{
                    class:"mt-5",
                    CodeBlock {
                    code: r#"
          Spinner {
                  height: "70".to_string(),
                  width: "70".to_string(),
                  spinner_color: Some("your_color".to_string()),
                  spinner_bg_color: None
              }
                    "#.to_string(),
                    }
                }
            }
        }
    }
}
