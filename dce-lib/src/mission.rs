use std::{fs::File, collections::HashMap};

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
    pub uncontrollable: bool,
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

#[derive(Deserialize, Serialize, Debug, PartialEq, Clone)]
pub struct Payload {
    pub pylons: HashMap<u32, Pylon>,
    pub fuel: f64,
    pub flare: f64,
    pub chaff: f64,
    pub gun: f64
}

#[derive(Deserialize, Serialize, Debug, PartialEq, Clone)]
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
    pub fn from_miz(miz_filename: String) -> Result<Mission, anyhow::Error> {
        let zipfile = File::open(miz_filename)?;
        let mut archive = ZipArchive::new(zipfile)?;

        let mut mission: String = Default::default();

        archive.by_name("mission")?.read_to_string(&mut mission)?;

        Mission::from_lua_str(&mission, "mission".into())
    }

    pub fn get_vehicle_groups(&self) -> Vec<&VehicleGroup> {
        let result = self
            .coalition
            .blue
            .countries
            .iter()
            .chain(self.coalition.red.countries.iter())
            .filter_map(|c| c.vehicle.as_ref())
            .map(|i| i.groups.as_slice())
            .map(|i| i.as_ref())
            .flat_map(|f| f)
            .collect::<Vec<_>>();

        return result;
    }

    pub fn get_zone_by_name(&self, name: &String) -> Result<&TriggerZone, anyhow::Error> {
        self.triggers
            .zones
            .iter()
            .filter(|z| &z.name == name)
            .next()
            .ok_or(anyhow!("Can't find a refpoint/zone with name {}", name))
    }
}

#[cfg(test)]
mod tests {
    use crate::serde_utils::LuaFileBased;

    use super::Mission;

    #[test]
    fn load_example() {
        let result = Mission::from_lua_file("C:\\Users\\Ben\\Saved Games\\DCS.openbeta\\Mods\\tech\\DCE\\Missions\\Campaigns\\War over Tchad 1987-Blue-Mirage-F1EE-3-30 Lorraine\\Init\\base_mission\\mission".into(), "mission".into());

        result.unwrap();
    }

    #[test]
    fn save_example() {
        let loaded = Mission::from_lua_file("C:\\Users\\Ben\\Saved Games\\DCS.openbeta\\Mods\\tech\\DCE\\Missions\\Campaigns\\War over Tchad 1987-Blue-Mirage-F1EE-3-30 Lorraine\\Init\\base_mission\\mission".into(), "mission".into()).unwrap();
        loaded
            .to_lua_file("mission".into(), "mission".into())
            .unwrap();
    }

    #[test]
    fn load_from_miz() {
        let loaded = Mission::from_miz("C:\\Users\\Ben\\Saved Games\\DCS.openbeta\\Mods\\tech\\DCE\\Missions\\Campaigns\\War over Tchad 1987-Blue-Mirage-F1EE-3-30 Lorraine\\Init\\base_mission.miz".into()).unwrap();
        loaded
            .to_lua_file("mission2".into(), "mission".into())
            .unwrap();
    }

    #[test]
    
    fn save_sa_example() {
        let loaded = Mission::from_miz("C:\\Users\\Ben\\Saved Games\\DCS.openbeta\\Mods\\tech\\DCE\\Missions\\Campaigns\\Falklands v1\\Init\\base_mission.miz".into()).unwrap();
        loaded
            .to_lua_file("mission_sa".into(), "mission".into())
            .unwrap();
    }
}
