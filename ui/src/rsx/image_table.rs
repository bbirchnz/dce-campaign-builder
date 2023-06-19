use dce_lib::bin_data::BinItem;
use dioxus::prelude::*;
use fermi::use_atom_ref;
use log::trace;

use crate::{selectable::ToSelectable, SELECTED};

#[derive(PartialEq, Props)]
pub struct ImageTableProps {
    pub data: Vec<BinItem>,
}

pub fn image_table(cx: Scope<ImageTableProps>) -> Element {
    let atom_selected = use_atom_ref(cx, SELECTED);

    cx.render(rsx! {
        div { class: "flex ml-2 mr-2 items-center",
            h4 { class: "font-semibold", "Campaign and target images" }
            div { class: "grow" }
        }
        div { class: "grid grid-flow-col auto-cols-max ml-2 mr-2 items-center",
            for image in cx.props.data.iter() {
                rsx! {
                img {
                    class: "mt-1 mb-1 p-1 rounded hover:bg-sky-600",
                    onclick: move |_| {
                        let mut selected = atom_selected.write();
                        trace!("Clicked image {:?}", image.name.to_owned());
                        *selected = image.to_selectable();
                    },
                    src: "https://imagesprotocol.example/{image.name}",
                    width: 200,
                }
            }
            }
        }
    })
}
