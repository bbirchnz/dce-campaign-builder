use std::collections::VecDeque;

use dce_lib::DCEInstance;
use dioxus::prelude::*;
use dioxus_desktop::use_window;
use fermi::{use_atom_ref, use_atom_state};
use log::{trace, warn};
use native_dialog::FileDialog;

use crate::{selectable::Selectable, IMAGE_LIST_TX, INSTANCE, INSTANCE_DIRTY, SELECTED};

#[derive(PartialEq, Props)]
pub struct MenuBarProps {
    #[props(into)]
    title: String,
}

pub fn menu_bar(cx: Scope<MenuBarProps>) -> Element {
    let w = use_window(cx);
    let toggled = use_state(cx, || false);
    let atom_instance = use_atom_ref(cx, INSTANCE);
    let atom_dirty = use_atom_state(cx, INSTANCE_DIRTY);
    let atom_image_tx = use_atom_ref(cx, IMAGE_LIST_TX);

    let dirty_state = match atom_dirty.get() {
        true => " *",
        false => "",
    };

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
        trace!("New instance from miz clicked");
        let result = FileDialog::new()
            .add_filter("DCS Mission", &["miz"])
            .show_open_single_file();
        match result {
            Ok(Some(path)) => match DCEInstance::new_from_miz(&path.to_string_lossy()) {
                Ok(new_instance) => {
                    // update bin_images vec:
                    let readable = atom_image_tx.read();
                    readable
                        .as_ref()
                        .unwrap()
                        .send(new_instance.bin_data.images.clone())
                        .unwrap();

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
                    // update bin_images vec:
                    let readable = atom_image_tx.read();
                    readable
                        .as_ref()
                        .unwrap()
                        .send(new_instance.bin_data.images.clone())
                        .unwrap();

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
                atom_dirty.set(false);
            }
            Ok(None) => {}
            Err(e) => warn!("Save file failed with error: {}", e),
        };
    };

    let export_click = move |_| {
        let result = FileDialog::new()
            .add_filter("DCE_Manager compatible zip", &["zip"])
            .show_save_single_file();
        match result {
            Ok(Some(path)) => {
                if let Some(instance) = atom_instance.read().as_ref() {
                    if let Err(e) = instance.export_dce_zip(&path.to_string_lossy()) {
                        warn!("Failed to export with error: {}", e)
                    }
                }
            }
            Ok(None) => {}
            Err(e) => warn!("Select zip file to save failed with error: {}", e),
        };
    };

    let update_from_miz_click = move |_| {
        trace!("Update template miz clicked");
        let result = FileDialog::new()
            .add_filter("DCS Mission", &["miz"])
            .show_open_single_file();
        let mut write_instance = atom_instance.write();
        let mut_instance = write_instance
            .as_mut()
            .expect("Should have a DCE instance loaded");

        match result {
            Ok(Some(path)) => {
                if let Err(e) = mut_instance.replace_miz(&path.to_string_lossy()) {
                    warn!("Failed to update miz with error: {}", e);
                }
                atom_dirty.set(true);
            }
            Ok(None) => {}
            Err(e) => warn!("Open file failed with error: {}", e),
        };
    };

    cx.render(rsx! {
        div { class: "fixed top-0 left-0 right-0 flex items-stretch bg-sky-500 text-slate-700 h-8 cursor-default select-none menubar",
            icon_button { onclick: new_click, tooltip: "Create new campaign from template DCS miz", "" }
            icon_button { onclick: open_click, tooltip: "Load campaign", "" }
            if is_loaded {
                rsx!{
                    icon_button {
                        onclick: save_click,
                        tooltip: "Save campaign",
                        ""
                    }
                    icon_button {
                        onclick: update_from_miz_click,
                        tooltip: "Load updated template DCS miz. Does not change any DCE content.",
                        "\u{E8B6}"
                    }
                    icon_button {
                        onclick: export_click,
                        tooltip: "Generate zip for DCE_Manager",
                        ""
                    }
                    navigation_buttons {}
                }
            }
            div {
                class: "flex flex-grow items-center justify-center",
                onmousedown: move |_| w.drag(),
                div { "{cx.props.title}{dirty_state}" }
            }
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

#[derive(Props)]
pub struct IconButtonProps<'a> {
    onclick: EventHandler<'a, MouseEvent>,
    tooltip: Option<&'a str>,
    children: Element<'a>,
}

pub fn icon_button<'a>(cx: Scope<'a, IconButtonProps<'a>>) -> Element<'a> {
    cx.render(rsx! {
        div {
            class: "flex items-center font-thin px-4 hover:bg-neutral-300 icon tooltip",
            onclick: move |e| cx.props.onclick.call(e),
            if cx.props.tooltip.is_some() {
                rsx! {
                    span {
                        class: "tooltiptext",
                        "{cx.props.tooltip.unwrap()}"
                    }
                }
            }
            &cx.props.children
        }
    })
}

pub fn navigation_buttons(cx: Scope) -> Element {
    let back_items: &UseRef<VecDeque<Selectable>> = use_ref(cx, || VecDeque::default());
    let current_item = use_state(cx, || Selectable::None);
    let forward_item = use_state(cx, || Selectable::None);

    let atom_selected = use_atom_ref(cx, SELECTED);
    // if a new selectable
    if *current_item.get() != *atom_selected.read() {
        log::trace!("Got a new selected item, pushing");
        back_items.with_mut(|items| items.push_front(current_item.get().clone()));
        current_item.set(atom_selected.read().clone());

        // wipe out the forward item:
        forward_item.set(Selectable::None);
    }
    // if greater than 10, pop_back
    if back_items.read().len() > 10 {
        back_items.with_mut(|items| {
            items.pop_back();
        })
    }

    cx.render(rsx! {
        icon_button {
            onclick: |_| {
                let mut writable = back_items.write();
                if let Some(item) = writable.pop_front() {
                    // record this item as the forward:
                    forward_item.set(current_item.get().clone());
                    // and go back one level:
                    let mut selected = atom_selected.write();
                    current_item.set(item.clone());
                    *selected = item;
                }
            },
            tooltip: "DANGER: BROKEN - Navigate Back",
            "\u{E72B}"
        }
        icon_button {
            onclick: |_| {
                if forward_item.get() != &Selectable::None {
                    let item = forward_item.get();

                    back_items.with_mut(|items| items.push_front(current_item.get().clone()));
                    current_item.set(item.clone());
                    let mut selected = atom_selected.write();
                    *selected = item.clone();

                    // wipe out the forward item:
                    forward_item.set(Selectable::None);
                }
            },
            tooltip: "DANGER: BROKEN - Navigate Forward",
            "\u{E72A}"
        }
    })
}
