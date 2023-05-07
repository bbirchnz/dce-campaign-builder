use std::collections::HashMap;

use serde::{Deserialize, Serialize};


use crate::{
    projections::{convert_dcs_lat_lon, TranverseMercator},
    DCEInstance,
};

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct MapPoint {
    lat: f64,
    lon: f64,
    name: String,
    side: String,
    class: String,
    #[serde(flatten)]
    extras: HashMap<String, f64>,
}

impl MapPoint {
    pub fn new_from_dcs(
        x: f64,
        y: f64,
        name: String,
        side: String,
        class: String,
        map: &TranverseMercator,
    ) -> MapPoint {
        let (lon, lat) = convert_dcs_lat_lon(x, y, map);
        MapPoint {
            lat,
            lon,
            name,
            side,
            class,
            extras: HashMap::default(),
        }
    }

    pub fn add_extras(mut self, extras: HashMap<String, f64>) -> Self {
        self.extras.extend(extras.into_iter());
        self
    }
}

pub trait Mappables {
    fn to_mappables(&self, instance: &DCEInstance) -> Vec<MapPoint>;
}
