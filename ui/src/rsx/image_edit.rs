use dce_lib::bin_data::BinItem;
use dioxus::prelude::*;
use fermi::{use_atom_ref, use_atom_state};
use log::trace;

use crate::{
    selectable::{Selectable, ToSelectable},
    INSTANCE, INSTANCE_DIRTY, SELECTED,
};

#[derive(PartialEq, Props)]
pub struct ImageEditProps {
    pub item: Selectable,
}

pub fn image_edit(cx: Scope<ImageEditProps>) -> Element {
    let atom_instance = use_atom_ref(cx, INSTANCE);
    let atom_dirty = use_atom_state(cx, INSTANCE_DIRTY);

    let item_from_props = BinItem::from_selectable(&cx.props.item).unwrap();
    let item_state = use_state(cx, || item_from_props.to_owned());
    let orig_name = use_state(cx, || item_state.get().name.to_owned());

    if item_from_props.name.as_str() != orig_name.as_str() {
        trace!("Selectable has changed, resetting form");
        orig_name.modify(|_| item_from_props.name.to_owned());
        item_state.modify(|_| item_from_props);
    }

    cx.render(rsx!{
        div { class: "p-2 m-2 rounded bg-sky-200",
            h4 { class: "font-semibold flex",
                div { class: "flex-grow", "Edit Image" }
                div {
                    class: "flex items-center font-thin rounded px-1 hover:bg-sky-300 hover:text-black icon",
                    onclick: move |_| {
                        let mut selected = use_atom_ref(cx, SELECTED).write();
                        *selected = Selectable::None;
                    },
                    ""
                }
            }
            img {
                class: "mt-1 mb-1 p-1 rounded",
                src: "https://imagesprotocol.example/{item_state.name}"
            }
            h4 { class: "w-full text-center", "{item_state.name}" }
        }
    }
)
}
