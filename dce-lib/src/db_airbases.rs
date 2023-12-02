use bevy_reflect::{FromReflect, Reflect};

use serde::{Deserialize, Serialize};
use std::{collections::HashMap, iter::repeat};

use crate::{
    db_airbases_internal::DBAirbasesInternal,
    dcs_airbase_export::dcs_airbases_for_theatre,
    editable::{Editable, FieldType, HeaderField, ValidationError, ValidationResult},
    miz_environment::MizEnvironment,
    serde_utils::LuaFileBased,
    DCEInstance, NewFromMission,
};

use anyhow::anyhow;
pub type DBAirbases = HashMap<String, AirBase>;

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(untagged)]
pub enum AirBase {
    Fixed(FixedAirBase),
    Ship(ShipBase),
    Farp(FarpBase),
    Reserve(ReserveBase),
    AirStart(AirStartBase),
}
impl AirBase {
    pub fn get_side(&self) -> String {
        match self {
            AirBase::Fixed(a) => a.side.to_owned(),
            AirBase::Ship(a) => a.side.to_owned(),
            AirBase::Farp(a) => a.side.to_owned(),
            AirBase::Reserve(a) => a.side.to_owned(),
            AirBase::AirStart(a) => a.side.to_owned(),
        }
    }

    pub fn to_editable(&self) -> Option<Box<dyn Editable>> {
        match self {
            AirBase::Fixed(a) => Some(Box::new(a.to_owned())),
            AirBase::Ship(a) => Some(Box::new(a.to_owned())),
            AirBase::Farp(a) => Some(Box::new(a.to_owned())),
            AirBase::AirStart(a) => Some(Box::new(a.to_owned())),
            _ => None,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Reflect, FromReflect)]
pub struct FixedAirBase {
    pub x: f64,
    pub y: f64,
    pub elevation: f64,
    #[serde(rename = "airdromeId")]
    pub airdrome_id: u32,
    #[serde(rename = "ATC_frequency")]
    pub atc_frequency: String,
    #[serde(default)]
    pub startup: u32,
    pub side: String,
    pub divert: bool,
    #[serde(rename = "VOR")]
    vor: Option<String>,
    #[serde(rename = "NDB")]
    ndb: Option<String>,
    #[serde(rename = "TACAN")]
    tacan: Option<String>,
    #[serde(rename = "ILS")]
    ils: Option<String>,
    #[serde(rename = "LimitedParkNb")]
    limited_park_number: u16,
    #[serde(default)]
    pub _name: String,
    #[serde(default)]
    pub inactive: bool,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Reflect, FromReflect)]
pub struct ShipBase {
    pub unitname: String,
    pub startup: Option<f64>,
    #[serde(rename = "ATC_frequency")]
    pub atc_frequency: Option<String>,
    pub side: String,
    #[serde(rename = "LimitedParkNb")]
    pub limited_park_number: u32,
    #[serde(default)]
    pub _name: String,
    #[serde(default)]
    pub inactive: bool,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Reflect, FromReflect)]
pub struct FarpBase {
    pub x: f64,
    pub y: f64,
    pub elevation: f64,
    #[serde(rename = "airdromeId")]
    pub airdrome_id: u64,
    #[serde(rename = "helipadId")]
    pub helipad_id: u64,
    #[serde(rename = "ATC_frequency")]
    pub atc_frequency: String,
    pub side: String,
    pub divert: bool,
    #[serde(default)]
    pub _name: String,
    #[serde(default)]
    pub inactive: bool,
    #[serde(rename = "LimitedParkNb")]
    pub limited_park_number: Option<u16>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Reflect, FromReflect)]
pub struct AirStartBase {
    #[serde(default)]
    pub inactive: bool,
    pub x: f64,
    pub y: f64,
    pub elevation: f64,
    #[serde(rename = "ATC_frequency")]
    atc_frequency: String,
    #[serde(rename = "BaseAirStart")]
    base_air_start: bool,
    pub side: String,
    #[serde(default)]
    pub _name: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Reflect, FromReflect)]
pub struct ReserveBase {
    inactive: bool,
    x: f64,
    y: f64,
    elevation: f64,
    #[serde(rename = "ATC_frequency")]
    atc_frequency: String,
    pub side: String,
    #[serde(default)]
    pub _name: String,
}

impl LuaFileBased<'_> for DBAirbases {}

impl NewFromMission for DBAirbases {
    fn new_from_mission(miz: &MizEnvironment) -> Result<Self, anyhow::Error>
    where
        Self: Sized,
    {
        let dcs_airbases = dcs_airbases_for_theatre(&miz.mission.theatre)?;

        let mut fixed = dcs_airbases
            .values()
            .map(|dcs_ab| {
                let warehouse = miz
                    .warehouses
                    .airports
                    .get(&dcs_ab.frequencies.airdrome_number)
                    .expect("Airport must have an entry in warehouses");
                (
                    dcs_ab.frequencies.name.to_owned(),
                    AirBase::Fixed(FixedAirBase {
                        x: dcs_ab.frequencies.x,
                        y: dcs_ab.frequencies.y,
                        elevation: dcs_ab.frequencies.height,
                        airdrome_id: dcs_ab.frequencies.airdrome_number,
                        atc_frequency: dcs_ab.get_first_uhf_freq(),
                        startup: 600,
                        side: warehouse.coalition.to_lowercase(),
                        divert: false,
                        vor: None,
                        ndb: None,
                        tacan: None,
                        ils: None,
                        limited_park_number: dcs_ab.stands.len() as u16,
                        _name: dcs_ab.frequencies.name.to_owned(),
                        inactive: false,
                    }),
                )
            })
            .collect::<HashMap<_, _>>();

        let ships = miz
            .mission
            .country_iter()
            .filter_map(|(i, side)| i.ship.as_ref().zip(Some(side)))
            .flat_map(|(i, side)| i.groups.as_slice().iter().zip(repeat(side)))
            .flat_map(|(i, side)| i.units.as_slice().iter().zip(repeat(side)))
            .filter_map(|(s, side)| {
                let parts = s.name.split('_').collect::<Vec<_>>();
                if parts.len() < 2 || parts[0].to_lowercase() != "cv" {
                    return None;
                }
                Some((
                    s.name.to_owned(),
                    AirBase::Ship(ShipBase {
                        unitname: s.name.to_owned(),
                        startup: Some(600.),
                        atc_frequency: Some((s.frequency as f64 / 1000000.).to_string()),
                        side: side.to_owned(),
                        limited_park_number: 4,
                        _name: s.name.to_owned(),
                        inactive: false,
                    }),
                ))
            })
            .collect::<HashMap<_, _>>();

        let air_starts = miz.mission.triggers.zones.iter().filter_map(|z| {
            let parts = z.name.split('_').collect::<Vec<_>>();
            if parts.len() < 3 || parts[1].to_lowercase() != "airstart" {
                return None;
            }
            Some((
                parts[2].to_owned(),
                AirBase::AirStart(AirStartBase {
                    inactive: false,
                    x: z.x,
                    y: z.y,
                    elevation: 6000.,
                    atc_frequency: "".into(),
                    base_air_start: true,
                    side: parts[0].to_lowercase(),
                    _name: parts[2].to_owned(),
                }),
            ))
        });

        let farps = miz
            .mission
            .country_iter()
            .filter_map(|(c, side)| c._static.as_ref().zip(Some(side)))
            .flat_map(|(s, side)| s.groups.as_slice().iter().zip(repeat(side)))
            .filter_map(|(sg, side)| {
                let first_unit = sg
                    .units
                    .first()
                    .expect("A static group must have at least one unit");
                let parking_spots = if first_unit._type == "FARP" {
                    4
                } else {
                    sg.units.len() as u16
                };
                match first_unit.category.as_str() {
                    "Heliports" => Some((
                        sg.name.to_owned(),
                        AirBase::Farp(FarpBase {
                            x: sg.x,
                            y: sg.y,
                            elevation: 0.,
                            airdrome_id: first_unit.unit_id,
                            helipad_id: first_unit.unit_id,
                            atc_frequency: first_unit
                                .heliport_frequency
                                .as_ref()
                                .unwrap()
                                .to_owned(),
                            side: side.to_owned(),
                            divert: false,
                            _name: sg.name.to_owned(),
                            inactive: false,
                            limited_park_number: Some(parking_spots),
                        }),
                    )),
                    _ => None,
                }
            })
            .collect::<HashMap<_, _>>();

        fixed.extend(ships);
        fixed.extend(air_starts);
        fixed.extend(farps);
        Ok(fixed)
    }
}

impl Editable for FixedAirBase {
    fn get_header() -> Vec<HeaderField> {
        vec![
            HeaderField::new("_name", "Name", FieldType::String, false),
            HeaderField::new(
                "elevation",
                "Elevation",
                FieldType::Float(|v| format!("{:.1}", v)),
                false,
            ),
            HeaderField::new(
                "side",
                "Side",
                FieldType::FixedEnum(vec!["blue".into(), "red".into(), "neutral".into()]),
                true,
            ),
            HeaderField::new("divert", "Divert", FieldType::Bool, true),
            HeaderField::new(
                "startup",
                "Startup time (min)",
                FieldType::DurationMin,
                true,
            ),
            HeaderField::new("atc_frequency", "Frequency", FieldType::String, false),
            HeaderField::new("inactive", "Inactive", FieldType::Bool, true),
        ]
    }
    fn get_mut_by_name<'a>(instance: &'a mut DCEInstance, name: &str) -> &'a mut Self {
        instance
            .airbases
            .fixed
            .iter_mut()
            .find(|item| item._name == name)
            .unwrap()
    }
    fn get_name(&self) -> String {
        self._name.to_owned()
    }

    fn validate(&self, _: &DCEInstance) -> ValidationResult {
        let mut errors = Vec::default();

        if self.side != "blue" && self.side != "red" && self.side != "neutral" {
            errors.push(ValidationError::new(
                "side",
                "Airbase Side",
                "Side must be blue/red/neutral",
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
        let new_airbases =
            DBAirbasesInternal::from_db_airbases(&DBAirbases::new_from_mission(&instance.miz_env)?);

        instance.airbases.fixed = new_airbases.fixed;

        Ok(())
    }

    fn delete_by_name(instance: &mut DCEInstance, name: &str) -> Result<(), anyhow::Error> {
        let container = &mut instance.airbases.fixed;

        if let Some(index) = container.iter().position(|i| i._name == name) {
            container.remove(index);
            return Ok(());
        }

        Err(anyhow!("Didn't find {}", name))
    }
}

impl Editable for FarpBase {
    fn get_header() -> Vec<HeaderField> {
        vec![
            HeaderField::new("_name", "Name", FieldType::String, false),
            HeaderField::new(
                "elevation",
                "Elevation",
                FieldType::Float(|v| format!("{:.1}", v)),
                false,
            ),
            HeaderField::new(
                "side",
                "Side",
                FieldType::FixedEnum(vec!["blue".into(), "red".into(), "neutral".into()]),
                true,
            ),
            HeaderField::new("divert", "Divert", FieldType::Bool, true),
            HeaderField::new("inactive", "Inactive", FieldType::Bool, true),
        ]
    }
    fn get_mut_by_name<'a>(instance: &'a mut DCEInstance, name: &str) -> &'a mut Self {
        instance
            .airbases
            .farp
            .iter_mut()
            .find(|item| item._name == name)
            .unwrap()
    }
    fn get_name(&self) -> String {
        self._name.to_owned()
    }

    fn validate(&self, _: &DCEInstance) -> ValidationResult {
        let mut errors = Vec::default();

        if self.side != "blue" && self.side != "red" && self.side != "neutral" {
            errors.push(ValidationError::new(
                "side",
                "Airbase Side",
                "Side must be blue/red/neutral",
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
        let new_airbases =
            DBAirbasesInternal::from_db_airbases(&DBAirbases::new_from_mission(&instance.miz_env)?);

        instance.airbases.farp = new_airbases.farp;

        Ok(())
    }

    fn delete_by_name(instance: &mut DCEInstance, name: &str) -> Result<(), anyhow::Error> {
        let container = &mut instance.airbases.farp;

        if let Some(index) = container.iter().position(|i| i._name == name) {
            container.remove(index);
            return Ok(());
        }

        Err(anyhow!("Didn't find {}", name))
    }
}

impl Editable for ShipBase {
    fn get_header() -> Vec<HeaderField> {
        vec![
            HeaderField::new("_name", "Name", FieldType::String, false),
            HeaderField::new(
                "side",
                "Side",
                FieldType::FixedEnum(vec!["blue".into(), "red".into(), "neutral".into()]),
                false,
            ),
            HeaderField::new(
                "limited_park_number",
                "Parking spaces",
                FieldType::Int,
                true,
            ),
            HeaderField::new("inactive", "Inactive", FieldType::Bool, true),
        ]
    }
    fn get_mut_by_name<'a>(instance: &'a mut DCEInstance, name: &str) -> &'a mut Self {
        instance
            .airbases
            .ship
            .iter_mut()
            .find(|item| item._name == name)
            .unwrap()
    }
    fn get_name(&self) -> String {
        self._name.to_owned()
    }

    fn validate(&self, _: &DCEInstance) -> ValidationResult {
        let mut errors = Vec::default();

        if self.side != "blue" && self.side != "red" && self.side != "neutral" {
            errors.push(ValidationError::new(
                "side",
                "Airbase Side",
                "Side must be blue/red/neutral",
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
        let new_airbases =
            DBAirbasesInternal::from_db_airbases(&DBAirbases::new_from_mission(&instance.miz_env)?);

        instance.airbases.ship = new_airbases.ship;

        Ok(())
    }

    fn delete_by_name(instance: &mut DCEInstance, name: &str) -> Result<(), anyhow::Error> {
        let container = &mut instance.airbases.ship;

        if let Some(index) = container.iter().position(|i| i._name == name) {
            container.remove(index);
            return Ok(());
        }

        Err(anyhow!("Didn't find {}", name))
    }
}

impl Editable for AirStartBase {
    fn get_header() -> Vec<HeaderField> {
        vec![
            HeaderField::new("_name", "Name", FieldType::String, false),
            HeaderField::new(
                "side",
                "Side",
                FieldType::FixedEnum(vec!["blue".into(), "red".into(), "neutral".into()]),
                false,
            ),
            HeaderField::new("inactive", "Inactive", FieldType::Bool, true),
        ]
    }
    fn get_mut_by_name<'a>(instance: &'a mut DCEInstance, name: &str) -> &'a mut Self {
        instance
            .airbases
            .air_start
            .iter_mut()
            .find(|item| item._name == name)
            .unwrap()
    }
    fn get_name(&self) -> String {
        self._name.to_owned()
    }

    fn validate(&self, _: &DCEInstance) -> ValidationResult {
        let mut errors = Vec::default();

        if self.side != "blue" && self.side != "red" && self.side != "neutral" {
            errors.push(ValidationError::new(
                "side",
                "Airbase Side",
                "Side must be blue/red/neutral",
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
        let new_airbases =
            DBAirbasesInternal::from_db_airbases(&DBAirbases::new_from_mission(&instance.miz_env)?);

        instance.airbases.air_start = new_airbases.air_start;

        Ok(())
    }

    fn delete_by_name(instance: &mut DCEInstance, name: &str) -> Result<(), anyhow::Error> {
        let container = &mut instance.airbases.air_start;

        if let Some(index) = container.iter().position(|i| i._name == name) {
            container.remove(index);
            return Ok(());
        }

        Err(anyhow!("Didn't find {}", name))
    }
}

#[cfg(test)]
mod tests {
    use crate::{miz_environment::MizEnvironment, serde_utils::LuaFileBased, NewFromMission};

    use super::DBAirbases;

    #[test]
    fn from_miz() {
        let miz_env =
            MizEnvironment::from_miz("test_resources\\base_mission_falklands.miz").unwrap();
        let airbases = DBAirbases::new_from_mission(&miz_env).unwrap();

        airbases
            .to_lua_file("db_airbases_sa.lua".into(), "db_airbases".into())
            .unwrap();
    }
}
