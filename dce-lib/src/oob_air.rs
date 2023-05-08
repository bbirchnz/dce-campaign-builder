use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::{mission::Mission, serde_utils::LuaFileBased, NewFromMission};

#[derive(Deserialize, Serialize, Debug, PartialEq)]
pub struct OobAir {
    blue: Vec<Squadron>,
    red: Vec<Squadron>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(untagged)]

enum LiveryEnum {
    One(String),
    Many(Vec<String>),
}

#[derive(Deserialize, Serialize, Debug, PartialEq)]
pub struct Squadron {
    name: String,

    #[serde(default)]
    inactive: bool,
    #[serde(default)]
    player: bool,

    #[serde(rename = "type")]
    _type: String,
    country: String,

    livery: LiveryEnum,

    base: String,
    skill: String,
    tasks: HashMap<String, bool>,

    #[serde(rename = "tasksCoef")]
    tasks_coef: Option<HashMap<String, f32>>,
    number: u16,
    reserve: u16,
}

impl LuaFileBased<'_> for OobAir {}

impl NewFromMission for OobAir {
    fn new_from_mission(mission: &Mission) -> Result<Self, anyhow::Error> {
        let blue = mission
            .coalition
            .blue
            .countries
            .iter()
            .filter_map(|c| c.plane.as_ref())
            .map(|g| g.groups.as_slice())
            .map(|g| g.as_ref())
            .flat_map(|vg| vg)
            .filter_map(|vg| {
                let unit = vg.units.get(0)?;
                Some(Squadron {
                    name: vg.name.to_owned(),
                    inactive: false,
                    player: false,
                    _type: unit._type.to_owned(),
                    country: "".into(),
                    livery: LiveryEnum::One(unit.livery_id.to_owned()),
                    base: "".into(),
                    skill: unit.skill.to_owned(),
                    tasks: vg
                        .units
                        .iter()
                        .map(|u| {
                            (
                                u.name
                                    .split("_")
                                    .map(|s| s.to_owned())
                                    .collect::<Vec<String>>()[1]
                                    .to_owned(),
                                true,
                            )
                        })
                        .collect(),
                    tasks_coef: Some(
                        vg.units
                            .iter()
                            .map(|u| {
                                (
                                    u.name
                                        .split("_")
                                        .map(|s| s.to_owned())
                                        .collect::<Vec<String>>()[1]
                                        .to_owned(),
                                    1.0f32,
                                )
                            })
                            .collect(),
                    ),
                    number: 6,
                    reserve: 6,
                })
            })
            .collect::<Vec<_>>();
        let red = mission
            .coalition
            .red
            .countries
            .iter()
            .filter_map(|c| c.plane.as_ref())
            .map(|g| g.groups.as_slice())
            .map(|g| g.as_ref())
            .flat_map(|vg| vg)
            .filter_map(|vg| {
                let unit = vg.units.get(0)?;
                Some(Squadron {
                    name: vg.name.to_owned(),
                    inactive: false,
                    player: false,
                    _type: unit._type.to_owned(),
                    country: "".into(),
                    livery: LiveryEnum::One(unit.livery_id.to_owned()),
                    base: "".into(),
                    skill: unit.skill.to_owned(),
                    tasks: vg
                        .units
                        .iter()
                        .map(|u| {
                            (
                                u.name
                                    .split("_")
                                    .map(|s| s.to_owned())
                                    .collect::<Vec<String>>()[1]
                                    .to_owned(),
                                true,
                            )
                        })
                        .collect(),
                    tasks_coef: Some(
                        vg.units
                            .iter()
                            .map(|u| {
                                (
                                    u.name
                                        .split("_")
                                        .map(|s| s.to_owned())
                                        .collect::<Vec<String>>()[1]
                                        .to_owned(),
                                    1.0f32,
                                )
                            })
                            .collect(),
                    ),
                    number: 6,
                    reserve: 6,
                })
            })
            .collect::<Vec<_>>();
        Ok(OobAir { blue, red })
    }
}

#[cfg(test)]
mod tests {
    use crate::{mission::Mission, serde_utils::LuaFileBased, NewFromMission};

    use super::OobAir;

    #[test]
    fn load_example() {
        let result = OobAir::from_lua_file("C:\\Users\\Ben\\Saved Games\\DCS.openbeta\\Mods\\tech\\DCE\\Missions\\Campaigns\\War over Tchad 1987-Blue-Mirage-F1EE-3-30 Lorraine\\Init\\oob_air_init.lua".into(), "oob_air".into());

        result.unwrap();
    }

    #[test]
    fn save_example() {
        let oob = OobAir::from_lua_file("C:\\Users\\Ben\\Saved Games\\DCS.openbeta\\Mods\\tech\\DCE\\Missions\\Campaigns\\War over Tchad 1987-Blue-Mirage-F1EE-3-30 Lorraine\\Init\\oob_air_init.lua".into(), "oob_air".into()).unwrap();
        oob.to_lua_file("test.lua".into(), "oob_air".into())
            .unwrap();
    }

    #[test]
    fn from_miz() {
        let mission = Mission::from_miz("C:\\Users\\Ben\\Saved Games\\DCS.openbeta\\Mods\\tech\\DCE\\Missions\\Campaigns\\Falklands v1\\Init\\base_mission.miz".into()).unwrap();
        let oob = OobAir::new_from_mission(&mission).unwrap();

        oob.to_lua_file("oob_sa.lua".into(), "oob_air".into())
            .unwrap();
    }
}
