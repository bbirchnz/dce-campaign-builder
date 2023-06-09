use std::{collections::HashMap, fs::File};

use bevy_reflect::{FromReflect, Reflect};
use serde::{Deserialize, Serialize};

use crate::serde_utils::LuaFileBased;

use std::io::prelude::*;
use zip::ZipArchive;

use anyhow::anyhow;

#[derive(Deserialize, Serialize, Debug, PartialEq)]
pub struct Mission {
    pub theatre: String,
    pub coalition: CoalitionCollection,
    pub triggers: Triggers,
}

#[derive(Deserialize, Serialize, Debug, PartialEq)]
pub struct Triggers {
    pub zones: Vec<TriggerZone>,
}

#[derive(Deserialize, Serialize, Debug, PartialEq)]
pub struct TriggerZone {
    pub radius: f64,
    #[serde(rename = "zoneId")]
    pub zone_id: u64,
    pub x: f64,
    pub y: f64,
    pub hidden: bool,
    pub name: String,
    #[serde(rename = "type")]
    pub _type: u64,
}

#[derive(Deserialize, Serialize, Debug, PartialEq)]
pub struct CoalitionCollection {
    pub blue: Coalition,
    pub red: Coalition,
    pub neutrals: Coalition,
}

#[derive(Deserialize, Serialize, Debug, PartialEq)]
pub struct Coalition {
    #[serde(rename = "country")]
    pub countries: Vec<Country>,
}

#[derive(Deserialize, Serialize, Debug, PartialEq)]
pub struct Country {
    pub name: String,
    pub id: u64,
    #[serde(rename = "static")]
    pub _static: Option<StaticGroupDummy>,
    pub vehicle: Option<VehicleGroupDummy>,
    pub ship: Option<ShipGroupDummy>,
    pub plane: Option<PlaneGroupDummy>,
    pub helicopter: Option<PlaneGroupDummy>,
}

#[derive(Deserialize, Serialize, Debug, PartialEq)]
pub struct StaticGroupDummy {
    #[serde[rename="group"]]
    pub groups: Vec<StaticGroup>,
}

#[derive(Deserialize, Serialize, Debug, PartialEq)]
pub struct StaticGroup {
    pub heading: f64,
    #[serde(rename = "groupId")]
    pub group_id: u64,
    #[serde(default)]
    pub hidden: bool,
    pub x: f64,
    pub y: f64,
    pub name: String,
    pub dead: bool,
    pub route: Route,
    pub units: Vec<StaticUnit>,
}

#[derive(Deserialize, Serialize, Debug, PartialEq)]
pub struct VehicleGroupDummy {
    #[serde[rename="group"]]
    pub groups: Vec<VehicleGroup>,
}

#[derive(Deserialize, Serialize, Debug, PartialEq)]
pub struct VehicleGroup {
    pub visible: bool,
    pub uncontrollable: bool,
    pub task: String,
    // pub route: Route,
    #[serde(rename = "groupId")]
    pub group_id: u64,
    pub hidden: bool,
    pub x: f64,
    pub y: f64,
    pub name: String,
    pub start_time: f64,
}

#[derive(Deserialize, Serialize, Debug, PartialEq)]
pub struct ShipGroupDummy {
    #[serde[rename="group"]]
    pub groups: Vec<ShipGroup>,
}

#[derive(Deserialize, Serialize, Debug, PartialEq)]
pub struct ShipGroup {
    pub visible: bool,
    pub uncontrollable: bool,
    #[serde(default)]
    #[serde(rename = "lateActivation")]
    pub late_activation: bool,
    // pub route: Route,
    #[serde(rename = "groupId")]
    pub group_id: u64,
    pub hidden: bool,
    pub x: f64,
    pub y: f64,
    pub name: String,
    pub start_time: f64,
    pub units: Vec<ShipUnit>,
}

#[derive(Deserialize, Serialize, Debug, PartialEq)]
pub struct ShipUnit {
    pub skill: String,
    #[serde(rename = "type")]
    pub _type: String,
    #[serde(rename = "unitId")]
    pub unit_id: u64,
    pub x: f64,
    pub y: f64,
    pub name: String,
    pub heading: f64,
    pub frequency: u64,
    pub modulation: u8,
}

#[derive(Deserialize, Serialize, Debug, PartialEq)]
pub struct PlaneGroupDummy {
    #[serde[rename="group"]]
    pub groups: Vec<PlaneGroup>,
}

#[derive(Deserialize, Serialize, Debug, PartialEq)]
pub struct PlaneGroup {
    #[serde(default)]
    pub uncontrollable: bool,
    #[serde(default)]
    pub uncontrolled: bool,
    pub modulation: u8,
    pub frequency: f64,
    #[serde(default)]
    #[serde(rename = "lateActivation")]
    pub late_activation: bool,
    pub task: String,
    // pub route: Route,
    #[serde(rename = "groupId")]
    pub group_id: u64,
    pub hidden: bool,
    pub x: f64,
    pub y: f64,
    pub name: String,
    pub start_time: f64,
    pub units: Vec<PlaneUnit>,
}

#[derive(Deserialize, Serialize, Debug, PartialEq)]
pub struct PlaneUnit {
    pub skill: String,
    #[serde(rename = "type")]
    pub _type: String,
    pub livery_id: String,
    #[serde(rename = "unitId")]
    pub unit_id: u64,
    pub x: f64,
    pub y: f64,
    pub name: String,
    pub heading: f64,
    pub payload: Payload,
}

#[derive(Deserialize, Serialize, Debug, PartialEq, Clone, Reflect, FromReflect)]
pub struct Payload {
    pub pylons: HashMap<u32, Pylon>,
    pub fuel: f64,
    pub flare: f64,
    pub chaff: f64,
    pub gun: f64,
}

#[derive(Deserialize, Serialize, Debug, PartialEq, Clone, Reflect, FromReflect)]
pub struct Pylon {
    #[serde(rename = "CLSID")]
    pub cls_id: String,
}

#[derive(Deserialize, Serialize, Debug, PartialEq)]
pub struct Route {
    pub points: Vec<StaticGroupPoint>,
}

#[derive(Deserialize, Serialize, Debug, PartialEq)]
pub struct StaticGroupPoint {
    pub alt: f64,
    #[serde(rename = "type")]
    pub _type: String,
    pub name: String,
    pub x: f64,
    pub y: f64,
    pub speed: f64,
    pub formation_template: String,
    pub action: String,
}

#[derive(Deserialize, Serialize, Debug, PartialEq)]
pub struct StaticUnit {
    pub category: String,
    pub shape_name: Option<String>,
    #[serde(rename = "type")]
    pub _type: String,
    #[serde(rename = "unitId")]
    pub unit_id: u64,
    pub rate: Option<u64>,
    pub x: f64,
    pub y: f64,
    pub name: String,
    pub heading: f64,
}

impl LuaFileBased<'_> for Mission {}

impl Mission {
    pub fn from_miz(miz_filename: &str) -> Result<Mission, anyhow::Error> {
        let zipfile = File::open(miz_filename)?;
        let mut archive = ZipArchive::new(zipfile)?;

        let mut mission: String = Default::default();

        archive.by_name("mission")?.read_to_string(&mut mission)?;

        Mission::from_lua_str(&mission, "mission")
    }

    pub fn get_vehicle_groups(&self) -> Vec<&VehicleGroup> {
        let result = self
            .coalition
            .blue
            .countries
            .iter()
            .chain(self.coalition.red.countries.iter())
            .filter_map(|c| c.vehicle.as_ref())
            .flat_map(|i| i.groups.as_slice())
            .collect::<Vec<_>>();

        result
    }

    pub fn get_plane_groups(&self) -> Vec<&PlaneGroup> {
        let countries = self
            .coalition
            .blue
            .countries
            .iter()
            .chain(self.coalition.red.countries.iter());

        let result = countries
            .clone()
            .filter_map(|c| c.plane.as_ref())
            .chain(countries.filter_map(|c: &Country| c.helicopter.as_ref()))
            .flat_map(|i| i.groups.as_slice())
            .collect::<Vec<_>>();

        result
    }

    pub fn get_ship_groups(&self) -> Vec<&ShipGroup> {
        let result = self
            .coalition
            .blue
            .countries
            .iter()
            .chain(self.coalition.red.countries.iter())
            .filter_map(|c| c.ship.as_ref())
            .flat_map(|i| i.groups.as_slice())
            .collect::<Vec<_>>();

        result
    }

    pub fn get_static_groups(&self) -> Vec<&StaticGroup> {
        let result = self
            .coalition
            .blue
            .countries
            .iter()
            .chain(self.coalition.red.countries.iter())
            .filter_map(|c| c._static.as_ref())
            .flat_map(|i| i.groups.as_slice())
            .collect::<Vec<_>>();

        result
    }

    pub fn get_zone_by_name(&self, name: &String) -> Result<&TriggerZone, anyhow::Error> {
        self.triggers
            .zones
            .iter()
            .find(|z| &z.name == name)
            .ok_or(anyhow!("Can't find a refpoint/zone with name {}", name))
    }
}

#[cfg(test)]
mod tests {
    use crate::serde_utils::LuaFileBased;

    use super::Mission;

    #[test]
    fn load_from_miz() {
        let loaded =
            Mission::from_miz("test_resources\\base_mission_falklands.miz".into()).unwrap();
        loaded
            .to_lua_file("mission2".into(), "mission".into())
            .unwrap();
    }
}
