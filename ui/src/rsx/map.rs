use std::time::Duration;

use dce_lib::{mappable::Mappables, projections::proj_from_map};
use dioxus::prelude::*;
use dioxus_desktop::{use_window, DesktopContext};
use fermi::use_atom_ref;
use log::trace;
use rand::Rng;

use crate::INSTANCE;

pub fn map(cx: Scope) -> Element {
    let div_id = use_state(cx, || random_id("map_"));
    let atom = use_atom_ref(cx, INSTANCE).read();

    let instance = atom.as_ref().unwrap();
    let proj = proj_from_map(&instance.projection).unwrap();
    let mut map_points = instance.airbases.to_mappables(instance, &proj);
    let targets = instance.target_list.to_mappables(instance, &proj);
    let squadrons = instance.oob_air.to_mappables(instance, &proj);
    map_points.extend(targets);
    map_points.extend(squadrons);

    let code = format!(
        "data_{} = {}; draw_map('{}', data_{})",
        &div_id,
        serde_json::to_string(&map_points).unwrap(),
        &div_id,
        &div_id
    );

    let w = use_window(cx).clone();
    // draw with slight delay so its done after the canvas is ready
    use_effect(cx, (div_id, &code.to_owned()), move |_| {
        trace!("Triggering Drawing Map");
        delayed_js(w, code, 10)
    });

    cx.render(rsx! { div { id: "{div_id}", class: "flex-grow flex-shrink min-h-0 m-2 rounded" } })
}

async fn delayed_js(dcx: DesktopContext, code: String, duration_ms: u64) {
    tokio::time::sleep(Duration::from_millis(duration_ms)).await;
    trace!("Drawing Map");
    let code = code.to_owned();
    dcx.eval(&code).await.unwrap();
}

pub fn random_id(base: &str) -> String {
    let mut rng = rand::thread_rng();
    base.to_owned() + &rng.gen::<u16>().to_string()
}
