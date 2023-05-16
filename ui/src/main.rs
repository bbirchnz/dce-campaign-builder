use dce_lib::{mappable::MapPoint, oob_air::Squadron, target_list::Strike, DCEInstance};
use dioxus::prelude::*;
use dioxus_desktop::{use_window, wry::http::Response, Config};
use fermi::{use_atom_ref, use_atom_root, use_init_atom_root, AtomRef};
use log::{info, warn};
use selectable::Selectable;
use simple_logger::SimpleLogger;
use tables::TableHeader;

use crate::rsx::{edit_form, menu_bar, EmptyDialog};

mod rsx;
mod selectable;

pub enum TableType {
    Squadron,
    StrikeTarget,
    None,
}

static INSTANCE: AtomRef<Option<DCEInstance>> = |_| None;
static SELECTED: AtomRef<Selectable> = |_| Selectable::None;
static TABLETYPE: AtomRef<TableType> = |_| TableType::StrikeTarget;

struct AppProps {
    rx: async_channel::Receiver<MapPoint>,
}

fn main() {
    SimpleLogger::new().init().unwrap();
    // launch the dioxus app in a webview

    let (s, r) = async_channel::unbounded::<MapPoint>();

    dioxus_desktop::launch_with_props(
        app,
        AppProps { rx: r.clone() },
        Config::default().with_custom_protocol("testprotocol".into(), move |req| {
            // this handle callbacks of clicked objects in leaflet
            let obj =
                serde_json::from_str::<MapPoint>(&String::from_utf8(req.body().to_vec()).unwrap());
            if let Ok(map_point) = obj {
                info!("Sending {:?}", map_point);
                s.send_blocking(map_point).unwrap();
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

fn app(cx: Scope<AppProps>) -> Element {
    use_init_atom_root(cx);

    let w = use_window(cx);
    w.set_title("DCE");
    w.set_decorations(false);

    let atom_instance = use_atom_ref(cx, INSTANCE);
    let atom_selected = use_atom_ref(cx, SELECTED);

    let instance_loaded = atom_instance.read().is_some();
    let atoms = use_atom_root(cx);
    let rx = cx.props.rx.to_owned();

    use_coroutine(cx, move |_: UnboundedReceiver<i32>| {
        let atom_selected = atom_selected.to_owned();
        let atoms = atoms.to_owned();
        let rx = rx.to_owned();

        async move {
            while let Ok(item) = rx.recv().await {
                let instance_ref = atoms.read(INSTANCE);
                let instance = instance_ref.as_ref().borrow();
                let dce = instance.as_ref().unwrap();
                let mut writable = atom_selected.write();
                let selectable = Selectable::from_map(&item, dce);
                *writable = selectable;
            }
        }
    });

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
    let table_type = use_atom_ref(cx, TABLETYPE).read();

    let instance = atom_instance.read();
    let squadrons = instance.as_ref().unwrap().oob_air.red.to_vec();

    cx.render(rsx! {
        div { class: "top-8 grid grid-cols-4 grid-rows-6 absolute inset-0 bg-slate-50",
            div { class: "col-span-1 row-span-full min-h-0 bg-sky-100",
                match selected.clone() {
                    Selectable::Squadron(_) => rsx!{
                        edit_form::<Squadron> { headers: Squadron::get_header(), title: "Edit Squadron".into(), item: selected.clone()}
                     },
                     Selectable::TargetStrike(_) => rsx!{
                        edit_form::<Strike> { headers: Strike::get_header(), title: "Edit Strike Target".into(), item: selected.clone()}
                    },
                    _ => rsx!{{}}
                }
            }
            div { class: "col-span-3 row-span-4 min-h-0 bg-slate-50 flex flex-col", rsx::map {} }
            div { class: "col-span-3 row-span-2 pl-2 pr-2 overflow-clip",
                EmptyDialog { visible: false, onclose: move |_| {}, div { "hello" } }
                match *table_type {
                    TableType::Squadron => rsx!{
                        rsx::table { headers: Squadron::get_header(), data: squadrons }
                    },
                    TableType::StrikeTarget => rsx! {
                        rsx::table { headers: Strike::get_header(), data: instance.as_ref().unwrap().target_list.strike.to_vec() }
                    },
                    _ => rsx!{{}}
                    }
            }
        }
    })
}
