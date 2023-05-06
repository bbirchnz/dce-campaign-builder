use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::serde_utils::LuaFileBased;

#[derive(Deserialize, Serialize, Debug, PartialEq)]
pub struct Mission {
    pub theatre: String,
    pub coalition: HashMap<String, Coalition>,
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
}
