use std::{collections::HashMap, iter::repeat};

use bevy_reflect::{FromReflect, Reflect};
use serde::{Deserialize, Serialize};
use tables::{FieldType, HeaderField, TableHeader};
use validator::Validate;

use crate::{serde_utils::LuaFileBased, NewFromMission};

use log::warn;

#[derive(Deserialize, Serialize, Debug, PartialEq, Validate)]
pub struct TargetList {
    #[validate]
    pub blue: HashMap<String, Target>,
    #[validate]
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

impl Validate for Target {
    fn validate(&self) -> Result<(), validator::ValidationErrors> {
        match self {
            Target::CAP(i) => i.validate(),
            Target::Refueling(i) => i.validate(),
            Target::Intercept(i) => i.validate(),
            Target::FighterSweep(i) => i.validate(),
            Target::Strike(i) => i.validate(),
            Target::AntiShipStrike(i) => i.validate(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Validate, Clone, Reflect, FromReflect)]
pub struct TargetFirepower {
    #[validate(range(min = 1, max = 20))]
    min: u32,
    #[validate(range(min = 1, max = 20))]
    max: u32,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Validate, Clone, Reflect, FromReflect)]
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
    #[serde(default)]
    pub _name: String,
    #[serde(default)]
    pub _side: String,
    // #[serde(default)]
    // pub attributes: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Validate, Clone, Reflect, FromReflect)]
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
    #[serde(default)]
    pub _name: String,
    #[serde(default)]
    pub _side: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Validate, Clone, Reflect, FromReflect)]
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
    #[serde(default)]
    pub _name: String,
    #[serde(default)]
    pub _side: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Validate, Clone, Reflect, FromReflect)]
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
    #[serde(default)]
    pub _name: String,
    #[serde(default)]
    pub _side: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Validate, Clone, Reflect, FromReflect)]
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
    #[serde(default)]
    pub _name: String,
    #[serde(default)]
    pub _side: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Reflect, FromReflect)]
#[serde(untagged)]
pub enum StrikeElement {
    FixedCoord(StrikeFixedCoordTarget),
    NamedStatic(StrikeNamedStaticTarget),
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Reflect, FromReflect)]
pub struct StrikeFixedCoordTarget {
    pub name: String,
    pub x: f64,
    pub y: f64,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Reflect, FromReflect)]
pub struct StrikeNamedStaticTarget {
    pub name: String,
}

fn default_class() -> String {
    "static".to_string()
}

impl TableHeader for CAP {
    fn get_header() -> Vec<tables::HeaderField> {
        vec![
            HeaderField {
                field: "text".into(),
                display: "Display Text".into(),
                type_: FieldType::String,
                editable: true,
            },
            HeaderField {
                field: "_side".into(),
                display: "Side".into(),
                type_: FieldType::String,
                editable: false,
            },
            HeaderField {
                field: "priority".into(),
                display: "Priority".into(),
                type_: FieldType::Int,
                editable: true,
            },
            HeaderField {
                field: "firepower".into(),
                display: "Req Firepower".into(),
                type_: FieldType::Debug,
                editable: false,
            },
            HeaderField {
                field: "axis".into(),
                display: "Axis".into(),
                type_: FieldType::Float(|v| format!("{:.0}", v)),
                editable: true,
            },
            HeaderField {
                field: "radius".into(),
                display: "Radius".into(),
                type_: FieldType::Float(|v| format!("{:.0}", v)),
                editable: true,
            },
            HeaderField {
                display: "Inactive".into(),
                field: "inactive".into(),
                type_: FieldType::Bool,
                editable: true,
            },
        ]
    }
}

impl TableHeader for Strike {
    fn get_header() -> Vec<HeaderField> {
        vec![
            HeaderField {
                field: "text".into(),
                display: "Display Text".into(),
                type_: FieldType::String,
                editable: true,
            },
            HeaderField {
                field: "_side".into(),
                display: "Side".into(),
                type_: FieldType::String,
                editable: false,
            },
            HeaderField {
                field: "priority".into(),
                display: "Priority".into(),
                type_: FieldType::Int,
                editable: true,
            },
            HeaderField {
                field: "firepower".into(),
                display: "Req Firepower".into(),
                type_: FieldType::Debug,
                editable: false,
            },
            HeaderField {
                display: "Inactive".into(),
                field: "inactive".into(),
                type_: FieldType::Bool,
                editable: true,
            },
        ]
    }
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
