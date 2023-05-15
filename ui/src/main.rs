use dce_lib::{mappable::MapPoint, oob_air::Squadron, DCEInstance};
use dioxus::prelude::*;
use dioxus_desktop::{use_window, wry::http::Response, Config};
use fermi::{use_atom_ref, use_init_atom_root, AtomRef};
use log::{info, warn};
use selectable::Selectable;
use simple_logger::SimpleLogger;
use tables::{FieldType, TableHeader};

use crate::rsx::{menu_bar, EmptyDialog};

mod rsx;
mod selectable;

static INSTANCE: AtomRef<Option<DCEInstance>> = |_| None;
static SELECTED: AtomRef<Selectable> = |_| Selectable::None;

fn main() {
    SimpleLogger::new().init().unwrap();
    // launch the dioxus app in a webview
    dioxus_desktop::launch_cfg(
        app,
        Config::default().with_custom_protocol("testprotocol".into(), |req| {
            // this handle callbacks of clicked objects in leaflet
            let obj =
                serde_json::from_str::<MapPoint>(&String::from_utf8(req.body().to_vec()).unwrap());
            if let Ok(map_point) = obj {
                info!("{:?}", map_point);
            } else {
                warn!(
                    "Failed to parse {:?} with error {:?}",
                    String::from_utf8(req.body().to_vec()).unwrap(),
                    obj.err().unwrap()
                );
            }

            Ok(Response::new(vec![]))
        }),
    )
}

fn app(cx: Scope) -> Element {
    use_init_atom_root(cx);

    let w = use_window(cx);
    w.set_title("DCE");
    w.set_decorations(false);

    let atom_instance = use_atom_ref(cx, INSTANCE);

    let instance_loaded = atom_instance.read().is_some();

    cx.render(rsx! {
        // TODO: replace this script inclusion
        script { include_str!("./js/tailwind_3.3.1.js") }
        script { include_str!("./js/leaflet.js") }
        script { include_str!("./js/leaflet-corridor.js") }
        script { include_str!("./js/leaflet_utils.js") }
        style { include_str!("./css/base.css") }
        style { include_str!("./css/leaflet.css") }

        div { class: "h-full select-none text-stone-700",
            menu_bar { title: "DCE Campaign Builder" }
            if instance_loaded {
                rsx! {
                    main_body {}
                }
            }
            else {
                rsx! {
                    "Help"
                }
            }
        }
    })
}

fn main_body(cx: Scope) -> Element {
    let atom_instance = use_atom_ref(cx, INSTANCE);
    let selected = use_atom_ref(cx, SELECTED).read();

    let instance = atom_instance.read();
    let squadrons = instance.as_ref().unwrap().oob_air.red.to_vec();

    cx.render(rsx! {
        div { class: "top-8 grid grid-cols-4 grid-rows-6 absolute inset-0 bg-slate-50",
            div { class: "col-span-1 row-span-full min-h-0 bg-sky-100",
                if let Selectable::Squadron(_) = *selected {
                    rsx!{
                        edit_form {}
                    }
                }
            }
            div { class: "col-span-3 row-span-4 min-h-0 bg-slate-50 flex flex-col", rsx::map {} }
            div { class: "col-span-3 row-span-2 pl-2 pr-2 overflow-clip",
                EmptyDialog { visible: false, onclose: move |_| {}, div { "hello" } }
                rsx::table { headers: Squadron::get_header(), data: squadrons }
            }
        }
    })
}

fn fieldtype_to_input(field: &FieldType) -> String {
    match field {
        FieldType::String => "text".into(),
        FieldType::Float => "number".into(),
        FieldType::Int => "number".into(),
        FieldType::Enum => "text".into(),
        FieldType::VecString => "text".into(),
        FieldType::Debug => "text".into(),
    }
}

fn fieldtype_editable(field: &FieldType) -> bool {
    match field {
        FieldType::String => true,
        FieldType::Float => true,
        FieldType::Int => true,
        FieldType::Enum => false,
        FieldType::VecString => false,
        FieldType::Debug => false,
    }
}

pub fn edit_form(cx: Scope) -> Element {
    let atom_instance = use_atom_ref(cx, INSTANCE);
    let selected = use_atom_ref(cx, SELECTED).read();

    match selected.clone() {
        Selectable::Squadron(squad) => {
            let orig_name = squad.name.to_owned();

            let on_submit = move |ev: FormEvent| {
                let mut instance_refmut = atom_instance.write();
                let w_instance = instance_refmut.as_mut().unwrap();
                let squad_to_change = w_instance
                    .oob_air
                    .red
                    .iter_mut()
                    .find(|s| s.name == orig_name)
                    .unwrap();

                let headers = Squadron::get_header();

                for (k, v) in ev.values.iter() {
                    // find header that matches key:
                    let h = headers.iter().find(|h| h.display == *k).unwrap();
                    if let Err(e) = h.set_value_fromstr(squad_to_change, v) {
                        warn!("Failed to set field: {} with {}. Error: {}", h.field, v, e);
                    }
                }
            };

            cx.render(rsx!{
                div {
                    class: "p-2 m-2 rounded bg-sky-200",
                    h4{ class: "font-semibold ", "Edit Squadron"}
                    form {
                        autocomplete: "off",
                        onsubmit: on_submit,
                        oninput: move |ev| println!("Input {:?}", ev.values),
                        for h in Squadron::get_header().iter().filter(|h| fieldtype_editable(&h.type_)) {
                            div {
                                class: "flex w-full mt-1 mb-1",
                                label {class: "flex-grow p-1", r#for: "{h.display}", "{h.display}" }
                                input { class: "rounded p-1",autocomplete: "off", r#type: "{fieldtype_to_input(&h.type_)}", name: "{h.display}", value: "{h.get_value_string(&squad)}" }
                            }
                        }
                        div {
                            class: "flex",
                            div { class: "flex-grow"}
                            button { class: "rounded p-2 mt-1 mb-1 bg-sky-300 border-1", r#type: "submit", value: "Submit", "Submit changes" }
                        }
                    }
                }
            })
        }
        _ => None,
    }
}
