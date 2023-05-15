use bevy_reflect::Struct;

use dioxus::prelude::*;
use fermi::use_atom_ref;
use log::info;
use tables::HeaderField;

use crate::{selectable::ToSelectable, SELECTED};

#[derive(PartialEq, Props)]
pub struct TableProps<T> {
    pub headers: Vec<HeaderField>,
    pub data: Vec<T>,
}

pub fn table<T>(cx: Scope<TableProps<T>>) -> Element
where
    T: Struct + ToSelectable + std::fmt::Debug,
{
    cx.render(rsx! {
        table { class: "text-stone-700 bg-slate-50 border-collapse divide-y border-slate-400 w-full",
            thead {
                tr { class: "divide-x",
                    for h in cx.props.headers.iter() {
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
                        for h in cx.props.headers.iter() {
                            td { class: "p-1 border-slate-300", "{h.get_value_string(squad).to_owned()}" }
                        }
                    }
                }
            }
        }
    })
}
