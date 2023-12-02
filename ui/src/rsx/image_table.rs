use dce_lib::bin_data::BinItem;
use dioxus::prelude::*;
use fermi::{use_atom_ref, use_atom_state};
use log::trace;
use native_dialog::FileDialog;

use crate::{selectable::ToSelectable, IMAGE_LIST_TX, INSTANCE, INSTANCE_DIRTY, SELECTED};

#[derive(PartialEq, Props)]
pub struct ImageTableProps {
    pub data: Vec<BinItem>,
}

pub fn image_table(cx: Scope<ImageTableProps>) -> Element {
    let atom_selected = use_atom_ref(cx, SELECTED);
    let atom_instance = use_atom_ref(cx, INSTANCE);
    let atom_dirty = use_atom_state(cx, INSTANCE_DIRTY);
    let atom_image_tx = use_atom_ref(cx, IMAGE_LIST_TX);

    cx.render(rsx! {
        div { class: "flex ml-2 mr-2 items-center",
            h4 { class: "font-semibold", "Campaign and target images" }
            div { class: "grow" }
            button { class: "p-1 bg-slate-100 hover:bg-slate-300 rounded border-slate-500 border-2 ml-2 tooltip",
                onclick: move |_| {

                    if let Ok(Some(path)) = FileDialog::new()
                    .add_filter("Image (png)", &["png"])
                    .show_open_single_file() {
                        // read image
                        let file_name = path.file_name().expect("Is a file").to_str().expect("Is valid unicode");
                        let new_item = BinItem::new_from_file(&file_name, &path.to_string_lossy());
                        match new_item {
                            Ok(bin_item) => {
                                // add to instance
                                let mut atom_instance = atom_instance.write();
                                let mut_instance = atom_instance.as_mut().expect("DCE instance is loaded");
                                mut_instance.bin_data.images.push(bin_item);
                                // set as dirty
                                atom_dirty.set(true);
                                // update bin_images vec:
                                let readable = atom_image_tx.read();
                                readable
                                    .as_ref()
                                    .unwrap()
                                    .send(mut_instance.bin_data.images.clone())
                                    .unwrap();
                            },
                            Err(e) => log::error!("Failed to load image {} with error: {}", file_name, e),
                        }
                        };
                },
                span { class: "tooltiptext ttright", "Upload a new image (png)" }
                "Upload New"
            }
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
