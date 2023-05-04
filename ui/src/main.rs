use dce_lib::{db_airbases::DBAirbases, serde_utils::LuaFileBased};
use dioxus::prelude::*;
use dioxus_desktop::{use_window, wry::http::Response, Config};
use fermi::use_init_atom_root;
use log::{info, warn};
use simple_logger::SimpleLogger;

use crate::rsx::{menu_bar, MapPoint};

mod rsx;

fn main() {
    SimpleLogger::new().init().unwrap();
    // launch the dioxus app in a webview
    dioxus_desktop::launch_cfg(
        app,
        Config::default().with_custom_protocol("testprotocol".into(), |req| {
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

    let db_airbase = DBAirbases::from_lua_file("C:\\Users\\Ben\\Saved Games\\DCS.openbeta\\Mods\\tech\\DCE\\Missions\\Campaigns\\War over Tchad 1987-Blue-Mirage-F1EE-3-30 Lorraine\\Init\\db_airbases.lua".into(), "db_airbases".into()).unwrap();

    cx.render(rsx! {
        // TODO: replace this script inclusion
        script { include_str!("./js/tailwind_3.3.1.js") }
        script { include_str!("./js/leaflet.js") }
        script { include_str!("./js/leaflet_utils.js") }
        style { include_str!("./css/base.css") }
        link {
            rel: "stylesheet",
            href: "https://unpkg.com/leaflet@1.9.3/dist/leaflet.css",
            integrity: "sha256-kLaT2GOSpHechhsozzB+flnD+zUyjE2LlfWPgU04xyI=",
            crossorigin: ""
        }
        style { include_str!("./css/leaflet.css") }
        div { class: "h-full select-none",
            menu_bar { title: "DCE" }
            div { class: "top-8 grid grid-cols-4 absolute inset-0 bg-red-100",
                div { class: "col-span-1 min-h-0 bg-sky-100", div {
                } }
                div { class: "col-span-3 min-h-0 bg-slate-50 flex flex-col", rsx::map { db_airbases: db_airbase } }
            }
        }
    })
}
