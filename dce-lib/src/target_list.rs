use serde::{Deserialize, Serialize};
use std::{collections::HashMap, iter::repeat};

use crate::{
    miz_environment::MizEnvironment,
    serde_utils::LuaFileBased,
    targets::{
        anti_ship::AntiShipStrike,
        awacs::AWACS,
        cap::CAP,
        fighter_sweep::FighterSweep,
        intercept::Intercept,
        refueling::Refueling,
        strike::{Strike, StrikeElement, StrikeFixedCoordTarget, StrikeNamedStaticTarget},
        TargetFirepower,
    },
    NewFromMission,
};

use log::{info, warn};

#[derive(Deserialize, Serialize, Debug, PartialEq)]
pub struct TargetList {
    pub blue: HashMap<String, Target>,
    pub red: HashMap<String, Target>,
}

impl LuaFileBased<'_> for TargetList {}

impl NewFromMission for TargetList {
    fn new_from_mission(miz: &MizEnvironment) -> Result<Self, anyhow::Error>
    where
        Self: Sized,
    {
        let mut blue_targets: HashMap<String, Target> = HashMap::default();
        let mut red_targets: HashMap<String, Target> = HashMap::default();

        miz.mission.triggers.zones.iter().for_each(|z| {
            let name_splits = z.name.split('_').collect::<Vec<_>>();
            if name_splits.len() < 2 {
                warn!("Expect zone names to be of form <SIDE>_<TYPE>");
            }

            let targets = match name_splits[0].to_lowercase().as_str() {
                "blue" => &mut blue_targets,
                _ => &mut red_targets,
            };

            match name_splits[1].to_lowercase().as_str() {
                "cap" => {
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
                            _side: name_splits[0].to_lowercase(),
                            attributes: Vec::default(),
                        }),
                    );
                }
                "refueling" => {
                    targets.insert(
                        z.name.to_owned(),
                        Target::Refueling(Refueling {
                            priority: 10,
                            ref_point: z.name.to_owned(),
                            radius: name_splits[3]
                                .parse::<f64>()
                                .expect("Failed to parse radius")
                                * 1000.0,
                            axis: name_splits[2].parse().expect("Failed to parse axis"),
                            text: z.name.to_owned(),
                            inactive: false,
                            firepower: TargetFirepower { min: 1, max: 1 },
                            _name: z.name.to_owned(),
                            _side: name_splits[0].to_lowercase(),
                            attributes: Vec::default(),
                        }),
                    );
                }
                "awacs" => {
                    targets.insert(
                        z.name.to_owned(),
                        Target::AWACS(AWACS {
                            priority: 10,
                            ref_point: z.name.to_owned(),
                            radius: name_splits[3]
                                .parse::<f64>()
                                .expect("Failed to parse radius")
                                * 1000.0,
                            axis: name_splits[2].parse().expect("Failed to parse axis"),
                            text: z.name.to_owned(),
                            inactive: false,
                            firepower: TargetFirepower { min: 1, max: 1 },
                            _name: z.name.to_owned(),
                            _side: name_splits[0].to_lowercase(),
                            attributes: Vec::default(),
                        }),
                    );
                }
                "fightersweep" => {
                    targets.insert(
                        z.name.to_owned(),
                        Target::FighterSweep(
                            FighterSweep { priority: 1, text: z.name.to_owned(), x: z.x, y: z.y, inactive: false, firepower: TargetFirepower { min: 2, max: 2 }, _name: z.name.to_owned(), _side: name_splits[0].to_lowercase(), attributes: Vec::default() }
                        ),
                    );
                }
                "staticstrike" => {
                    if name_splits.len() < 4 {
                        panic!("Failed to process {}, should be <SIDE>_STATICSTRIKE_<TGT GROUP NAME>_<TGT NAME>", &z.name);
                    }

                    let tgt_element = StrikeElement::FixedCoord(StrikeFixedCoordTarget {
                        name: name_splits[3].to_owned(),
                        x: z.x,
                        y: z.y,
                    });

                    if let Some(Target::Strike(existing_target)) = targets.get_mut(name_splits[2]) {
                        existing_target.elements.as_mut().expect("There should be elements initialised").push(tgt_element);
                    }
                    else {
                        // if it doesn't exist, create a whole new strike target.
                        let new_target = Strike {
                            priority: 1,
                            text: name_splits[2].to_owned(),
                            inactive: false,
                            firepower: TargetFirepower { min: 2, max: 4 },
                            class: None,
                            class_template: None,
                            elements: Some(vec![tgt_element]),
                            _name: name_splits[2].to_owned(),
                            _side: name_splits[0].to_lowercase(),
                            attributes: Vec::default(),
                            picture: Vec::default(),
                        };
                        targets.insert(name_splits[2].to_owned(), Target::Strike(new_target));
                    }
                }
                _ => {
                    warn!("Didn't know what to do with zone {}", z.name);
                }
            }
        });

        // add vehicle groups
        miz.mission
            .country_iter()
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

                if name_splits[0].to_lowercase().as_str() == "strike" {
                    targets.insert(
                        name_splits[1].to_owned(),
                        Target::Strike(Strike {
                            priority: 1,
                            text: name_splits[1].to_owned(),
                            inactive: false,
                            firepower: TargetFirepower { min: 2, max: 2 },
                            class: Some("vehicle".to_owned()),
                            class_template: Some(vg.name.to_owned()),
                            elements: None,
                            _name: vg.name.to_owned(),
                            _side: "blue".into(),
                            attributes: Vec::default(),
                            picture: Vec::default(),
                        }),
                    );
                }
            });

        // add ship groups:
        miz.mission
            .country_iter()
            .filter_map(|(c, side)| c.ship.as_ref().zip(Some(side)))
            .flat_map(|(sgd, side)| sgd.groups.as_slice().iter().zip(repeat(side)))
            .for_each(|(sg, side)| {
                let targets = match side {
                    "red" => &mut blue_targets,
                    _ => &mut red_targets,
                };

                targets.insert(
                    sg.name.to_owned(),
                    Target::AntiShipStrike(AntiShipStrike {
                        priority: 2,
                        text: sg.name.to_owned(),
                        inactive: false,
                        firepower: TargetFirepower { min: 2, max: 4 },
                        class: "ship".to_owned(),
                        class_template: Some(sg.name.to_owned()),
                        _name: sg.name.to_owned(),
                        _side: "blue".into(),
                        attributes: Vec::default(),
                    }),
                );
            });

        // add static groups:
        miz.mission.country_iter()
        .filter_map(|(c, side)| c._static.as_ref().zip(Some(side)))
        .flat_map(|(sgd, side)| sgd.groups.as_slice().iter().zip(repeat(side)))
        .for_each(|(sg, side)| {
            let name_splits = sg.name.split('_').collect::<Vec<_>>();
            let targets = match side.to_lowercase().as_str() {
                // target group with red side = target for blue to attack
                "blue" => &mut red_targets,
                _ => &mut blue_targets,
            };

            if name_splits[0] != "STATICSTRIKE" {
                info!("Not generating static strike target for {}, if you want this as a target rename as STATICSTRIKE_<TGT GROUP NAME>_<TGT NAME>", sg.name);
                return;
            }

            if name_splits.len() < 3 {
                panic!("Failed to process {}, should be STATICSTRIKE_<TGT GROUP NAME>_<TGT NAME>", &sg.name);
            }

            let tgt_element = StrikeElement::NamedStatic(StrikeNamedStaticTarget{
                name: sg.name.to_owned(),
            });

            if let Some(Target::Strike(existing_target)) = targets.get_mut(name_splits[1]) {
                existing_target.elements.as_mut().expect("There should be elements initialised").push(tgt_element);
            }
            else {
                // if it doesn't exist, create a whole new strike target.
                let new_target = Strike {
                    priority: 1,
                    text: name_splits[1].to_owned(),
                    inactive: false,
                    firepower: TargetFirepower { min: 2, max: 4 },
                    class: Some("static".to_owned()),
                    class_template: None,
                    elements: Some(vec![tgt_element]),
                    _name: name_splits[1].to_owned(),
                    _side: side.to_owned(),
                    attributes: Vec::default(),
                    picture: Vec::default(),
                };

                targets.insert(name_splits[1].to_owned(), Target::Strike(new_target));
            }
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
    AntiShipStrike(AntiShipStrike),
    AWACS(AWACS),
}

#[cfg(test)]
mod tests {

    use crate::{miz_environment::MizEnvironment, serde_utils::LuaFileBased, NewFromMission};

    use super::TargetList;

    #[test]
    fn from_miz() {
        let miz = MizEnvironment::from_miz("test_resources\\base_mission.miz".into()).unwrap();
        let targets = TargetList::new_from_mission(&miz).unwrap();

        targets
            .to_lua_file("..\\target\\targetlist.lua".into(), "target_list".into())
            .unwrap();
    }
}
