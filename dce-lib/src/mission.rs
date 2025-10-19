use nestify::nest;
use serde_aux::prelude::*;
use std::{collections::HashMap, fs::File, iter::repeat, slice::Iter};

use bevy_reflect::{FromReflect, Reflect};
use serde::{Deserialize, Serialize};

use crate::serde_utils::LuaFileBased;

use crate::serde_utils::deserialize_as_string_regardless;
use anyhow::anyhow;
use std::io::prelude::*;
use zip::ZipArchive;

#[derive(Deserialize, Serialize, Debug, PartialEq)]
pub struct Mission {
    pub theatre: String,
    pub coalition: CoalitionCollection,
    pub triggers: Triggers,
    pub date: Date,
    pub sortie: String,
    pub weather: Weather,
}

#[derive(Deserialize, Serialize, Debug, PartialEq)]
pub struct Triggers {
    #[serde(default)]
    pub zones: Vec<TriggerZone>,
}

#[derive(Deserialize, Serialize, Debug, PartialEq)]
pub struct Date {
    #[serde(rename = "Year")]
    pub year: i32,
    #[serde(rename = "Month")]
    pub month: u32,
    #[serde(rename = "Day")]
    pub day: u32,
}

#[derive(Deserialize, Serialize, Debug, PartialEq)]
pub struct Weather {
    pub wind: Wind,
    pub qnh: f64,
    pub season: Season,
}

#[derive(Deserialize, Serialize, Debug, PartialEq)]
pub struct Season {
    pub temperature: f64,
}

#[derive(Deserialize, Serialize, Debug, PartialEq)]
pub struct Wind {
    #[serde(rename = "atGround")]
    pub at_ground: WindLayer,
    #[serde(rename = "at2000")]
    pub at_2000: WindLayer,
    #[serde(rename = "at8000")]
    pub at_8000: WindLayer,
}

#[derive(Deserialize, Serialize, Debug, PartialEq)]
pub struct WindLayer {
    pub speed: f64,
    pub dir: f64,
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
    pub _type: Option<u64>,
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
    pub bullseye: Bullseye,
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
    #[serde(default)]
    pub uncontrollable: bool,
    pub task: Option<String>,
    pub route: Route,
    #[serde(rename = "groupId")]
    pub group_id: u64,
    pub hidden: bool,
    pub x: f64,
    pub y: f64,
    pub name: String,
    pub start_time: f64,
    pub units: Vec<VehicleUnit>,
}

#[derive(Deserialize, Serialize, Debug, PartialEq)]
pub struct Bullseye {
    pub x: f64,
    pub y: f64,
}

#[derive(Deserialize, Serialize, Debug, PartialEq)]
pub struct ShipGroupDummy {
    #[serde[rename="group"]]
    pub groups: Vec<ShipGroup>,
}

#[derive(Deserialize, Serialize, Debug, PartialEq)]
pub struct ShipGroup {
    pub visible: bool,
    #[serde(default)]
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
    #[serde(default)]
    pub frequency: u64,
    #[serde(default)]
    pub modulation: u8,
}

#[derive(Deserialize, Serialize, Debug, PartialEq)]
pub struct VehicleUnit {
    pub skill: String,
    #[serde(default)]
    #[serde(rename = "coldAtStart")]
    pub cold_at_start: bool,
    #[serde(rename = "type")]
    pub _type: String,
    #[serde(rename = "unitId")]
    pub unit_id: u64,
    pub x: f64,
    pub y: f64,
    pub name: String,
    pub heading: f64,
    #[serde(rename = "playerCanDrive")]
    pub player_can_drive: Option<bool>,
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
    pub route: PlaneRoute,
}

#[derive(Deserialize, Serialize, Debug, PartialEq)]
pub struct PlaneUnit {
    pub skill: String,
    #[serde(rename = "type")]
    pub _type: String,
    #[serde(deserialize_with = "deserialize_as_string_regardless")] // seen as 37 on grayflag
    #[serde(default)]
    pub livery_id: String, // this can be missing sometimes?
    #[serde(rename = "unitId")]
    pub unit_id: u64,
    pub x: f64,
    pub y: f64,
    #[serde(deserialize_with = "deserialize_as_string_regardless")] // seen as -1 on grayflag
    pub name: String,
    pub heading: f64,
    pub payload: Payload,
}

#[derive(Deserialize, Serialize, Debug, PartialEq, Clone, Reflect, FromReflect)]
pub struct Payload {
    pub pylons: HashMap<u32, Pylon>,
    #[serde(deserialize_with = "deserialize_number_from_string")]
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
    #[serde(default)]
    pub name: String,
    pub x: f64,
    pub y: f64,
    pub speed: f64,
    pub formation_template: String,
    pub action: String,
    pub task: Option<VehicleTask>,
}

nest! {
    #[derive(Deserialize, Serialize, Debug, PartialEq)]*
    #[serde(tag = "id")]
    pub enum VehicleTask {
        ComboTask {
                params: pub struct ComboTaskInner {
                    pub tasks: Vec<VehicleTask>
                }
            },
        WrappedAction {
            enabled: bool,
            number: u32,
            params: pub struct ActionWrapper {
                pub action: #[serde(tag = "id")] pub enum TaskAction {
                    ActivateRSBN {
                        params: pub struct ActivateRSBNParams {
                            pub callsign: String,
                            pub channel: u32
                        }
                    },
                    ActivateBeacon {
                        params: pub struct ActivateBeaconParams {
                            pub callsign: String,
                            pub channel: u32
                        }
                    },
                    #[serde(other)]
                    UNMAPPED
                }
            }
        },
        EWR, // TODO: handle this,
        #[serde(other)]
        UNMAPPED
    }
}

#[derive(Deserialize, Serialize, Debug, PartialEq)]
pub struct PlaneRoute {
    pub points: Vec<PlaneGroupPoint>,
}

#[derive(Deserialize, Serialize, Debug, PartialEq)]
pub struct PlaneGroupPoint {
    pub alt: f64,
    pub action: String,
    pub alt_type: String,
    // properties
    pub speed: f64,
    // task
    #[serde(rename = "type")]
    pub _type: String,
    // eta
    #[serde(default)]
    pub name: String, // this can be missing sometimes
    pub x: f64,
    pub y: f64,
    pub formation_template: String,
}

#[derive(Deserialize, Serialize, Debug, PartialEq)]
pub struct StaticUnit {
    #[serde(default)]
    pub category: String,
    pub shape_name: Option<String>,
    #[serde(rename = "type")]
    pub _type: String,
    #[serde(rename = "unitId")]
    pub unit_id: u64,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    #[serde(default = "default_i64::<30>")]
    pub rate: i64,
    pub x: f64,
    pub y: f64,
    pub name: String,
    pub heading: f64,
    pub heliport_callsign_id: Option<u32>,
    pub heliport_modulation: Option<u32>,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    #[serde(default)]
    pub heliport_frequency: f64,
}

impl LuaFileBased<'_> for Mission {}

impl Mission {
    pub fn from_miz(miz_filename: &str) -> Result<Mission, anyhow::Error> {
        let zipfile = File::open(miz_filename)?;
        let mut archive = ZipArchive::new(zipfile)?;

        let mut mission: String = Default::default();

        archive.by_name("mission")?.read_to_string(&mut mission)?;

        Mission::from_lua_str(&mission, "mission")
            .map_err(|e| anyhow::anyhow!("Failed to parse miz {} with error: {}", miz_filename, e))
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

    pub fn get_max_group_id(&self) -> u64 {
        let mut id = 0;

        self.coalition
            .blue
            .countries
            .iter()
            .chain(self.coalition.red.countries.iter())
            .chain(self.coalition.neutrals.countries.iter())
            .for_each(|c| {
                if c._static.is_some() {
                    c._static.as_ref().unwrap().groups.iter().for_each(|g| {
                        id = id.max(g.group_id);
                    });
                }
                if c.helicopter.is_some() {
                    c.helicopter.as_ref().unwrap().groups.iter().for_each(|g| {
                        id = id.max(g.group_id);
                    });
                }
                if c.plane.is_some() {
                    c.plane.as_ref().unwrap().groups.iter().for_each(|g| {
                        id = id.max(g.group_id);
                    });
                }
                if c.vehicle.is_some() {
                    c.vehicle.as_ref().unwrap().groups.iter().for_each(|g| {
                        id = id.max(g.group_id);
                    });
                }
                if c.ship.is_some() {
                    c.ship.as_ref().unwrap().groups.iter().for_each(|g| {
                        id = id.max(g.group_id);
                    });
                }
            });

        id
    }

    pub fn get_max_unit_id(&self) -> u64 {
        let mut id = 0;

        self.coalition
            .blue
            .countries
            .iter()
            .chain(self.coalition.red.countries.iter())
            .chain(self.coalition.neutrals.countries.iter())
            .for_each(|c| {
                if c._static.is_some() {
                    c._static
                        .as_ref()
                        .unwrap()
                        .groups
                        .iter()
                        .flat_map(|g| g.units.as_slice())
                        .for_each(|u| {
                            id = id.max(u.unit_id);
                        });
                }
                if c.helicopter.is_some() {
                    c.helicopter
                        .as_ref()
                        .unwrap()
                        .groups
                        .iter()
                        .flat_map(|g| g.units.as_slice())
                        .for_each(|u| {
                            id = id.max(u.unit_id);
                        });
                }
                if c.plane.is_some() {
                    c.plane
                        .as_ref()
                        .unwrap()
                        .groups
                        .iter()
                        .flat_map(|g| g.units.as_slice())
                        .for_each(|u| {
                            id = id.max(u.unit_id);
                        });
                }
                if c.ship.is_some() {
                    c.ship
                        .as_ref()
                        .unwrap()
                        .groups
                        .iter()
                        .flat_map(|g| g.units.as_slice())
                        .for_each(|u| {
                            id = id.max(u.unit_id);
                        });
                }
                if c.vehicle.is_some() {
                    c.vehicle
                        .as_ref()
                        .unwrap()
                        .groups
                        .iter()
                        .flat_map(|g| g.units.as_slice())
                        .for_each(|u| {
                            id = id.max(u.unit_id);
                        });
                }
            });

        id
    }

    /// Returns an immutable iter of countries zipped with side
    pub fn country_iter(
        &self,
    ) -> std::iter::Chain<
        std::iter::Zip<Iter<Country>, std::iter::Repeat<&str>>,
        std::iter::Zip<Iter<Country>, std::iter::Repeat<&str>>,
    > {
        return self
            .coalition
            .blue
            .countries
            .iter()
            .zip(repeat("blue"))
            .chain(self.coalition.red.countries.iter().zip(repeat("red")));
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

    #[test]
    fn load_get_route_for_group_name() {
        let loaded =
            Mission::from_miz("C:\\Games\\Eagle Dynamics\\DCS World OpenBeta\\Mods\\aircraft\\Uh-1H\\Missions\\quickStart/UH-1H_MAR_IA_Free Flight.miz".into()).unwrap();
    }

    #[test]
    fn load_get_route_for_group_name2() {
        let loaded = Mission::from_miz(
            "C:\\Users\\benbi\\AppData\\Local\\Temp\\DCS.openbeta\\tempMission.miz".into(),
        )
        .unwrap();
    }
}
