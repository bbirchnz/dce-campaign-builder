use std::{collections::HashMap, iter::repeat};

use serde::{Deserialize, Serialize};

use crate::{
    serde_utils::LuaFileBased,
    targets::{
        anti_ship::AntiShipStrike, awacs::AWACS, cap::CAP, fighter_sweep::FighterSweep,
        intercept::Intercept, refueling::Refueling, strike::Strike, TargetFirepower,
    },
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
                            _side: name_splits[0].to_lowercase(),
                            _firepower_min: 2,
                            _firepower_max: 2,
                        }),
                    );
                }
                "Refueling" => {
                    let targets = match name_splits[0] {
                        "BLUE" => &mut blue_targets,
                        _ => &mut red_targets,
                    };
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
                        }),
                    );
                }
                "AWACS" => {
                    let targets = match name_splits[0] {
                        "BLUE" => &mut blue_targets,
                        _ => &mut red_targets,
                    };
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
                            _firepower_min: 1,
                            _firepower_max: 1,
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
                    Target::AntiShipStrike(AntiShipStrike {
                        priority: 2,
                        text: sg.name.to_owned(),
                        inactive: false,
                        firepower: TargetFirepower { min: 2, max: 4 },
                        class: "ship".to_owned(),
                        class_template: Some(sg.name.to_owned()),
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
    AntiShipStrike(AntiShipStrike),
    AWACS(AWACS),
}

#[cfg(test)]
mod tests {

    use crate::{mission::Mission, serde_utils::LuaFileBased, NewFromMission};

    use super::TargetList;

    #[test]
    fn from_miz() {
        let mission = Mission::from_miz("test_resources\\base_mission.miz".into()).unwrap();
        let targets = TargetList::new_from_mission(&mission).unwrap();

        targets
            .to_lua_file("..\\target\\targetlist.lua".into(), "target_list".into())
            .unwrap();
    }
}
