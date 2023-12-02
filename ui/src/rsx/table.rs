use bevy_reflect::Struct;

use dce_lib::editable::Editable;
use dioxus::prelude::*;
use fermi::{use_atom_ref, use_atom_state};
use log::info;

use crate::{selectable::ToSelectable, INSTANCE, INSTANCE_DIRTY, SELECTED};

#[derive(PartialEq, Props)]
pub struct TableProps<'a, T> {
    pub title: &'a str,
    pub data: Vec<T>,
}

pub fn table<'a, T>(cx: Scope<'a, TableProps<'a, T>>) -> Element<'a>
where
    T: Struct + ToSelectable + std::fmt::Debug + Editable,
{
    let headers = T::get_header();
    let atom_instance = use_atom_ref(cx, INSTANCE);
    let atom_dirty = use_atom_state(cx, INSTANCE_DIRTY);
    let atom_selected = use_atom_ref(cx, SELECTED);

    cx.render(rsx! {
        div { class: "flex ml-2 mr-2 items-center",
            h4 { class: "font-semibold", "{cx.props.title}" }
            div { class: "grow" }
            if T::can_reset_from_miz() {
                rsx!{
                    button {
                        class: "p-1 bg-slate-100 hover:bg-slate-300 rounded border-slate-500 border-2 ml-2 tooltip",
                        onclick: move |_| {
                            let mut atom_instance = atom_instance.write();
                            let mut_instance = atom_instance.as_mut().expect("DCE instance is loaded");
                            match T::reset_all_from_miz(mut_instance) {
                                Ok(()) => {},
                                Err(_) => {}
                            }
                            atom_dirty.set(true);
                        },
                        span {
                            class: "tooltiptext ttright",
                            "Deletes all from this list and recreates from the loaded template mission"
                        }
                        "Reset to Miz"
                    }
                }
            }
            for a in T::actions_all_entities() {
                rsx! {
                    button {
                        class: "p-1 bg-slate-100 hover:bg-slate-300 rounded border-slate-500 border-2 ml-2 tooltip",
                        onclick: move |_| {
                            let mut atom_instance = atom_instance.write();
                            let mut_instance = atom_instance.as_mut().expect("DCE instance is loaded");
                            match (a.function)(mut_instance) {
                                Ok(()) => {},
                                Err(_) => {}
                            };
                        },
                        span {
                            class: "tooltiptext ttright",
                            "{a.description}"
                        }
                        "{a.name}"
                    }
                }
            }
        }
        table { class: "text-stone-700 bg-slate-50 border-collapse divide-y border-slate-400 w-full",
            thead {
                tr { class: "divide-x sticky top-0 bg-slate-50",
                    for h in headers.iter() {
                        th { class: "p-1 border-slate-300 font-normal", "{h.display.to_owned()}" }
                    }
                    if T::can_delete() {
                        rsx!{
                            th { class: "p-1 border-slate-300 font-normal w-10", }
                        }
                    }
                }
            }
            tbody {
                for item in cx.props.data.iter() {
                    tr {
                        class: "divide-x hover:bg-slate-200",
                        onclick: move |_| {
                            let mut selected = atom_selected.write();
                            info!("Got row {:?}", item);
                            *selected = item.to_selectable();
                        },
                        for h in headers.iter() {
                            td { class: "p-1 border-slate-300", "{h.get_value_string(item).to_owned()}" }
                        }
                        if T::can_delete() {
                            rsx!{
                                td {
                                    class: "p-1 border-slate-300",
                                    button {
                                        class: "icon w-full",
                                        onclick: move |_| {
                                            let mut instance = atom_instance.write();
                                            T::delete_by_name(instance.as_mut().unwrap(), item.get_name().as_str()).expect("Item should exist and be deleted");
                                            // mark the instance dirty
                                            use_atom_state(cx, INSTANCE_DIRTY).set(true);
                                        },
                                        "\u{E74D}"
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    })
}
