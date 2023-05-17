use dce_lib::{
    campaign_header::Header,
    db_airbases::FixedAirBase,
    mappable::MapPoint,
    oob_air::Squadron,
    target_list::{Strike, CAP},
    DCEInstance,
};
use dioxus::prelude::*;
use dioxus_desktop::{use_window, wry::http::Response, Config};
use fermi::{use_atom_ref, use_atom_root, use_init_atom_root, AtomRef};
use log::{info, warn};
use selectable::Selectable;
use simple_logger::SimpleLogger;
use tables::TableHeader;

use crate::rsx::{edit_form, menu_bar};

mod rsx;
mod selectable;

static INSTANCE: AtomRef<Option<DCEInstance>> = |_| None;
static SELECTED: AtomRef<Selectable> = |_| Selectable::None;

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
    let selected_table = use_atom_ref(cx, SELECTED).read();
    let selected_form = use_atom_ref(cx, SELECTED).read();

    let atom_read = atom_instance.read();
    let instance = atom_read.as_ref().unwrap();

    let edit_col_width = if let Selectable::None = selected_form.to_owned() {
        ""
    } else {
        "basis-1/4"
    };

    let edit_table_height = if let Selectable::None = selected_form.to_owned() {
        ""
    } else {
        "basis-1/4"
    };

    cx.render(rsx! {
        div { class: "top-8 flex absolute inset-0 bg-slate-50",
            // selector col
            div { class: "basis-12 shrink-0 min-h-0 bg-sky-500 flex flex-col items-center",
                icon_button {
                    path: "images/airfield_grey.png".into(),
                    on_click: |_| select_first_fixed_airbase(cx)
                }
                icon_button {
                    path: "images/target_grey.png".into(),
                    on_click: |_| select_first_strike_target(cx)
                }
                icon_button { path: "images/plane_grey.png".into(), on_click: |_| select_first_squadron(cx) }
                // icon_button { path: "images/ship_grey.png".into(), on_click: |_| select_first_cap_target(cx) }
                icon_button {
                    path: "images/settings_grey.png".into(),
                    on_click: |_| select_campaign_settings(cx)
                }
            }
            // edit col
            div { class: "{edit_col_width} min-h-0 bg-sky-100",
                match *selected_form {
                    Selectable::Squadron(_) => rsx!{
                        edit_form::<Squadron> { headers: Squadron::get_header(), title: "Edit Squadron".into(), item: selected_form.clone()}
                     },
                    Selectable::TargetStrike(_) => rsx!{
                        edit_form::<Strike> { headers: Strike::get_header(), title: "Edit Strike Target".into(), item: selected_form.clone()}
                    },
                    Selectable::TargetCAP(_) => rsx!{
                        edit_form::<CAP> { headers: CAP::get_header(), title: "Edit CAP".into(), item: selected_form.clone()}
                    },
                    Selectable::FixedAirBase(_) => rsx!{
                        edit_form::<FixedAirBase> { headers: FixedAirBase::get_header(), title: "Edit Airbase".into(), item: selected_form.clone()}
                    },
                    Selectable::CampaignSettings(_) => rsx!{
                        edit_form::<Header> { headers: Header::get_header(), title: "Campaign Settings".into(), item: selected_form.clone()}
                    },
                    _ => rsx!{{}}
                }
            }
            // map and table col
            div { class: "flex-grow flex flex-col",
                div { class: "flex-grow min-h-0 bg-slate-50 flex flex-col", rsx::map {} }
                div { class: "{edit_table_height} grow-0 overflow-y-auto",
                    match *selected_table {
                        Selectable::Squadron(_) => rsx!{
                            rsx::table { headers: Squadron::get_header(), data: instance.oob_air.red.iter().chain(instance.oob_air.blue.iter()).cloned().collect::<Vec<Squadron>>() }
                        },
                        Selectable::TargetStrike(_) => rsx! {
                            rsx::table { headers: Strike::get_header(), data: instance.target_list.strike.to_vec() }
                        },
                        Selectable::TargetCAP(_) => rsx! {
                            rsx::table { headers: CAP::get_header(), data: instance.target_list.cap.to_vec() }
                        },
                        Selectable::FixedAirBase(_) => rsx! {
                            rsx::table { headers: FixedAirBase::get_header(), data: instance.airbases.fixed.to_vec() }
                        },
                        // not the right things to do, but if we don't there will be an empty space:
                        Selectable::CampaignSettings(_) => rsx! {
                            rsx::table { headers: FixedAirBase::get_header(), data: instance.airbases.fixed.to_vec() }
                        },
                        _ => rsx!{{}}
                        }
                }
            }
        }
    })
}

#[derive(Props)]
struct IconButtonProps<'a> {
    path: String,
    on_click: EventHandler<'a, MouseEvent>,
}

fn icon_button<'a>(cx: Scope<'a, IconButtonProps<'a>>) -> Element<'a> {
    cx.render(rsx! {
        img {
            class: "mt-1 mb-1 p-1 rounded hover:bg-sky-600",
            src: "{cx.props.path}",
            width: 40,
            height: 40,
            onclick: |e| cx.props.on_click.call(e)
        }
    })
}

fn select_first_fixed_airbase(cx: Scope) {
    let atom_instance = use_atom_ref(cx, INSTANCE);
    let atom_selected = use_atom_ref(cx, SELECTED);

    if let Some(fixed) = atom_instance
        .read()
        .as_ref()
        .unwrap()
        .airbases
        .fixed
        .first()
    {
        let mut writable = atom_selected.write();
        *writable = Selectable::FixedAirBase(fixed.clone());
    }
}

fn select_first_strike_target(cx: Scope) {
    let atom_instance = use_atom_ref(cx, INSTANCE);
    let atom_selected = use_atom_ref(cx, SELECTED);

    if let Some(item) = atom_instance
        .read()
        .as_ref()
        .unwrap()
        .target_list
        .strike
        .first()
    {
        let mut writable = atom_selected.write();
        *writable = Selectable::TargetStrike(item.clone());
    }
}

fn select_first_squadron(cx: Scope) {
    let atom_instance = use_atom_ref(cx, INSTANCE);
    let atom_selected = use_atom_ref(cx, SELECTED);

    if let Some(item) = atom_instance.read().as_ref().unwrap().oob_air.blue.first() {
        let mut writable = atom_selected.write();
        *writable = Selectable::Squadron(item.clone());
    }
}

fn select_campaign_settings(cx: Scope) {
    let atom_instance = use_atom_ref(cx, INSTANCE);
    let atom_selected = use_atom_ref(cx, SELECTED);

    let item = atom_instance
        .read()
        .as_ref()
        .unwrap()
        .campaign_header
        .clone();

    let mut writable = atom_selected.write();
    *writable = Selectable::CampaignSettings(item);
}
