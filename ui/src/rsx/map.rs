use std::time::Duration;

use dce_lib::{
    db_airbases::{AirBase, DBAirbases},
    projections::{convert_dcs_lat_lon, PG},
};
use dioxus::prelude::*;
use dioxus_desktop::{use_window, DesktopContext};
use log::info;
use rand::Rng;
use serde::{Deserialize, Serialize};
use serde_json;

#[derive(PartialEq, Props)]
pub struct MapProps {
    db_airbases: DBAirbases,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct MapPoint {
    x: f64,
    y: f64,
    name: String,
}

pub fn map(cx: Scope<MapProps>) -> Element {
    let div_id = use_state(cx, || random_id("map_"));

    let map_points = cx
        .props
        .db_airbases
        .0
        .iter()
        .filter(|(_, v)| matches!(v, AirBase::Fixed(_)))
        .map(|(k, v)| match v {
            AirBase::Fixed(ab) => Some((k.to_owned(), convert_dcs_lat_lon(ab.x, ab.y, &PG))),
            _ => None,
        })
        .map(|item| {
            let (name, (lon, lat)) = item.unwrap();
            MapPoint {
                x: lon,
                y: lat,
                name: name,
            }
        })
        .collect::<Vec<_>>();

    let code = format!(
        "data_{} = {}; drawmap('{}', data_{})",
        &div_id,
        serde_json::to_string(&map_points).unwrap(),
        &div_id,
        &div_id
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
    dcx.eval(&code).await.unwrap();
}

pub fn random_id(base: &str) -> String {
    let mut rng = rand::thread_rng();
    base.to_owned() + &rng.gen::<u16>().to_string()
}
