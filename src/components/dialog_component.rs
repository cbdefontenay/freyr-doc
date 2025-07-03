use dioxus::prelude::*;
use dioxus_i18n::t;
use freyr::prelude::*;
use crate::components::CodeBlock;

#[component]
pub fn DialogComponent() -> Element {
    let mut show_dialog = use_signal(|| false);

    let dialog_props = DialogWithoutButtonProps {
        show_modal: show_dialog,
        dialog_content: Some(rsx! {
            h2 { class: "text-xl font-semibold mb-4", {t!("dialog_add_todo")} }
            input {
                r#type: "text",
                class: "border rounded p-2 w-full",
                placeholder: t!("dialog_placeholder"),
            }
        }),
        wrap_class: "bg-white rounded-lg shadow-lg w-full max-w-md p-6 z-50 relative".to_string(),
        close_button_label: Some(t!("dialog_close")),
        close_button_class: Some(
            "cursor-pointer mt-4 bg-purple-800 text-white px-4 py-2 rounded-lg hover:bg-purple-900 transition".to_string()
        ),
        cross_svg_class: Some("w-6 h-6 absolute top-4 right-4 text-gray-500 hover:text-black cursor-pointer".to_string()),
    };


    rsx! {
        div { class: "",
            button {
                class: "cursor-pointer bg-blue-800 text-white font-bold py-2 px-4 rounded-lg hover:bg-blue-900 transition",
                onclick: move |_| show_dialog.set(true),
                {t!("dialog_show_dialog")}
            }

            Dialog { ..dialog_props }
        }
        div { class: "w-full mx-4 md:w-[850px]",
            h1 { class: "text-xl text-slate-100 mt-10 mb-5", {t!("dialog_implementation")} }
            CodeBlock {
                code: "
                                    let dialog_props = DialogWithoutButtonProps {
                                         show_modal: show_dialog,
                                         dialog_content: Some(rsx! {
                                         h2 { class: \"class-1\", \"Add a new Todo\" }
                                         input {
                                         r#type: \"text\",
                                         class: \"input-class\",
                                         placeholder: \"Enter something...\"}}),
                                         wrap_class: \"my-wrap-class\".to_string(),
                                         close_button_label: Some(\"Close\".to_string()),
                                         close_button_class: Some(\"my-other-class\".to_string()),
                                         cross_svg_class: Some(\"my-svg-super-class\".to_string()),};
                                                                    
                                        rsx! {
                                            button {
                                            class: \"button-class\",
                                            onclick: move |_| show_dialog.set(true),
                                            \"Show Dialog\"
                                        }
                                                                    
                                        Dialog { ..dialog_props }
                                    }"
                    .to_string(),
            }
        }
    }
}