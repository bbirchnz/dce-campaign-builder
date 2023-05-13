use dioxus::{events::MouseEvent, prelude::*};

#[derive(Props)]
pub struct EmptyDialogProps<'a> {
    onclose: EventHandler<'a, MouseEvent>,
    visible: bool,
    children: Element<'a>,
}

#[allow(non_snake_case)]
pub fn EmptyDialog<'a>(cx: Scope<'a, EmptyDialogProps<'a>>) -> Element {
    cx.render(rsx!(
        div {
            class: "fixed z_dialog inset-0 overflow-y-auto",
            hidden: "{!cx.props.visible}",
            aria_modal: "true",
            role: "dialog",
            aria_labelledby: "modal-title",
            div { class: "flex items-end justify-center min-h-screen pt-4 px-4 pb-20 text-center sm:block sm:p-0",
                div {
                    class: "fixed inset-0 bg-gray-500 bg-opacity-75 transition-opacity",
                    onclick: move |event| cx.props.onclose.call(event),
                    aria_hidden: "true"
                }
                span {
                    class: "hidden sm:inline-block sm:align-middle sm:h-screen",
                    aria_hidden: "true",
                    // "&#8203;"
                    ""
                }
                div { class: "relative inline-block align-bottom bg-white rounded-lg text-left overflow-hidden shadow-xl transform transition-all sm:my-8 sm:align-middle sm:max-w-lg sm:w-full",
                    div { class: "bg-white px-4 pt-5 pb-4 sm:p-6 sm:pb-4", &cx.props.children }
                }
            }
        }
    ))
}
