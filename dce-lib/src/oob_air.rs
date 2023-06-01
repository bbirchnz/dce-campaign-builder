use anyhow::anyhow;

use bevy_reflect::{FromReflect, Reflect};
use log::warn;
use proj::Proj;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, iter::repeat};

use crate::{
    db_airbases::{AirBase, DBAirbases},
    editable::{Editable, FieldType, HeaderField, ValidationError, ValidationResult},
    mappable::Mappables,
    mission::{Country, Mission},
    serde_utils::LuaFileBased,
    DCEInstance, NewFromMission,
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

    pub fn set_to_closest_base(
        &mut self,
        mission: &Mission,
        airbases: &DBAirbases,
    ) -> Result<(), anyhow::Error> {
        let ship_group = mission.get_ship_groups();
        let air_group = mission.get_plane_groups();

        let mappable_bases = airbases
            .iter()
            .filter_map(|(name, a)| match a {
                AirBase::Fixed(ab) => Some((name, &ab.side, ab.x, ab.y)),
                AirBase::Ship(ab) => {
                    let ship = ship_group
                        .iter()
                        .flat_map(|g| g.units.as_slice())
                        .find(|s| s.name == ab.unitname)
                        .expect("Must be a ship unit that matches airbase");
                    Some((name, &ab.side, ship.x, ship.y))
                }
                AirBase::AirStart(ab) => Some((name, &ab.side, ab.x, ab.y)),
                _ => None,
            })
            .collect::<Vec<_>>();

        let distance = |x1: f64, y1: f64, x2: f64, y2: f64| -> f64 {
            let delta_x = x1 - x2;
            let delta_y = y1 - y2;
            (delta_x.powi(2) + delta_y.powi(2)).sqrt()
        };

        // blue
        for sqn in self.blue.iter_mut() {
            let sqn_group = air_group
                .iter()
                .find(|ag| ag.name == sqn.name)
                .expect("Air group exists with same name as squadron");
            let mut bases = mappable_bases
                .iter()
                .filter_map(|(name, side, x, y)| {
                    if *side == "blue" {
                        return Some((
                            name.to_string(),
                            distance(*x, *y, sqn_group.x, sqn_group.y),
                        ));
                    }
                    None
                })
                .collect::<Vec<_>>();
            bases.sort_by(|(_, dist), (_, dist2)| dist.partial_cmp(dist2).expect("no nan"));
            sqn.base = bases.first().expect("at least one blue base").0.to_owned();
        }

        // red
        for sqn in self.red.iter_mut() {
            let sqn_group = air_group
                .iter()
                .find(|ag| ag.name == sqn.name)
                .expect("Air group exists with same name as squadron");
            let mut bases = mappable_bases
                .iter()
                .filter_map(|(name, side, x, y)| {
                    if *side == "red" {
                        return Some((
                            name.to_string(),
                            distance(*x, *y, sqn_group.x, sqn_group.y),
                        ));
                    }
                    None
                })
                .collect::<Vec<_>>();
            bases.sort_by(|(_, dist), (_, dist2)| dist.partial_cmp(dist2).expect("no nan"));
            sqn.base = bases.first().expect("at least one red base").0.to_owned();
        }

        Ok(())
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

impl Mappables for OobAir {
    fn to_mappables(
        &self,
        instance: &crate::DCEInstance,
        proj: &Proj,
    ) -> Vec<crate::mappable::MapPoint> {
        let airbase_mappables = instance.airbases.to_mappables(instance, proj);

        instance
            .oob_air
            .blue
            .iter()
            .zip(repeat("blue"))
            .chain(instance.oob_air.red.iter().zip(repeat("red")))
            .filter_map(|(squad, side)| {
                // its got the same coords as the airbase, so lets save some trouble
                let mappable = airbase_mappables.iter().find(|ab| ab.name == squad.base);
                match mappable {
                    Some(map) => {
                        let mut map = map.clone();
                        map.name = squad.name.to_owned();
                        map.side = side.to_owned();
                        map.class = "Squadron".into();
                        Some(map)
                    }
                    None => {
                        warn!("Couldn't find airbase mappable for squadron {}", squad.name);
                        None
                    }
                }
            })
            .collect::<Vec<_>>()
    }
}

impl Editable for Squadron {
    fn get_header() -> Vec<HeaderField> {
        vec![
            HeaderField::new("name", "Name", FieldType::String, true),
            HeaderField::new("base", "Base", FieldType::String, true),
            HeaderField::new("country", "Country", FieldType::String, false),
            HeaderField::new("_type", "Airframe", FieldType::String, false),
            HeaderField::new("number", "Number", FieldType::Int, true),
            HeaderField::new("reserve", "Reserve", FieldType::Int, true),
            HeaderField::new("tasks", "Tasks", FieldType::Debug, false),
            HeaderField::new("inactive", "Inactive", FieldType::Bool, true),
        ]
    }
    fn get_mut_by_name<'a>(instance: &'a mut DCEInstance, name: &str) -> &'a mut Squadron {
        instance
            .oob_air
            .red
            .iter_mut()
            .chain(instance.oob_air.blue.iter_mut())
            .find(|s| s.name == name)
            .unwrap()
    }
    fn get_name(&self) -> String {
        self.name.to_string()
    }

    fn validate(&self, instance: &DCEInstance) -> ValidationResult {
        let mut errors = Vec::default();

        if !instance.airbases.airbase_exists(&self.base) {
            errors.push(ValidationError::new(
                "base",
                "Airbase Name",
                "Airbase must be a fixed airbase, ship, farp, reserve or airstart",
            ));
        }

        if errors.is_empty() {
            return ValidationResult::Pass;
        }
        ValidationResult::Fail(errors)
    }

    fn can_reset_from_miz() -> bool {
        true
    }

    fn reset_all_from_miz(instance: &mut DCEInstance) -> Result<(), anyhow::Error> {
        let mut new_oob_air = OobAir::new_from_mission(&instance.mission)?;
        new_oob_air.set_player_defaults();
        new_oob_air.set_to_closest_base(&instance.mission, &instance.airbases.to_db_airbases())?;

        instance.oob_air = new_oob_air;

        Ok(())
    }

    fn delete_by_name(instance: &mut DCEInstance, name: &str) -> Result<(), anyhow::Error> {
        if let Some(index) = instance.oob_air.blue.iter().position(|i| i.name == name) {
            instance.oob_air.blue.remove(index);
            return Ok(());
        }
        if let Some(index) = instance.oob_air.red.iter().position(|i| i.name == name) {
            instance.oob_air.red.remove(index);
            return Ok(());
        }

        Err(anyhow!("Didn't find {}", name))
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
