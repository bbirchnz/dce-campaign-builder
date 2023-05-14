use dce_lib::DCEInstance;
use dioxus::prelude::*;
use dioxus_desktop::use_window;
use fermi::use_atom_ref;
use log::warn;
use native_dialog::FileDialog;

use crate::INSTANCE;

#[derive(PartialEq, Props)]
pub struct MenuBarProps {
    #[props(into)]
    title: String,
}

pub fn menu_bar(cx: Scope<MenuBarProps>) -> Element {
    let w = use_window(cx);
    let toggled = use_state(cx, || false);
    let atom_instance = use_atom_ref(cx, INSTANCE);

    let is_loaded = atom_instance.read().is_some();

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

    let new_click = move |_| {
        let result = FileDialog::new()
            .add_filter("DCS Mission", &["miz"])
            .show_open_single_file();
        match result {
            Ok(Some(path)) => match DCEInstance::new_from_miz(&path.to_string_lossy()) {
                Ok(new_instance) => {
                    let mut writable = atom_instance.write();
                    let _ = writable.insert(new_instance);
                }
                Err(e) => warn!("Failed to parse miz with error: {}", e),
            },
            Ok(None) => {}
            Err(e) => warn!("Open file failed with error: {}", e),
        };
    };

    let open_click = move |_| {
        let result = FileDialog::new()
            .add_filter("DCE Builder json", &["json"])
            .show_open_single_file();
        match result {
            Ok(Some(path)) => match DCEInstance::load_from_json(&path.to_string_lossy()) {
                Ok(new_instance) => {
                    let mut writable = atom_instance.write();
                    let _ = writable.insert(new_instance);
                }
                Err(e) => warn!("Failed to load instance with error: {}", e),
            },
            Ok(None) => {}
            Err(e) => warn!("Open file failed with error: {}", e),
        };
    };
    let save_click = move |_| {
        let result = FileDialog::new()
            .add_filter("DCE Builder json", &["json"])
            .show_save_single_file();
        match result {
            Ok(Some(path)) => {
                if let Some(instance) = atom_instance.read().as_ref() {
                    if let Err(e) = instance.save_to_json(&path.to_string_lossy()) {
                        warn!("Failed to save instance with error: {}", e)
                    }
                }
            }
            Ok(None) => {}
            Err(e) => warn!("Save file failed with error: {}", e),
        };
    };

    let export_click = move |_| {
        let result = FileDialog::new().show_open_single_dir();
        match result {
            Ok(Some(path)) => {
                if let Some(instance) = atom_instance.read().as_ref() {
                    if let Err(e) = instance.generate_lua(&path.to_string_lossy()) {
                        warn!("Failed to export with error: {}", e)
                    }
                }
            }
            Ok(None) => {}
            Err(e) => warn!("Select directory failed with error: {}", e),
        };
    };

    cx.render(rsx! {
        div {
            class: "fixed top-0 left-0 right-0 flex items-stretch bg-sky-500 text-slate-700 h-8 cursor-default select-none",
            onmousedown: move |_| w.drag(),
            div {
                // New
                class: "flex items-center font-thin px-4 hover:bg-neutral-300 icon",
                onclick: new_click,
                ""
            }
            div {
                // Open
                class: "flex items-center font-thin px-4 hover:bg-neutral-300 icon",
                onclick: open_click,
                ""
            }
            if is_loaded {
                rsx!{
                div {
                    // Save
                    hidden: "{!is_loaded}",
                    class: "flex items-center font-thin px-4 hover:bg-neutral-300 icon",
                    onclick: save_click,
                    ""
                }
                div {
                    // Export
                    hidden: !is_loaded,
                    class: "flex items-center font-thin px-4 hover:bg-neutral-300 icon",
                    onclick: export_click,
                    ""
                }
            }
            }
            div { class: "grow" }
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
