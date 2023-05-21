use std::collections::HashMap;

use proj::Proj;
use serde::{Deserialize, Serialize};

use crate::{projections::convert_dcs_lat_lon, DCEInstance};

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct MapPoint {
    pub lat: f64,
    pub lon: f64,
    pub name: String,
    pub side: String,
    pub class: String,
    #[serde(flatten)]
    pub extras: HashMap<String, f64>,
}

impl MapPoint {
    pub fn new_from_dcs(
        x: f64,
        y: f64,
        name: String,
        side: String,
        class: String,
        map: &Proj,
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
    fn to_mappables(&self, instance: &DCEInstance, proj: &Proj) -> Vec<MapPoint>;
}
