use dioxus::prelude::*;
use dioxus_i18n::t;
use freyr::prelude::*;

#[component]
pub fn DialogComponent() -> Element {
    let mut todos: Signal<Vec<String>> = use_signal(|| vec![]);
    let mut new_todo: Signal<String> = use_signal(|| "".to_string());

    let dialog_props = DialogProps {
        label: t!("dialog_add_todo_label"),
        dialog_button_class: Some(
            "bg-green-700 text-white font-semibold py-2 px-6 rounded-xl hover:bg-green-600 transition duration-300 shadow-md".to_string(),
        ),
        dialog_content: Some(rsx! {
            h2 { class: "text-2xl font-bold text-gray-800 mb-6", {t!("dialog_add_todo_header")} }
            input {
                class: "border border-gray-300 rounded-lg p-3 w-full focus:outline-none focus:ring-2 focus:ring-green-500",
                r#type: "text",
                value: "{new_todo()}",
                autofocus: true,
                oninput: move |e| new_todo.set(e.value().clone()),
            }
            button {
                class: "bg-blue-600 text-white px-5 py-3 rounded-xl hover:bg-blue-500 transition mt-6 w-full font-medium",
                onclick: move |_| {
                    if !new_todo().is_empty() {
                        todos.write().push(new_todo().clone());
                        new_todo.set("".to_string());
                    }
                },
                {t!("dialog_add_button")}
            }
        }),
        wrap_class: "bg-white rounded-2xl shadow-xl w-full max-w-lg p-8".to_string(),
        close_button_label: Some(t!("dialog_close_button_label")),
        close_button_class: Some(
            "bg-red-600 text-white px-5 py-3 rounded-xl hover:bg-red-500 transition font-medium".to_string(),
        ),
        cross_svg_class: None,
    };

    rsx! {
        Dialog {
            label: dialog_props.label,
            dialog_button_class: dialog_props.dialog_button_class,
            dialog_content: dialog_props.dialog_content,
            wrap_class: dialog_props.wrap_class,
            close_button_label: dialog_props.close_button_label,
            close_button_class: dialog_props.close_button_class,
            cross_svg_class: dialog_props.cross_svg_class,
        }
        ul { class: "mt-8 space-y-3",
            for todo in todos().iter() {
                li { class: "p-3 rounded-md bg-gray-100 border border-gray-200 shadow-sm text-gray-800",
                    "{todo}"
                }
            }
        }
    }
}

#[component]
fn MyDialog() -> Element {
    let mut show_dialog = use_signal(|| false);

    let dialog_props = DialogWithoutButtonProps {
        show_modal: show_dialog,
        dialog_content: Some(rsx! {
        h2 { class: "text-xl font-semibold mb-4", "Add a new To-Do" }
    }),
        wrap_class: "bg-white rounded-lg shadow-lg w-full max-w-md p-6".to_string(),
        close_button_label: Some("Close".to_string()),
        close_button_class: Some(
            "bg-purple-800 text-white px-4 py-2 rounded-lg hover:bg-purple-900 transition".to_string(),
        ),
        cross_svg_class: None,
    };

    rsx!{
    button {
        class: "bg-blue-800 text-white font-bold py-2 px-4 rounded-lg hover:bg-blue-900 transition",
        onclick: move |_| show_dialog.set(!show_dialog()),
        "Show Dialog"
    }
        DialogWithoutButton { ..dialog_props }
    }
}