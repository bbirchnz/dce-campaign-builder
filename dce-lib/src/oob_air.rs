use anyhow::anyhow;

use bevy_reflect::{FromReflect, Reflect};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, iter::repeat};
use tables::{FieldType, HeaderField};

use crate::{
    db_airbases::DBAirbases,
    mission::{Country, Mission},
    serde_utils::LuaFileBased,
    NewFromMission,
};

#[derive(Deserialize, Serialize, Debug, PartialEq, Reflect, FromReflect)]
pub struct OobAir {
    pub blue: Vec<Squadron>,
    pub red: Vec<Squadron>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Reflect, FromReflect, Clone)]
#[serde(untagged)]

pub enum LiveryEnum {
    One(String),
    Many(Vec<String>),
}

#[derive(Deserialize, Serialize, Debug, PartialEq, Reflect, FromReflect, Clone)]
#[reflect(Debug)]
pub struct Squadron {
    pub name: String,

    #[serde(default)]
    pub inactive: bool,
    #[serde(default)]
    pub player: bool,

    #[serde(rename = "type")]
    pub _type: String,
    pub country: String,

    pub livery: LiveryEnum,

    pub base: String,
    pub skill: String,
    pub tasks: HashMap<String, bool>,

    #[serde(rename = "tasksCoef")]
    pub tasks_coef: Option<HashMap<String, f32>>,
    pub number: u32,
    pub reserve: u32,
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
        let airbases = DBAirbases::new_from_mission(mission)?;
        
        let blue_airbases = airbases
            .iter()
            .filter(|(_, ab)| ab.get_side() == *"blue")
            .map(|(a, _)| a)
            .collect::<Vec<&String>>();
        let first_blue_name = blue_airbases
            .first()
            .ok_or(anyhow!("No blue airbases found in mission"))?;

        let red_airbases = airbases
            .iter()
            .filter(|(_, ab)| ab.get_side() == *"red")
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
                                .split('_')
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
                                    .split('_')
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

impl tables::TableHeader for Squadron {
    fn get_header() -> Vec<tables::HeaderField> {
        vec![
            HeaderField {
                display: "Name".into(),
                field: "name".into(),
                type_: FieldType::String,
            },
            HeaderField {
                display: "Country".into(),
                field: "country".into(),
                type_: FieldType::String,
            },
            HeaderField {
                display: "Airframe".into(),
                field: "_type".into(),
                type_: FieldType::String,
            },
            HeaderField {
                display: "Number".into(),
                field: "number".into(),
                type_: FieldType::Int,
            },
            HeaderField {
                display: "Reserve".into(),
                field: "reserve".into(),
                type_: FieldType::Int,
            },
            HeaderField {
                display: "Tasks".into(),
                field: "tasks".into(),
                type_: FieldType::Debug,
            },
            HeaderField {
                display: "Base".into(),
                field: "base".into(),
                type_: FieldType::String,
            },
        ]
    }
}

#[cfg(test)]
mod tests {

    use bevy_reflect::Struct;

    use crate::{mission::Mission, serde_utils::LuaFileBased, NewFromMission};

    use super::OobAir;

    #[test]
    fn introspection() {
        let oob =  OobAir::from_lua_file("C:\\Users\\Ben\\Saved Games\\DCS.openbeta\\Mods\\tech\\DCE\\Missions\\Campaigns\\War over Tchad 1987-Blue-Mirage-F1EE-3-30 Lorraine\\Init\\oob_air_init.lua".into(), "oob_air".into()).unwrap();

        for (i, value) in oob.iter_fields().enumerate() {
            let field_name = oob.name_at(i).unwrap();
            if let Some(value) = value.downcast_ref::<u32>() {
                println!("{} is a u32 with the value: {}", field_name, *value);
            }

            println!(
                "{} is type {}",
                field_name,
                value.get_type_info().type_name()
            );
        }
    }

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
