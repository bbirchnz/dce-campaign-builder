use bevy_reflect::Struct;
use dioxus::prelude::*;
use tables::HeaderField;

#[derive(PartialEq, Props)]
pub struct TableProps<T> {
    pub headers: Vec<HeaderField>,
    pub data: Vec<T>,
}

pub fn table<T>(cx: Scope<TableProps<T>>) -> Element
where
    T: Struct,
{
    cx.render(rsx! {
        table { class: "bg-slate-50 border-collapse divide-y border-slate-400 w-full",
            thead {
                tr { class: "divide-x",
                    for h in cx.props.headers.iter() {
                        th { class: "p-1 border-slate-300", "{h.display.to_owned()}" }
                    }
                }
            }
            tbody {
                for squad in cx.props.data.iter() {
                    tr { class: "divide-x hover:bg-slate-200",
                        for h in cx.props.headers.iter() {
                            td { class: "p-1 border-slate-300", "{h.get_value_string(squad).to_owned()}" }
                        }
                    }
                }
            }
        }
    })
}
