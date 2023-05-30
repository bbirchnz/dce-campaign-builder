#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![cfg_attr(debug_assertions, windows_subsystem = "console")]
use dce_lib::{
    campaign_header::Header,
    db_airbases::{AirStartBase, FixedAirBase, ShipBase},
    editable::Editable,
    loadouts::{AARLoadout, AWACSLoadout, AntiShipLoadout, CAPLoadout, StrikeLoadout},
    mappable::MapPoint,
    oob_air::Squadron,
    targets::{
        anti_ship::AntiShipStrike, awacs::AWACS, cap::CAP, refueling::Refueling, strike::Strike,
    },
    trigger::Trigger,
    DCEInstance,
};

use dioxus::prelude::*;
use dioxus_desktop::tao::event::ElementState::Pressed;
use dioxus_desktop::tao::event::ElementState::Released;
use dioxus_desktop::tao::keyboard::KeyCode::ControlLeft;
use dioxus_desktop::tao::keyboard::KeyCode::KeyS;
use dioxus_desktop::{
    tao::{self, event::DeviceEvent},
    use_window,
    wry::http::Response,
    Config,
};
use fermi::{use_atom_ref, use_atom_root, use_init_atom_root, Atom, AtomRef};
use log::{trace, warn};
use selectable::Selectable;
use simple_logger::SimpleLogger;

use directories::ProjectDirs;

use crate::{
    helpers::select_first_helpers::*,
    rsx::{edit_form, menu_bar},
};

mod helpers;
mod rsx;
mod selectable;

static INSTANCE: AtomRef<Option<DCEInstance>> = |_| None;
static SELECTED: AtomRef<Selectable> = |_| Selectable::None;
static INSTANCE_DIRTY: Atom<bool> = |_| false;

struct AppProps {
    rx: async_channel::Receiver<MapPoint>,
}

fn main() {
    SimpleLogger::new().init().unwrap();
    // launch the dioxus app in a webview

    let (s, r) = async_channel::unbounded::<MapPoint>();

    dioxus_desktop::launch_with_props(
        app,
        AppProps { rx: r },
        Config::default()
            .with_custom_protocol("testprotocol".into(), move |req| {
                // this handle callbacks of clicked objects in leaflet
                let obj = serde_json::from_str::<MapPoint>(
                    &String::from_utf8(req.body().to_vec()).unwrap(),
                );
                if let Ok(map_point) = obj {
                    trace!("Got from WebView/Sending to channel {:?}", map_point);
                    s.send_blocking(map_point).unwrap();
                } else {
                    warn!(
                        "Failed to parse {:?} with error {:?}",
                        String::from_utf8(req.body().to_vec()).unwrap(),
                        obj.err().unwrap()
                    );
                }

                Ok(Response::new(vec![].into()))
            })
            .with_data_directory(
                ProjectDirs::from("com", "BB", "DCE Builder")
                    .unwrap()
                    .data_dir(),
            ),
    )
}

fn app(cx: Scope<AppProps>) -> Element {
    use_init_atom_root(cx);

    let w = use_window(cx);

    let s_state = use_state(cx, || false);
    let ctrl_state = use_state(cx, || false);

    // setup handler to detect CTRL-S for save
    w.create_wry_event_handler(move |event, _| {
        if let tao::event::Event::DeviceEvent { event, .. } = event {
            if let DeviceEvent::Key(rke) = event {
                if rke.physical_key == KeyS && rke.state == Pressed {
                    trace!("s key pressed")
                }
                if rke.physical_key == KeyS && rke.state == Released {
                    trace!("s key released")
                }
                if rke.physical_key == ControlLeft && rke.state == Pressed {
                    trace!("ctrl key pressed")
                }
                if rke.physical_key == ControlLeft && rke.state == Released {
                    trace!("ctrl key released")
                }
            }
        }
    });
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
                    div { 
                        class: "top-8 flex absolute inset-0 bg-slate-50 items-center justify-center",
                        div {
                            class: "",
                            "Click ", span{ class:"icon", "" }, 
                                " to start a new campaign from a DCS mission file"
                        }
                    }
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

    // states and stuff for the draggable vertical divider
    let edit_width_state = use_state(cx, || 500_i32);
    let dragging_state = use_state(cx, || false);
    let mouse_x_state = use_state(cx, || 0_f64);
    const MIN_WIDTH: i32 = 300;
    const MAX_WIDTH: i32 = 1000;

    let edit_col_width = if let Selectable::None = selected_form.to_owned() {
        "width: 0px".into()
    } else {
        format!("width: {}px", edit_width_state)
    };

    // height for the edit table at the bottom
    let edit_table_height = if let Selectable::None = selected_form.to_owned() {
        ""
    } else {
        "basis-1/4"
    };

    cx.render(rsx! {
        div {
            class: "top-8 flex absolute inset-0 bg-slate-50",
            // catch movement during edit col width drag
            onmousemove: |ev| {
                if *dragging_state.get() {
                    let current_x = ev.screen_coordinates().x;
                    let delta = current_x - mouse_x_state.get();
                    let mut cur_width = *edit_width_state.get();
                    cur_width += delta.floor() as i32;
                    cur_width = cur_width.clamp(MIN_WIDTH, MAX_WIDTH);
                    edit_width_state.set(cur_width);
                    mouse_x_state.set(current_x);
                }
            },
            // catch end of edit col width event
            onmouseup: |_| { dragging_state.set(false) },
            // selector col
            div { class: "basis-12 shrink-0 min-h-0 bg-sky-500 flex flex-col items-center",
                popout_menu {
                    onclick: |_| select_first_fixed_airbase(cx),
                    base_icon_url: "images/airfield_fixed.svg",
                    icon_button {
                        path: "images/airfield_fixed.svg".into(),
                        on_click: |_| select_first_fixed_airbase(cx)
                    }
                    icon_button {
                        path: "images/airfield_ship.svg".into(),
                        on_click: |_| select_first_ship_airbase(cx)
                    }
                    icon_button {
                        path: "images/airfield_airstart.svg".into(),
                        on_click: |_| select_first_airstart_airbase(cx)
                    }
                }
                popout_menu { onclick: |_| select_first_cap_target(cx), base_icon_url: "images/target_none.svg",
                    icon_button {
                        path: "images/target_strike.svg".into(),
                        on_click: |_| select_first_strike_target(cx)
                    }
                    icon_button {
                        path: "images/target_ship.svg".into(),
                        on_click: |_| select_first_ship_target(cx)
                    }
                    icon_button {
                        path: "images/target_cap.svg".into(),
                        on_click: |_| select_first_cap_target(cx)
                    }
                    icon_button {
                        path: "images/target_aar.svg".into(),
                        on_click: |_| select_first_aar_target(cx)
                    }
                    icon_button {
                        path: "images/target_awacs.svg".into(),
                        on_click: |_| select_first_awacs_target(cx)
                    }
                }
                popout_menu { onclick: |_| select_first_cap_loadout(cx), base_icon_url: "images/loadout_cap.svg",
                    icon_button {
                        path: "images/loadout_cap.svg".into(),
                        on_click: |_| select_first_cap_loadout(cx)
                    }
                    icon_button {
                        path: "images/loadout_strike.svg".into(),
                        on_click: |_| select_first_strike_loadout(cx)
                    }
                    icon_button {
                        path: "images/loadout_antiship.svg".into(),
                        on_click: |_| select_first_antiship_loadout(cx)
                    }
                    icon_button {
                        path: "images/loadout_awacs.svg".into(),
                        on_click: |_| select_first_awacs_loadout(cx)
                    }
                    icon_button {
                        path: "images/loadout_aar.svg".into(),
                        on_click: |_| select_first_aar_loadout(cx)
                    }
                }
                icon_button { path: "images/plane.svg".into(), on_click: |_| select_first_squadron(cx) }
                icon_button {
                    path: "images/settings_grey.png".into(),
                    on_click: |_| select_campaign_settings(cx)
                }
                icon_button {
                    path: "images/settings_grey.png".into(),
                    on_click: |_| select_first_trigger(cx)
                }
            }
            // edit col
            div { class: "shrink-0 min-h-0 bg-sky-100", style: "{edit_col_width}",
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
                    Selectable::TargetAWACS(_) => rsx!{
                        edit_form::<AWACS> { headers: AWACS::get_header(), title: "Edit AWACS".into(), item: selected_form.clone()}
                    },
                    Selectable::TargetAAR(_) => rsx!{
                        edit_form::<Refueling> { headers: Refueling::get_header(), title: "Edit AAR".into(), item: selected_form.clone()}
                    },
                    Selectable::TargetAntiShip(_) => rsx!{
                        edit_form::<AntiShipStrike> { headers: AntiShipStrike::get_header(), title: "Edit Anti-ship Strike".into(), item: selected_form.clone()}
                    },
                    Selectable::FixedAirBase(_) => rsx!{
                        edit_form::<FixedAirBase> { headers: FixedAirBase::get_header(), title: "Edit Airbase".into(), item: selected_form.clone()}
                    },
                    Selectable::ShipAirBase(_) => rsx!{
                        edit_form::<ShipBase> { headers: ShipBase::get_header(), title: "Edit Ship Base".into(), item: selected_form.clone()}
                    },
                    Selectable::AirstartBase(_) => rsx!{
                        edit_form::<AirStartBase> { headers: AirStartBase::get_header(), title: "Edit Airstart".into(), item: selected_form.clone()}
                    },
                    Selectable::CampaignSettings(_) => rsx!{
                        edit_form::<Header> { headers: Header::get_header(), title: "Campaign Settings".into(), item: selected_form.clone()}
                    },
                    Selectable::LoadoutCAP(_) => rsx!{
                        edit_form::<CAPLoadout> { headers: CAPLoadout::get_header(), title: "Edit CAP Loadout".into(), item: selected_form.clone()}
                    },
                    Selectable::LoadoutStrike(_) => rsx!{
                        edit_form::<StrikeLoadout> { headers: StrikeLoadout::get_header(), title: "Edit Strike Loadout".into(), item: selected_form.clone()}
                    },
                    Selectable::LoadoutAntiship(_) => rsx!{
                        edit_form::<AntiShipLoadout> { headers: AntiShipLoadout::get_header(), title: "Edit Anti-ship Loadout".into(), item: selected_form.clone()}
                    },
                    Selectable::LoadoutAAR(_) => rsx!{
                        edit_form::<AARLoadout> { headers: AARLoadout::get_header(), title: "Edit Refueling Loadout".into(), item: selected_form.clone()}
                    },
                    Selectable::LoadoutAWACS(_) => rsx!{
                        edit_form::<AWACSLoadout> { headers: AWACSLoadout::get_header(), title: "Edit AWACS Loadout".into(), item: selected_form.clone()}
                    },
                    Selectable::Trigger(_) => rsx!{
                        edit_form::<Trigger> { headers: Trigger::get_header(), title: "Edit Trigger".into(), item: selected_form.clone()}
                    },
                    _ => rsx!{{}}
                }
            }
            // divider
            div {
                class: "shrink-0 w-1 bg-sky-500 cursor-col-resize",
                onmousedown: |ev| {
                    mouse_x_state.set(ev.screen_coordinates().x);
                    dragging_state.set(true);
                }
            }

            // map and table col
            div { class: "flex-grow flex flex-col",
                div { class: "flex-grow min-h-0 bg-slate-50 flex flex-col", rsx::map {} }
                div { class: "{edit_table_height} grow-0 overflow-y-auto",
                    match *selected_table {
                        Selectable::Squadron(_) => rsx!{
                            rsx::table { data: instance.oob_air.red.iter().chain(instance.oob_air.blue.iter()).cloned().collect::<Vec<Squadron>>() }
                        },
                        Selectable::TargetStrike(_) => rsx! {
                            rsx::table { data: instance.target_list.strike.to_vec() }
                        },
                        Selectable::TargetCAP(_) => rsx! {
                            rsx::table {  data: instance.target_list.cap.to_vec() }
                        },
                        Selectable::TargetAntiShip(_) => rsx! {
                            rsx::table { data: instance.target_list.antiship.to_vec() }
                        },
                        Selectable::TargetAWACS(_) => rsx! {
                            rsx::table { data: instance.target_list.awacs.to_vec() }
                        },
                        Selectable::TargetAAR(_) => rsx! {
                            rsx::table { data: instance.target_list.refuel.to_vec() }
                        },
                        Selectable::FixedAirBase(_) => rsx! {
                            rsx::table { data: instance.airbases.fixed.to_vec() }
                        },
                        Selectable::ShipAirBase(_) => rsx! {
                            rsx::table { data: instance.airbases.ship.to_vec() }
                        },
                        Selectable::AirstartBase(_) => rsx! {
                            rsx::table { data: instance.airbases.air_start.to_vec() }
                        },
                        Selectable::LoadoutCAP(_) => rsx! {
                            rsx::table { data: instance.loadouts.cap.to_vec() }
                        },
                        Selectable::LoadoutStrike(_) => rsx! {
                            rsx::table { data: instance.loadouts.strike.to_vec() }
                        },
                        Selectable::LoadoutAntiship(_) => rsx! {
                            rsx::table { data: instance.loadouts.antiship.to_vec() }
                        },
                        Selectable::LoadoutAAR(_) => rsx! {
                            rsx::table { data: instance.loadouts.aar.to_vec() }
                        },
                        Selectable::LoadoutAWACS(_) => rsx! {
                            rsx::table { data: instance.loadouts.awacs.to_vec() }
                        },
                        Selectable::Trigger(_) => rsx! {
                            rsx::table { data: instance.triggers.to_vec() }
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
            class: "mt-1 mb-1 p-1 rounded opacity-70 hover:opacity-100 hover:bg-sky-600",
            src: "{cx.props.path}",
            width: 40,
            height: 40,
            onclick: |e| cx.props.on_click.call(e)
        }
    })
}

#[derive(Props)]
struct PopoutMenuProps<'a> {
    onclick: EventHandler<'a, MouseEvent>,
    base_icon_url: &'a str,
    children: Element<'a>,
}

fn popout_menu<'a>(cx: Scope<'a, PopoutMenuProps<'a>>) -> Element<'a> {
    cx.render(rsx! {
        div { class: "dropdown relative inline-block",
            div { icon_button { path: cx.props.base_icon_url.into(), on_click: |e| cx.props.onclick.call(e) } }
            div { class: "dropdown-content rounded-r-lg flex flex-col items-end pr-1 hidden absolute bg-sky-500 w-12 left-10 top-0",
                &cx.props.children
            }
        }
    })
}
