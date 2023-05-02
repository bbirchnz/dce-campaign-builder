use std::time::Duration;

use dce_lib::{
    db_airbases::{AirBase, DBAirbases},
    projections::{convert_dcs_lat_lon, PG},
};
use dioxus::prelude::*;
use dioxus_desktop::{use_window, DesktopContext};
use log::info;
use rand::Rng;

#[derive(PartialEq, Props)]
pub struct MapProps {
    db_airbases: DBAirbases,
}

pub fn map(cx: Scope<MapProps>) -> Element {
    let div_id = use_state(cx, || random_id("map_"));

    let filtered = cx
        .props
        .db_airbases
        .0
        .iter()
        .clone()
        .filter(|(_, v)| matches!(v, AirBase::Fixed(_)))
        .map(|(k, v)| match v {
            AirBase::Fixed(ab) => Some((k.to_owned(), convert_dcs_lat_lon(ab.x, ab.y, &PG))),
            _ => None,
        })
        .map(|item| {
            let (name, (lon, lat)) = item.unwrap();
            format!(
                "L.marker([{}, {}]).addTo(map).bindPopup('{}');",
                lat, lon, name
            )
        })
        .collect::<Vec<String>>()
        .join("\n");

    let code = format!(
        "var map = L.map('{}').setView([51.505, -0.09], 13); L.tileLayer('https://tile.openstreetmap.org/{{z}}/{{x}}/{{y}}.png', {{
            attribution: '&copy; <a href=\"https://www.openstreetmap.org/copyright\">OpenStreetMap</a> contributors'
        }}).addTo(map);{}",
        &div_id, filtered
    );

    let w = use_window(cx).clone();
    // draw with slight delay so its done after the canvas is ready
    use_effect(cx, div_id, move |_| delayed_js(w, code, 10));

    cx.render(rsx! { div { id: "{div_id}", class: "flex-grow flex-shrink min-h-0 m-4" } })
}

async fn delayed_js(dcx: DesktopContext, code: String, duration_ms: u64) {
    tokio::time::sleep(Duration::from_millis(duration_ms)).await;
    info!("Drawing Map");
    let code = code.to_owned();
    dcx.eval(&code);
}

pub fn random_id(base: &str) -> String {
    let mut rng = rand::thread_rng();
    base.to_owned() + &rng.gen::<u16>().to_string()
}
