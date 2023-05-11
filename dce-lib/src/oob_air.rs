use anyhow::anyhow;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, iter::repeat};

use crate::{
    db_airbases::DBAirbases,
    mission::{Country, Mission},
    serde_utils::LuaFileBased,
    NewFromMission,
};

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

impl OobAir {
    /// Sets player to first blue squadron
    pub fn set_player_defaults(&mut self) {
        self.blue.iter_mut().enumerate().for_each(|(i, s)| {
            if i == 0 {
                s.player = true;
            } else {
                s.player = false;
            }
        });
        self.red.iter_mut().for_each(|s| {
            s.player = false;
        });
    }
}

impl NewFromMission for OobAir {
    fn new_from_mission(mission: &Mission) -> Result<Self, anyhow::Error> {
        // get first airbase for each side:
        let airbases = DBAirbases::new_from_mission(&mission)?;
        let blue_airbases = airbases
            .iter()
            .filter(|(_, ab)| ab.get_side() == "blue".to_string())
            .map(|(a, _)| a)
            .collect::<Vec<&String>>();
        let first_blue_name = blue_airbases
            .first()
            .ok_or(anyhow!("No blue airbases found in mission"))?;

        let red_airbases = airbases
            .iter()
            .filter(|(_, ab)| ab.get_side() == "red".to_string())
            .map(|(a, _)| a)
            .collect::<Vec<&String>>();
        let first_red_name = red_airbases
            .first()
            .ok_or(anyhow!("No red airbases found in mission"))?;

        Ok(OobAir {
            blue: side_to_squadrons(
                mission.coalition.blue.countries.as_slice(),
                first_blue_name.to_string(),
            ),
            red: side_to_squadrons(
                mission.coalition.red.countries.as_slice(),
                first_red_name.to_string(),
            ),
        })
    }
}

fn side_to_squadrons(countries: &[Country], base: String) -> Vec<Squadron> {
    countries
        .iter()
        .filter_map(|c| c.plane.as_ref().zip(Some(&c.name)))
        .flat_map(|(vg, country)| vg.groups.iter().zip(repeat(country)))
        .filter_map(|(vg, country)| {
            let unit = vg.units.get(0)?;
            Some(Squadron {
                name: vg.name.to_owned(),
                inactive: false,
                player: false,
                _type: unit._type.to_owned(),
                country: country.to_owned(),
                livery: LiveryEnum::One(unit.livery_id.to_owned()),
                base: base.to_owned(),
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
        .collect::<Vec<_>>()
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
