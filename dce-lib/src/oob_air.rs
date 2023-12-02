use anyhow::anyhow;

use bevy_reflect::{FromReflect, Reflect};
use log::warn;
use proj::Proj;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, iter::repeat};

use crate::{
    db_airbases::{AirBase, DBAirbases},
    editable::{
        Editable, EntityTemplateAction, FieldType, HeaderField, ValidationError, ValidationResult,
    },
    loadouts::str_to_task,
    mappable::Mappables,
    mission::{Country, Mission},
    miz_environment::MizEnvironment,
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

    fn set_to_closest_base(
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
                AirBase::Farp(ab) => Some((name, &ab.side, ab.x, ab.y)),
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

    pub fn squadrons_for_airbase(&self, ab_name: &str) -> Vec<Squadron> {
        let mut squadrons = Vec::default();

        self.blue
            .iter()
            .filter(|s| s.base == ab_name)
            .for_each(|s| squadrons.push(s.to_owned()));

        self.red
            .iter()
            .filter(|s| s.base == ab_name)
            .for_each(|s| squadrons.push(s.to_owned()));

        squadrons
    }
}

impl NewFromMission for OobAir {
    fn new_from_mission(miz: &MizEnvironment) -> Result<Self, anyhow::Error> {
        // get first airbase for each side:
        let airbases = DBAirbases::new_from_mission(miz)?;

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

        let mut oob_air = OobAir {
            blue: side_to_squadrons(
                miz.mission.coalition.blue.countries.as_slice(),
                first_blue_name.to_string(),
            )?,
            red: side_to_squadrons(
                miz.mission.coalition.red.countries.as_slice(),
                first_red_name.to_string(),
            )?,
        };

        oob_air.set_to_closest_base(&miz.mission, &airbases)?;

        Ok(oob_air)
    }
}

/// setup a hashmap so we can lookup and check if a squadron already exists.
/// this is used when more than 4 roles are to be assigned to a squadron, and therefore
/// multiple groups need to be created. So long as they are named <"squadron name">_"anything else"> it should work
fn side_to_squadrons(countries: &[Country], base: String) -> Result<Vec<Squadron>, anyhow::Error> {
    let mut squadron_hm: HashMap<String, Squadron> = HashMap::default();

    countries
        .iter()
        .filter_map(|c| c.plane.as_ref().zip(Some(&c.name)))
        .chain(
            countries
                .iter()
                .filter_map(|c| c.helicopter.as_ref().zip(Some(&c.name))),
        )
        .flat_map(|(vg, country)| vg.groups.iter().zip(repeat(country)))
        .try_for_each(|(vg, country)| {
            let unit = vg
                .units
                .get(0)
                .expect("Plane Group must have at least one unit");
            let name_parts = vg.name.split('_').collect::<Vec<_>>();
            let squadron_name = name_parts[0];

            let squadron = squadron_hm
                .entry(squadron_name.to_string())
                .or_insert(Squadron {
                    name: squadron_name.to_owned(),
                    inactive: false,
                    player: false,
                    _type: unit._type.to_owned(),
                    country: country.to_owned(),
                    livery: LiveryEnum::Many(Vec::default()),
                    base: base.to_owned(),
                    skill: unit.skill.to_owned(),
                    tasks: HashMap::default(),
                    tasks_coef: Some(HashMap::default()),
                    number: 12,
                    reserve: 6,
                });
            // cycle through the units, add liveries and tasks:
            for unit in vg.units.iter() {
                // add liveries
                let livery = unit.livery_id.to_owned();
                if let LiveryEnum::Many(vec) = &mut squadron.livery {
                    if !vec.iter().any(|l| **l == livery) {
                        vec.push(livery);
                    }
                }
                // add task:
                let task = unit
                    .name
                    .split('_')
                    .map(|s| s.to_owned())
                    .collect::<Vec<String>>()[1]
                    .to_owned();

                // check casing and convert as needed
                let task = str_to_task(task.as_str())?;

                squadron.tasks.insert(task.to_owned(), true);

                // and task coef:
                squadron.tasks_coef.as_mut().unwrap().insert(task, 1_f32);
            }
            Ok::<(), anyhow::Error>(())
        })?;

    let result = squadron_hm
        .values()
        .map(|v| v.to_owned())
        .collect::<Vec<_>>();

    Ok(result)
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
            HeaderField::new("player", "Player Squadron", FieldType::Bool, true),
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
        let mut new_oob_air = OobAir::new_from_mission(&instance.miz_env)?;
        new_oob_air.set_player_defaults();
        new_oob_air.set_to_closest_base(
            &instance.miz_env.mission,
            &instance.airbases.to_db_airbases(),
        )?;

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

    fn actions_one_entity() -> Vec<EntityTemplateAction<Self>>
    where
        Self: Sized,
    {
        let set_player =
            |item: &mut Self, instance: &mut DCEInstance| -> Result<(), anyhow::Error> {
                for s in instance.oob_air.blue.iter_mut() {
                    if s.get_name() == item.get_name() {
                        s.player = true;
                    } else {
                        s.player = false;
                    }
                }
                for s in instance.oob_air.red.iter_mut() {
                    if s.get_name() == item.get_name() {
                        s.player = true;
                    } else {
                        s.player = false;
                    }
                }
                item.player = true;
                Ok(())
            };

        vec![EntityTemplateAction::new(
            "Set as player",
            "Set this as the playable squadron",
            set_player,
        )]
    }

    fn related(&self, instance: &DCEInstance) -> Vec<Box<dyn Editable>> {
        let mut res: Vec<Box<dyn Editable>> = Vec::default();

        // add parent airbase:
        let abs = instance.airbases.to_db_airbases();

        let ab = abs
            .iter()
            .find(|(name, _)| name.as_str() == self.get_name())
            .as_ref()
            .unwrap()
            .1
            .to_editable()
            .expect("Not a reserve airbase");

        res.push(ab);

        // add loadouts:
        let all_loadouts = instance.loadouts.to_loadouts();

        let (_, loadouts) = all_loadouts
            .iter()
            .filter(|(name, _)| name.as_str() == self._type)
            .next()
            .expect("There should be a loadout for this aircraft");

        if loadouts.aar.is_some() {
            res.append(&mut convert_loadouts(
                &loadouts
                    .aar
                    .as_ref()
                    .unwrap()
                    .iter()
                    .map(|(_, _type)| _type)
                    .collect::<Vec<_>>(),
            ));
        }

        res
    }
}

fn convert_loadouts<T>(_: &[&T]) -> Vec<Box<dyn Editable>>
where
    T: Editable,
{
    todo!()
}

#[cfg(test)]
mod tests {

    use crate::{miz_environment::MizEnvironment, serde_utils::LuaFileBased, NewFromMission};

    use super::OobAir;

    #[test]
    fn from_miz() {
        let miz =
            MizEnvironment::from_miz("test_resources\\base_mission_falklands.miz".into()).unwrap();
        let oob = OobAir::new_from_mission(&miz).unwrap();

        oob.to_lua_file("oob_sa.lua".into(), "oob_air".into())
            .unwrap();
    }
}
