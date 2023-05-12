use std::{collections::HashMap, iter::repeat};

use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::{
    mappable::{MapPoint, Mappables},
    projections::{convert_dcs_lat_lon, offset},
    serde_utils::LuaFileBased,
    NewFromMission,
};

use log::{info, warn};

#[derive(Deserialize, Serialize, Debug, PartialEq, Validate)]
pub struct TargetList {
    #[validate]
    blue: HashMap<String, Target>,
    #[validate]
    red: HashMap<String, Target>,
}

impl Mappables for TargetList {
    fn to_mappables(&self, instance: &crate::DCEInstance) -> Vec<crate::mappable::MapPoint> {
        let results = self
            .blue
            .iter()
            .zip(repeat("blue"))
            .chain(self.red.iter().zip(repeat("red")))
            .filter_map(|((name, target), side)| match target {
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
                                    side.into(),
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
                                    side.into(),
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
                Target::Strike(strike) => {
                    if strike.class == "vehicle" && strike.class_template.is_some() {
                        let all_groups = instance.mission.get_vehicle_groups();

                        let groups = all_groups
                            .iter()
                            .filter(|g| &g.name == strike.class_template.as_ref().unwrap())
                            .collect::<Vec<_>>();

                        if groups.len() == 1 {
                            return Some(MapPoint::new_from_dcs(
                                groups[0].x,
                                groups[0].y,
                                strike.text.to_owned(),
                                side.into(),
                                "TargetStrike".into(),
                                &instance.projection,
                            ));
                        }
                    }
                    None
                }
            })
            .collect::<Vec<_>>();

        return results;
    }
}

impl LuaFileBased<'_> for TargetList {}

impl NewFromMission for TargetList {
    fn new_from_mission(mission: &crate::mission::Mission) -> Result<Self, anyhow::Error>
    where
        Self: Sized,
    {
        let mut blue_targets: HashMap<String, Target> = HashMap::default();
        let mut red_targets: HashMap<String, Target> = HashMap::default();

        mission.triggers.zones.iter().for_each(|z| {
            let name_splits = z.name.split("_").collect::<Vec<_>>();
            if name_splits.len() < 2 {
                warn!("Expect zone names to be of form <SIDE>_<TYPE>");
            }

            match name_splits[1] {
                "CAP" => {
                    let targets = match name_splits[0] {
                        "BLUE" => &mut blue_targets,
                        _ => &mut red_targets,
                    };
                    targets.insert(
                        z.name.to_owned(),
                        Target::CAP(CAP {
                            priority: 1,
                            ref_point: z.name.to_owned(),
                            radius: name_splits[3]
                                .parse::<f64>()
                                .expect("Failed to parse radius")
                                * 1000.0,
                            axis: name_splits[2].parse().expect("Failed to parse axis"),
                            text: z.name.to_owned(),
                            inactive: false,
                            firepower: TargetFirepower { min: 2, max: 2 },
                        }),
                    );
                }
                "AAR" => {
                    let targets = match name_splits[0] {
                        "BLUE" => &mut blue_targets,
                        _ => &mut red_targets,
                    };
                    targets.insert(
                        z.name.to_owned(),
                        Target::Refueling(Refueling {
                            priority: 1,
                            ref_point: z.name.to_owned(),
                            radius: name_splits[3]
                                .parse::<f64>()
                                .expect("Failed to parse radius")
                                * 1000.0,
                            axis: name_splits[2].parse().expect("Failed to parse axis"),
                            text: z.name.to_owned(),
                            inactive: false,
                            firepower: TargetFirepower { min: 2, max: 2 },
                        }),
                    );
                }
                _ => {
                    warn!("Didn't know what to do with zone {}", z.name);
                }
            }
        });

        mission
            .coalition
            .red
            .countries
            .iter()
            .zip(repeat("red"))
            .filter_map(|(c, side)| c.vehicle.as_ref().zip(Some(side)))
            .flat_map(|(vgd, side)| vgd.groups.as_slice().iter().zip(repeat(side)))
            .for_each(|(vg, side)| {
                let name_splits = vg.name.split("_").collect::<Vec<_>>();
                if name_splits.len() < 2 {
                    return warn!("Expect zone names to be of form <SIDE>_<TYPE>");
                }
                let targets = match side {
                    "red" => &mut blue_targets,
                    _ => &mut red_targets,
                };

                match name_splits[0] {
                    "STRIKE" => {
                        targets.insert(
                            name_splits[1].to_owned(),
                            Target::Strike(Strike {
                                priority: 1,
                                text: name_splits[1].to_owned(),
                                inactive: false,
                                firepower: TargetFirepower { min: 2, max: 2 },
                                class: "vehicle".to_owned(),
                                class_template: Some(vg.name.to_owned()),
                                elements: None,
                            }),
                        );
                    }
                    _ => {}
                }
            });

        Ok(TargetList {
            blue: blue_targets,
            red: red_targets,
        })
    }
}

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

impl Validate for Target {
    fn validate(&self) -> Result<(), validator::ValidationErrors> {
        match self {
            Target::CAP(i) => i.validate(),
            Target::Refueling(i) => i.validate(),
            Target::Intercept(i) => i.validate(),
            Target::FighterSweep(i) => i.validate(),
            Target::Strike(i) => i.validate(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Validate)]
pub struct TargetFirepower {
    #[validate(range(min = 1, max = 20))]
    min: u32,
    #[validate(range(min = 1, max = 20))]
    max: u32,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Validate)]
pub struct CAP {
    #[validate(range(min = 1, max = 50))]
    pub priority: u32,
    #[serde(rename = "refpoint")]
    pub ref_point: String,
    #[validate(range(min = 10000, max = 1000000))]
    pub radius: f64,
    #[validate(range(min = 0, max = 360))]
    pub axis: f64,
    pub text: String,
    pub inactive: bool,
    #[validate]
    pub firepower: TargetFirepower,
    // #[serde(default)]
    // pub attributes: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Validate)]
pub struct Refueling {
    #[validate(range(min = 1, max = 50))]
    pub priority: u32,
    #[serde(rename = "refpoint")]
    pub ref_point: String,
    #[validate(range(min = 10000, max = 1000000))]
    pub radius: f64,
    #[validate(range(min = 0, max = 360))]
    pub axis: f64,
    pub text: String,
    #[serde(default)]
    pub inactive: bool,
    #[validate]
    pub firepower: TargetFirepower,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Validate)]
pub struct Intercept {
    #[validate(range(min = 1, max = 50))]
    pub priority: u32,
    #[serde(default)]
    pub text: String,
    pub base: String,
    #[serde(default)]
    pub inactive: bool,
    #[validate]
    pub firepower: TargetFirepower,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Validate)]
pub struct FighterSweep {
    #[validate(range(min = 1, max = 50))]
    pub priority: u32,
    pub text: String,
    pub x: f64,
    pub y: f64,
    #[serde(default)]
    pub inactive: bool,
    #[validate]
    pub firepower: TargetFirepower,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Validate)]
pub struct Strike {
    #[validate(range(min = 1, max = 50))]
    pub priority: u32,
    #[serde(default)]
    pub text: String,
    #[serde(default)]
    pub inactive: bool,
    #[validate]
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
    use validator::Validate;

    use crate::{mission::Mission, serde_utils::LuaFileBased, NewFromMission};

    use super::TargetList;

    #[test]
    fn load_example() {
        let result = TargetList::from_lua_file("C:\\Users\\Ben\\Saved Games\\DCS.openbeta\\Mods\\tech\\DCE\\Missions\\Campaigns\\War over Tchad 1987-Blue-Mirage-F1EE-3-30 Lorraine\\Init\\targetlist_init.lua".into(), "targetlist".into());
        result.unwrap().validate().unwrap();
    }

    #[test]
    fn save_example() {
        let loaded = TargetList::from_lua_file("C:\\Users\\Ben\\Saved Games\\DCS.openbeta\\Mods\\tech\\DCE\\Missions\\Campaigns\\War over Tchad 1987-Blue-Mirage-F1EE-3-30 Lorraine\\Init\\targetlist_init.lua".into(), "targetlist".into()).unwrap();
        loaded
            .to_lua_file("targetlist_init.lua".into(), "targetlist".into())
            .unwrap();
    }

    #[test]
    fn from_miz() {
        let mission = Mission::from_miz("C:\\Users\\Ben\\Saved Games\\DCS.openbeta\\Mods\\tech\\DCE\\Missions\\Campaigns\\Falklands v1\\Init\\base_mission.miz".into()).unwrap();
        let targets = TargetList::new_from_mission(&mission).unwrap();

        targets
            .to_lua_file("targetlist_sa.lua".into(), "target_list".into())
            .unwrap();
    }
}
