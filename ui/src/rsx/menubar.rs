use dioxus::prelude::*;
use dioxus_desktop::use_window;

#[derive(PartialEq, Props)]
pub struct MenuBarProps {
    #[props(into)]
    title: String,
}

pub fn menu_bar(cx: Scope<MenuBarProps>) -> Element {
    let w = use_window(cx);
    let toggled = use_state(cx, || false);

    let buttons = move || {
        if w.is_maximized() {
            rsx! {
                div {
                    class: "flex items-center font-thin px-4 hover:bg-neutral-300 icon",
                    onclick: move |_| {
                        w.set_maximized(false);
                        toggled.set(!toggled);
                        println!("toggle restore");
                    },
                    ""
                }
            }
        } else {
            rsx! {
                div {
                    class: "flex items-center font-thin px-4 hover:bg-neutral-300 icon",
                    onclick: move |_| {
                        w.set_maximized(true);
                        toggled.set(!toggled);
                        println!("toggle max");
                    },
                    ""
                }
            }
        }
    };

    cx.render(rsx! {
        div {
            class: "fixed top-0 left-0 right-0 flex items-stretch bg-sky-500 text-slate-700 h-8 cursor-default select-none",
            onmousedown: move |_| w.drag(),
            div { class: "ml-3 flex items-center", "{cx.props.title}" }
            div { class: "grow" }
            div {
                class: "flex items-center font-thin px-4 hover:bg-neutral-300 icon",
                onclick: move |_| w.set_minimized(true),
                ""
            }
            buttons(),
            div {
                class: "flex items-center font-thin px-4 hover:bg-red-500 hover:text-white icon",
                onclick: move |_| w.close(),
                ""
            }
        }
    })
}
