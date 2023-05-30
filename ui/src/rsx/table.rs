use bevy_reflect::Struct;

use dce_lib::editable::Editable;
use dioxus::prelude::*;
use fermi::{use_atom_ref, use_atom_state};
use log::info;

use crate::{selectable::ToSelectable, INSTANCE, INSTANCE_DIRTY, SELECTED};

#[derive(PartialEq, Props)]
pub struct TableProps<T> {
    pub data: Vec<T>,
}

pub fn table<T>(cx: Scope<TableProps<T>>) -> Element
where
    T: Struct + ToSelectable + std::fmt::Debug + Editable,
{
    let headers = T::get_header();
    let atom_instance = use_atom_ref(cx, INSTANCE);
    let atom_dirty = use_atom_state(cx, INSTANCE_DIRTY);

    cx.render(rsx! {
        if T::can_reset_from_miz() {
            rsx!{
                button {
                    onclick: move |_| {
                        let mut atom_instance = atom_instance.write();
                        let mut_instance = atom_instance.as_mut().expect("DCE instance is loaded");
                        match T::reset_all_from_miz(mut_instance) {
                            Ok(()) => {},
                            Err(_) => {}
                        }
                        atom_dirty.set(true);
                    },
                    "Reset to Miz"
                }
            }
        }
        table { class: "text-stone-700 bg-slate-50 border-collapse divide-y border-slate-400 w-full",
            thead {
                tr { class: "divide-x sticky top-0 bg-slate-50",
                    for h in headers.iter() {
                        th { class: "p-1 border-slate-300 font-normal", "{h.display.to_owned()}" }
                    }
                }
            }
            tbody {
                for squad in cx.props.data.iter() {
                    tr {
                        class: "divide-x hover:bg-slate-200",
                        onclick: move |_| {
                            let mut selected = use_atom_ref(cx, SELECTED).write();
                            info!("Got row {:?}", squad);
                            *selected = squad.to_selectable();
                        },
                        for h in headers.iter() {
                            td { class: "p-1 border-slate-300", "{h.get_value_string(squad).to_owned()}" }
                        }
                    }
                }
            }
        }
    })
}
