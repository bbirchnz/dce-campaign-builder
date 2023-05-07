use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::{
    mappable::{MapPoint, Mappables},
    projections::{convert_dcs_lat_lon, offset},
    serde_utils::LuaFileBased,
};

use log::info;

#[derive(Deserialize, Serialize, Debug, PartialEq)]
pub struct TargetList {
    blue: HashMap<String, Target>,
    red: HashMap<String, Target>,
}

impl Mappables for TargetList {
    fn to_mappables(&self, instance: &crate::DCEInstance) -> Vec<crate::mappable::MapPoint> {
        let results = self
            .blue
            .iter()
            .filter_map(|(name, target)| match target {
                Target::CAP(cap) => {
                    let zone = instance.mission.get_zone_by_name(&cap.ref_point);
                    match zone {
                        Ok(zone) => {
                            let (x2, y2) = offset(zone.x, zone.y, cap.axis, cap.radius);
                            info!("{} {}, {} {}", zone.x, zone.y, x2, y2);
                            let (lon2, lat2) = convert_dcs_lat_lon(x2, y2, &instance.projection);
                            Some(
                                MapPoint::new_from_dcs(
                                    zone.x,
                                    zone.y,
                                    name.to_owned(),
                                    "blue".into(),
                                    "TargetCAP".into(),
                                    &instance.projection,
                                )
                                .add_extras(HashMap::from([
                                    ("radius".to_string(), cap.radius),
                                    ("axis".to_string(), cap.axis),
                                    ("lat2".to_string(), lat2),
                                    ("lon2".to_string(), lon2),
                                ])),
                            )
                        }
                        Err(e) => {
                            info!("{:?}", e);
                            None
                        }
                    }
                }
                Target::Refueling(refuel) => {
                    let zone = instance.mission.get_zone_by_name(&refuel.ref_point);
                    match zone {
                        Ok(zone) => {
                            let (x2, y2) = offset(zone.x, zone.y, refuel.axis, refuel.radius);
                            info!("{} {}, {} {}", zone.x, zone.y, x2, y2);
                            let (lon2, lat2) = convert_dcs_lat_lon(x2, y2, &instance.projection);
                            Some(
                                MapPoint::new_from_dcs(
                                    zone.x,
                                    zone.y,
                                    name.to_owned(),
                                    "blue".into(),
                                    "TargetRefuel".into(),
                                    &instance.projection,
                                )
                                .add_extras(HashMap::from([
                                    ("radius".to_string(), refuel.radius),
                                    ("axis".to_string(), refuel.axis),
                                    ("lat2".to_string(), lat2),
                                    ("lon2".to_string(), lon2),
                                ])),
                            )
                        }
                        Err(e) => {
                            info!("{:?}", e);
                            None
                        }
                    }
                }
                Target::Intercept(_) => None,
                Target::FighterSweep(_) => None,
                Target::Strike(_) => None,
            })
            .collect::<Vec<_>>();

        return results;
    }
}

impl LuaFileBased<'_> for TargetList {}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(tag = "task")]
pub enum Target {
    CAP(CAP),
    Refueling(Refueling),
    Intercept(Intercept),
    #[serde(rename = "Fighter Sweep")]
    FighterSweep(FighterSweep),
    Strike(Strike),
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct TargetFirepower {
    min: f64,
    max: f64,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct CAP {
    pub priority: f64,
    #[serde(rename = "refpoint")]
    pub ref_point: String,
    pub radius: f64,
    pub axis: f64,
    pub text: String,
    pub inactive: bool,
    pub firepower: TargetFirepower,
    // #[serde(default)]
    // pub attributes: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Refueling {
    pub priority: f64,
    #[serde(rename = "refpoint")]
    pub ref_point: String,
    pub radius: f64,
    pub axis: f64,
    pub text: String,
    #[serde(default)]
    pub inactive: bool,
    pub firepower: TargetFirepower,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Intercept {
    pub priority: f64,
    #[serde(default)]
    pub text: String,
    pub base: String,
    #[serde(default)]
    pub inactive: bool,
    pub firepower: TargetFirepower,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct FighterSweep {
    pub priority: f64,
    pub text: String,
    pub x: f64,
    pub y: f64,
    #[serde(default)]
    pub inactive: bool,
    pub firepower: TargetFirepower,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Strike {
    pub priority: f64,
    #[serde(default)]
    pub text: String,
    #[serde(default)]
    pub inactive: bool,
    pub firepower: TargetFirepower,
    #[serde(default = "default_class")]
    pub class: String,
    #[serde(rename = "name")]
    pub class_template: Option<String>,
    pub elements: Option<Vec<StrikeElement>>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(untagged)]
pub enum StrikeElement {
    FixedCoord(StrikeFixedCoordTarget),
    NamedStatic(StrikeNamedStaticTarget),
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct StrikeFixedCoordTarget {
    pub name: String,
    pub x: f64,
    pub y: f64,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct StrikeNamedStaticTarget {
    pub name: String,
}

fn default_class() -> String {
    "static".to_string()
}

#[cfg(test)]
mod tests {
    use crate::serde_utils::LuaFileBased;

    use super::TargetList;

    #[test]
    fn load_example() {
        let result = TargetList::from_lua_file("C:\\Users\\Ben\\Saved Games\\DCS.openbeta\\Mods\\tech\\DCE\\Missions\\Campaigns\\War over Tchad 1987-Blue-Mirage-F1EE-3-30 Lorraine\\Init\\targetlist_init.lua".into(), "targetlist".into());

        result.unwrap();
    }

    #[test]
    fn save_example() {
        let loaded = TargetList::from_lua_file("C:\\Users\\Ben\\Saved Games\\DCS.openbeta\\Mods\\tech\\DCE\\Missions\\Campaigns\\War over Tchad 1987-Blue-Mirage-F1EE-3-30 Lorraine\\Init\\targetlist_init.lua".into(), "targetlist".into()).unwrap();
        loaded
            .to_lua_file("targetlist_init.lua".into(), "targetlist".into())
            .unwrap();
    }
}
