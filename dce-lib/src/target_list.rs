use std::{collections::HashMap, iter::repeat};

use bevy_reflect::{FromReflect, Reflect};
use serde::{Deserialize, Serialize};

use crate::{
    serde_utils::LuaFileBased,
    targets::{cap::CAP, strike::Strike, TargetFirepower},
    NewFromMission,
};

use log::warn;

#[derive(Deserialize, Serialize, Debug, PartialEq)]
pub struct TargetList {
    pub blue: HashMap<String, Target>,
    pub red: HashMap<String, Target>,
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
            let name_splits = z.name.split('_').collect::<Vec<_>>();
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
                            _name: z.name.to_owned(),
                            _side: "blue".into(),
                            _firepower_min: 2,
                            _firepower_max: 2,
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
                            _name: z.name.to_owned(),
                            _side: "blue".into(),
                        }),
                    );
                }
                _ => {
                    warn!("Didn't know what to do with zone {}", z.name);
                }
            }
        });

        // add vehicle groups
        mission
            .coalition
            .red
            .countries
            .iter()
            .zip(repeat("red"))
            .chain(mission.coalition.blue.countries.iter().zip(repeat("blue")))
            .filter_map(|(c, side)| c.vehicle.as_ref().zip(Some(side)))
            .flat_map(|(vgd, side)| vgd.groups.as_slice().iter().zip(repeat(side)))
            .for_each(|(vg, side)| {
                let name_splits = vg.name.split('_').collect::<Vec<_>>();
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
                                _name: vg.name.to_owned(),
                                _side: "blue".into(),
                                _firepower_min: 2,
                                _firepower_max: 2,
                            }),
                        );
                    }
                    _ => {}
                }
            });

        // add ship groups:
        mission
            .coalition
            .red
            .countries
            .iter()
            .zip(repeat("red"))
            .chain(mission.coalition.blue.countries.iter().zip(repeat("blue")))
            .filter_map(|(c, side)| c.ship.as_ref().zip(Some(side)))
            .flat_map(|(sgd, side)| sgd.groups.as_slice().iter().zip(repeat(side)))
            .for_each(|(sg, side)| {
                let targets = match side {
                    "red" => &mut blue_targets,
                    _ => &mut red_targets,
                };

                targets.insert(
                    sg.name.to_owned(),
                    Target::AntiShipStrike(Strike {
                        priority: 2,
                        text: sg.name.to_owned(),
                        inactive: false,
                        firepower: TargetFirepower { min: 2, max: 4 },
                        class: "ship".to_owned(),
                        class_template: Some(sg.name.to_owned()),
                        elements: None,
                        _name: sg.name.to_owned(),
                        _side: "blue".into(),
                        _firepower_min: 2,
                        _firepower_max: 4,
                    }),
                );
            });

        Ok(TargetList {
            blue: blue_targets,
            red: red_targets,
        })
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(tag = "task")]
pub enum Target {
    CAP(CAP),
    Refueling(Refueling),
    Intercept(Intercept),
    #[serde(rename = "Fighter Sweep")]
    FighterSweep(FighterSweep),
    Strike(Strike),
    #[serde(rename = "Anti-ship Strike")]
    AntiShipStrike(Strike),
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Reflect, FromReflect)]
pub struct Refueling {
    pub priority: u32,
    #[serde(rename = "refpoint")]
    pub ref_point: String,
    pub radius: f64,
    pub axis: f64,
    pub text: String,
    #[serde(default)]
    pub inactive: bool,
    pub firepower: TargetFirepower,
    #[serde(default)]
    pub _name: String,
    #[serde(default)]
    pub _side: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Reflect, FromReflect)]
pub struct Intercept {
    pub priority: u32,
    #[serde(default)]
    pub text: String,
    pub base: String,
    #[serde(default)]
    pub inactive: bool,
    pub firepower: TargetFirepower,
    #[serde(default)]
    pub _name: String,
    #[serde(default)]
    pub _side: String,
    #[serde(default)]
    pub _firepower_min: u32,
    #[serde(default)]
    pub _firepower_max: u32,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Reflect, FromReflect)]
pub struct FighterSweep {
    pub priority: u32,
    pub text: String,
    pub x: f64,
    pub y: f64,
    #[serde(default)]
    pub inactive: bool,
    pub firepower: TargetFirepower,
    #[serde(default)]
    pub _name: String,
    #[serde(default)]
    pub _side: String,
    #[serde(default)]
    pub _firepower_min: u32,
    #[serde(default)]
    pub _firepower_max: u32,
}

#[cfg(test)]
mod tests {

    use crate::{mission::Mission, serde_utils::LuaFileBased, NewFromMission};

    use super::TargetList;

    #[test]
    fn load_example() {
        TargetList::from_lua_file("C:\\Users\\Ben\\Saved Games\\DCS.openbeta\\Mods\\tech\\DCE\\Missions\\Campaigns\\War over Tchad 1987-Blue-Mirage-F1EE-3-30 Lorraine\\Init\\targetlist_init.lua".into(), "targetlist".into()).unwrap();
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
