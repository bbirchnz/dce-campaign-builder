#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![cfg_attr(debug_assertions, windows_subsystem = "console")]
use std::{borrow::Cow, cell::RefCell, sync::mpsc};

use dce_lib::{
    bin_data::BinItem,
    campaign_header::HeaderInternal,
    conf_mod::ConfMod,
    db_airbases::{AirStartBase, FarpBase, FixedAirBase, ShipBase},
    editable::Editable,
    loadouts::{
        AARLoadout, AWACSLoadout, AntiShipLoadout, CAPLoadout, EscortLoadout, InterceptLoadout,
        SEADLoadout, StrikeLoadout, TransportLoadout,
    },
    mappable::MapPoint,
    oob_air::Squadron,
    targets::{
        anti_ship::AntiShipStrike, awacs::AWACS, cap::CAP, intercept::Intercept,
        refueling::Refueling, strike::Strike,
    },
    trigger::Trigger,
    DCEInstance,
};

use dioxus::prelude::*;
use dioxus_desktop::tao::event::ElementState::Released;
use dioxus_desktop::tao::keyboard::KeyCode::ControlLeft;
use dioxus_desktop::tao::keyboard::KeyCode::KeyS;
use dioxus_desktop::{tao::event::ElementState::Pressed, wry::http::StatusCode};
use dioxus_desktop::{
    tao::{self, event::DeviceEvent},
    use_window,
    wry::http::Response,
    Config,
};
use fermi::{use_atom_ref, use_atom_root, use_init_atom_root, Atom, AtomRef};
use log::{trace, warn, LevelFilter};
use log4rs::{
    append::{
        console::{ConsoleAppender, Target},
        file::FileAppender,
    },
    config::{Appender, Root},
};
use selectable::Selectable;

use directories::ProjectDirs;

use crate::{
    helpers::select_first_helpers::*,
    rsx::{edit_form, image_edit, image_table, menu_bar},
};

mod helpers;
mod rsx;
mod selectable;

static INSTANCE: AtomRef<Option<DCEInstance>> = |_| None;
static SELECTED: AtomRef<Selectable> = |_| Selectable::None;
static INSTANCE_DIRTY: Atom<bool> = |_| false;
static IMAGE_LIST_TX: AtomRef<Option<mpsc::Sender<Vec<BinItem>>>> = |_| None;

struct AppProps {
    rx_mappoint: async_channel::Receiver<MapPoint>,
    tx_vec_binitem: mpsc::Sender<Vec<BinItem>>,
}

fn main() {
    // get project data directory:
    let project_dir = ProjectDirs::from("com", "BB", "DCE Builder").unwrap();
    let data_dir = project_dir.data_dir();

    configure_logging(data_dir);

    // launch the dioxus app in a webview
    let image_vec: RefCell<Option<Vec<BinItem>>> = None.into();

    let (tx_mappoint, rx_mappoint) = async_channel::unbounded::<MapPoint>();
    let (tx_vec_binitem, rx_vec_binitem) = mpsc::channel::<Vec<BinItem>>();

    dioxus_desktop::launch_with_props(
        app,
        AppProps {
            rx_mappoint,
            tx_vec_binitem,
        },
        Config::default()
            .with_custom_protocol("testprotocol".into(), move |req| {
                // this handle callbacks of clicked objects in leaflet
                let obj = serde_json::from_str::<MapPoint>(
                    &String::from_utf8(req.body().to_vec()).unwrap(),
                );
                if let Ok(map_point) = obj {
                    trace!("Got from WebView/Sending to channel {:?}", map_point);
                    tx_mappoint.send_blocking(map_point).unwrap();
                } else {
                    warn!(
                        "Failed to parse {:?} with error {:?}",
                        String::from_utf8(req.body().to_vec()).unwrap(),
                        obj.err().unwrap()
                    );
                }

                Ok(Response::new(vec![].into()))
            })
            .with_custom_protocol("imagesprotocol".into(), move |req| {
                // make sure we've got the latest vec:
                while let Ok(data) = rx_vec_binitem.try_recv() {
                    image_vec.borrow_mut().replace(data);
                }

                // remove leading '/' from path
                let requested_image = req.uri().path().strip_prefix('/').unwrap();

                // remove any url encoding (spaces etc)
                let decoded = urlencoding::decode(requested_image).expect("UTF-8");

                // now see if we've got the image requested:
                if let Some(v) = image_vec.borrow().as_ref() {
                    if let Some(image) = v.iter().find(|bd| bd.name == decoded) {
                        let response = Response::builder()
                            .header("Content-Type", "image/png")
                            .header("Content-Length", image.data.len().to_string())
                            .status(StatusCode::OK)
                            .body(Cow::Owned(image.data.to_vec()));

                        return response.map_err(|e| e.into());
                    }
                }

                return Response::builder()
                    .status(StatusCode::NOT_FOUND)
                    .body(vec![].into())
                    .map_err(|e| e.into());
            })
            .with_data_directory(data_dir),
    )
}

fn configure_logging(data_dir: &std::path::Path) {
    // configure logging
    let file_path = data_dir
        .as_os_str()
        .to_str()
        .expect("valid path string")
        .to_owned()
        + "/dce_builder.log";

    // Build a stderr logger.
    let stderr = ConsoleAppender::builder().target(Target::Stderr).build();

    // Logging to log file.
    let logfile = FileAppender::builder().build(file_path).unwrap();

    // Log Trace level output to file where trace is the default level
    // and the programmatically specified level to stderr.
    let config = log4rs::Config::builder()
        .appender(Appender::builder().build("logfile", Box::new(logfile)))
        .appender(Appender::builder().build("stderr", Box::new(stderr)))
        .build(
            Root::builder()
                .appender("logfile")
                .appender("stderr")
                .build(LevelFilter::Trace),
        )
        .unwrap();

    log4rs::init_config(config).expect("Logger must initialise correctly");
}

fn app(cx: Scope<AppProps>) -> Element {
    use_init_atom_root(cx);

    let w = use_window(cx);

    // WIP - ctrl-s for save
    // can catch the event but probably need to setup an async channel to actually get it to dioxus
    // context
    let _s_state = use_state(cx, || false);
    let _ctrl_state = use_state(cx, || false);

    // setup handler to detect CTRL-S for save
    w.create_wry_event_handler(move |event, _| {
        if let tao::event::Event::DeviceEvent {
            event: DeviceEvent::Key(rke),
            ..
        } = event
        {
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
    });
    w.set_title("DCE");
    w.set_decorations(false);

    // state to allow things to run once only. I'm sure theres a hook for this..
    let initialised_state = use_state(cx, || false);

    let atom_instance = use_atom_ref(cx, INSTANCE);
    let atom_selected = use_atom_ref(cx, SELECTED);
    let atom_image_list_tx = use_atom_ref(cx, IMAGE_LIST_TX);

    if !initialised_state {
        let mut write_atom_image_list_tx = atom_image_list_tx.write();
        write_atom_image_list_tx.replace(cx.props.tx_vec_binitem.to_owned());
        initialised_state.set(true);
    }

    let instance_loaded = atom_instance.read().is_some();
    let atoms = use_atom_root(cx);

    use_coroutine(cx, move |_: UnboundedReceiver<i32>| {
        let atom_selected = atom_selected.to_owned();
        let atoms = atoms.to_owned();
        let rx = cx.props.rx_mappoint.to_owned();

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
            class: "top-8 bottom-6 flex absolute inset-0 bg-slate-50",
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
                        on_click: |_| select_first_fixed_airbase(cx),
                        tooltip: "Fixed airbases"
                    }
                    icon_button {
                        path: "images/airfield_ship.svg".into(),
                        on_click: |_| select_first_ship_airbase(cx),
                        tooltip: "Aircraft carriers"
                    }
                    icon_button {
                        path: "images/airfield_airstart.svg".into(),
                        on_click: |_| select_first_airstart_airbase(cx),
                        tooltip: "Airstart/virtual airbases"
                    }
                    icon_button {
                        path: "images/airfield_farp.svg".into(),
                        on_click: |_| select_first_farp_airbase(cx),
                        tooltip: "FARPs"
                    }
                }
                popout_menu { onclick: |_| select_first_cap_target(cx), base_icon_url: "images/target_none.svg",
                    icon_button {
                        path: "images/target_strike.svg".into(),
                        on_click: |_| select_first_strike_target(cx),
                        tooltip: "Strike targets"
                    }
                    icon_button {
                        path: "images/target_ship.svg".into(),
                        on_click: |_| select_first_ship_target(cx),
                        tooltip: "Anti-ship strike targets"
                    }
                    icon_button {
                        path: "images/target_cap.svg".into(),
                        on_click: |_| select_first_cap_target(cx),
                        tooltip: "Combat air patrol zones"
                    }
                    icon_button {
                        path: "images/target_intercept.svg".into(),
                        on_click: |_| select_first_intercept_target(cx),
                        tooltip: "Ground Controlled Intercept tasks"
                    }
                    icon_button {
                        path: "images/target_aar.svg".into(),
                        on_click: |_| select_first_aar_target(cx),
                        tooltip: "Air to air refueling zones"
                    }
                    icon_button {
                        path: "images/target_awacs.svg".into(),
                        on_click: |_| select_first_awacs_target(cx),
                        tooltip: "AWACS patrol zones"
                    }
                }
                popout_menu { onclick: |_| select_first_cap_loadout(cx), base_icon_url: "images/loadout_cap.svg",
                    icon_button {
                        path: "images/loadout_cap.svg".into(),
                        on_click: |_| select_first_cap_loadout(cx),
                        tooltip: "Combat air patrol loadouts and flight profiles"
                    }
                    icon_button {
                        path: "images/loadout_strike.svg".into(),
                        on_click: |_| select_first_strike_loadout(cx),
                        tooltip: "Strike loadouts and flight profiles"
                    }
                    icon_button {
                        path: "images/loadout_antiship.svg".into(),
                        on_click: |_| select_first_antiship_loadout(cx),
                        tooltip: "Anti-ship strike loadouts and flight profiles"
                    }
                    icon_button {
                        path: "images/loadout_escort.svg".into(),
                        on_click: |_| select_first_escort_loadout(cx),
                        tooltip: "Escort loadouts and flight profiles"
                    }
                    icon_button {
                        path: "images/loadout_sead.svg".into(),
                        on_click: |_| select_first_sead_loadout(cx),
                        tooltip: "SEAD escort loadouts and flight profiles"
                    }
                    icon_button {
                        path: "images/loadout_intercept.svg".into(),
                        on_click: |_| select_first_intercept_loadout(cx),
                        tooltip: "Ground controlled intercept loadouts and flight profiles"
                    }
                    icon_button {
                        path: "images/loadout_awacs.svg".into(),
                        on_click: |_| select_first_awacs_loadout(cx),
                        tooltip: "AWACS loadouts and flight profiles"
                    }
                    icon_button {
                        path: "images/loadout_aar.svg".into(),
                        on_click: |_| select_first_aar_loadout(cx),
                        tooltip: "Air to air refueling loadouts and flight profiles"
                    }
                    icon_button {
                        path: "images/loadout_transport.svg".into(),
                        on_click: |_| select_first_transport_loadout(cx),
                        tooltip: "Transport loadouts and flight profiles"
                    }
                }
                icon_button {
                    path: "images/plane.svg".into(),
                    on_click: |_| select_first_squadron(cx),
                    tooltip: "Squadrons"
                }
                icon_button {
                    path: "images/settings_grey.png".into(),
                    on_click: |_| select_campaign_settings(cx),
                    tooltip: "Campaign settings"
                }
                icon_button {
                    path: "images/settings_grey.png".into(),
                    on_click: |_| select_configuration_mod(cx),
                    tooltip: "Configuration"
                }
                icon_button {
                    path: "images/triggers.svg".into(),
                    on_click: |_| select_first_trigger(cx),
                    tooltip: "Campaign actions and triggers"
                }
                icon_button {
                    path: "images/images.svg".into(),
                    on_click: |_| select_first_image(cx),
                    tooltip: "Campaign and target images"
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
                    Selectable::TargetIntercept(Some(_)) => rsx!{
                        edit_form::<Intercept> { headers: Intercept::get_header(), title: "Edit Intercept".into(), item: selected_form.clone()}
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
                    Selectable::FARPBase(_) => rsx!{
                        edit_form::<FarpBase> { headers: FarpBase::get_header(), title: "Edit FARP".into(), item: selected_form.clone()}
                    },
                    Selectable::CampaignSettings(_) => rsx!{
                        edit_form::<HeaderInternal> { headers: HeaderInternal::get_header(), title: "Campaign Settings".into(), item: selected_form.clone()}
                    },
                    Selectable::ConfigurationMod(_) => rsx!{
                        edit_form::<ConfMod> { headers: ConfMod::get_header(), title: "Configuration".into(), item: selected_form.clone()}
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
                    Selectable::LoadoutEscort(_) => rsx!{
                        edit_form::<EscortLoadout> { headers: EscortLoadout::get_header(), title: "Edit Escort Loadout".into(), item: selected_form.clone()}
                    },
                    Selectable::LoadoutIntercept(_) => rsx!{
                        edit_form::<InterceptLoadout> { headers: InterceptLoadout::get_header(), title: "Edit Intercept Loadout".into(), item: selected_form.clone()}
                    },
                    Selectable::LoadoutSEAD(_) => rsx!{
                        edit_form::<SEADLoadout> { headers: SEADLoadout::get_header(), title: "Edit SEAD Loadout".into(), item: selected_form.clone()}
                    },
                    Selectable::LoadoutTransport(_) => rsx!{
                        edit_form::<TransportLoadout> { headers: TransportLoadout::get_header(), title: "Edit Transport Loadout".into(), item: selected_form.clone()}
                    },
                    Selectable::Trigger(_) => rsx!{
                        edit_form::<Trigger> { headers: Trigger::get_header(), title: "Edit Trigger".into(), item: selected_form.clone()}
                    },
                    Selectable::Image(_) => rsx! {
                        image_edit {item: selected_form.clone()}
                    },
                    Selectable::None | Selectable::TargetIntercept(None) => rsx!{{}}
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
                            rsx::table { title: "Squadrons", data: instance.oob_air.red.iter().chain(instance.oob_air.blue.iter()).cloned().collect::<Vec<Squadron>>() }
                        },
                        Selectable::TargetStrike(_) => rsx! {
                            rsx::table { title: "Strike Targets", data: instance.target_list.strike.to_vec() }
                        },
                        Selectable::TargetCAP(_) => rsx! {
                            rsx::table {  title: "Combat Air Patrols", data: instance.target_list.cap.to_vec() }
                        },
                        Selectable::TargetAntiShip(_) => rsx! {
                            rsx::table { title: "Anti-ship Strike Targets", data: instance.target_list.antiship.to_vec() }
                        },
                        Selectable::TargetAWACS(_) => rsx! {
                            rsx::table { title: "AWACS Patrols", data: instance.target_list.awacs.to_vec() }
                        },
                        Selectable::TargetAAR(_) => rsx! {
                            rsx::table { title: "Refueling Zones", data: instance.target_list.refuel.to_vec() }
                        },
                        Selectable::TargetIntercept(_) => rsx! {
                            rsx::table { title: "Intercept Zones", data: instance.target_list.intercept.to_vec() }
                        },
                        Selectable::FixedAirBase(_) => rsx! {
                            rsx::table { title: "Airbases", data: instance.airbases.fixed.to_vec() }
                        },
                        Selectable::ShipAirBase(_) => rsx! {
                            rsx::table { title: "Aircraft Carriers", data: instance.airbases.ship.to_vec() }
                        },
                        Selectable::AirstartBase(_) => rsx! {
                            rsx::table { title: "Air-start zones", data: instance.airbases.air_start.to_vec() }
                        },
                        Selectable::FARPBase(_) => rsx! {
                            rsx::table { title: "FARPs", data: instance.airbases.farp.to_vec() }
                        },
                        Selectable::LoadoutCAP(_) => rsx! {
                            rsx::table { title: "CAP Loadout and Profiles", data: instance.loadouts.cap.to_vec() }
                        },
                        Selectable::LoadoutStrike(_) => rsx! {
                            rsx::table { title: "Strike Loadout and Profiles", data: instance.loadouts.strike.to_vec() }
                        },
                        Selectable::LoadoutAntiship(_) => rsx! {
                            rsx::table { title: "Anti-ship Strike Loadout and Profiles", data: instance.loadouts.antiship.to_vec() }
                        },
                        Selectable::LoadoutAAR(_) => rsx! {
                            rsx::table { title: "Refueling Profiles", data: instance.loadouts.aar.to_vec() }
                        },
                        Selectable::LoadoutAWACS(_) => rsx! {
                            rsx::table {title: "AWACS Profiles",  data: instance.loadouts.awacs.to_vec() }
                        },
                        Selectable::LoadoutEscort(_) => rsx! {
                            rsx::table { title: "Escort Loadout and Profiles", data: instance.loadouts.escort.to_vec() }
                        },
                        Selectable::LoadoutIntercept(_) => rsx! {
                            rsx::table { title: "Intercept Loadout and Profiles", data: instance.loadouts.intercept.to_vec() }
                        },
                        Selectable::LoadoutSEAD(_) => rsx! {
                            rsx::table { title: "SEAD Escort Loadout and Profiles", data: instance.loadouts.sead.to_vec() }
                        },
                        Selectable::LoadoutTransport(_) => rsx! {
                            rsx::table { title: "Transport Profiles", data: instance.loadouts.transport.to_vec() }
                        },
                        Selectable::Trigger(_) => rsx! {
                            rsx::table { title: "Campaign Triggers and Actions", data: instance.triggers.to_vec() }
                        },
                        Selectable::Image(_) => rsx! {
                            image_table {data: instance.bin_data.images.to_vec()}
                        },
                        Selectable::None | Selectable::CampaignSettings(_) | Selectable::ConfigurationMod(_) => rsx! {
                            {}
                        },
                        }
                }
            }
            // right edge border
            div { class: "basis-1 shrink-0 min-h-0 bg-sky-500" }
        }
        div { class: "bg-sky-500 bottom-0 h-6 absolute w-full flex", "footer" }
    })
}

#[derive(Props)]
struct IconButtonProps<'a> {
    path: String,
    on_click: EventHandler<'a, MouseEvent>,
    tooltip: Option<&'a str>,
}

fn icon_button<'a>(cx: Scope<'a, IconButtonProps<'a>>) -> Element<'a> {
    cx.render(rsx! {
        div { class: "tooltip",
            img {
                class: "mt-1 mb-1 p-1 rounded opacity-70 hover:opacity-100 hover:bg-sky-600 tooltip",
                src: "{cx.props.path}",
                width: 40,
                height: 40,
                onclick: |e| cx.props.on_click.call(e)
            }
            if cx.props.tooltip.is_some() {
                rsx! {
                    span {
                        class: "tooltiptext",
                        "{cx.props.tooltip.unwrap()}"
                    }
                }
            }
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
            div { class: "dropdown-content rounded-r-lg flex flex-col items-end pr-1 absolute bg-sky-500 w-12 left-10 top-0",
                &cx.props.children
            }
        }
    })
}
