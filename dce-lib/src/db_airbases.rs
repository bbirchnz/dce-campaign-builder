use bevy_reflect::{FromReflect, Reflect};
use itertools::Itertools;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::{
    db_airbases_internal::DBAirbasesInternal,
    dcs_airbase_export::dcs_airbases_for_theatre,
    editable::{Editable, FieldType, HeaderField, ValidationError, ValidationResult},
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
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Reflect, FromReflect)]
pub struct FarpBase {
    x: f64,
    y: f64,
    elevation: f64,
    #[serde(rename = "airdromeId")]
    airdrome_id: u16,
    #[serde(rename = "helipadId")]
    helipad_id: u16,
    #[serde(rename = "ATC_frequency")]
    atc_frequency: String,
    side: String,
    divert: bool,
    #[serde(default)]
    pub _name: String,
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
    // #[serde(rename = "airdromeId")]
    // airdrome_id: Option<u16>,
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
    fn new_from_mission(mission: &crate::mission::Mission) -> Result<Self, anyhow::Error>
    where
        Self: Sized,
    {
        let dcs_airbases = dcs_airbases_for_theatre(&mission.theatre)?;

        let mut fixed = dcs_airbases
            .values()
            .map(|dcs_ab| {
                (
                    dcs_ab.frequencies.name.to_owned(),
                    AirBase::Fixed(FixedAirBase {
                        x: dcs_ab.frequencies.x,
                        y: dcs_ab.frequencies.y,
                        elevation: dcs_ab.frequencies.height,
                        airdrome_id: dcs_ab.frequencies.airdrome_number,
                        atc_frequency: dcs_ab
                            .frequencies
                            .frequency_list
                            .iter()
                            .sorted()
                            .collect::<Vec<_>>()
                            .last()
                            .copied()
                            .copied()
                            .unwrap_or_default()
                            .to_string(),
                        startup: 600,
                        side: "red".into(),
                        divert: false,
                        vor: None,
                        ndb: None,
                        tacan: None,
                        ils: None,
                        limited_park_number: dcs_ab.stands.len() as u16,
                        _name: "".into(),
                    }),
                )
            })
            .collect::<HashMap<_, _>>();

        let ships_blue = mission
            .coalition
            .blue
            .countries
            .iter()
            .filter_map(|i| i.ship.as_ref())
            .flat_map(|i| i.groups.as_slice())
            .flat_map(|i| i.units.as_slice())
            .filter_map(|s| {
                let parts = s.name.split('_').collect::<Vec<_>>();
                if parts.len() < 2 || parts[0] != "CV" {
                    return None;
                }
                Some((
                    s.name.to_owned(),
                    AirBase::Ship(ShipBase {
                        unitname: s.name.to_owned(),
                        startup: Some(600.),
                        atc_frequency: None,
                        side: "blue".into(),
                        limited_park_number: 4,
                        _name: "".into(),
                    }),
                ))
            })
            .collect::<HashMap<_, _>>();

        let air_starts = mission.triggers.zones.iter().filter_map(|z| {
            let parts = z.name.split('_').collect::<Vec<_>>();
            if parts.len() < 3 || parts[1] != "AIRSTART" {
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
                    _name: "".into(),
                }),
            ))
        });

        fixed.extend(ships_blue);
        fixed.extend(air_starts);
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

    fn reset_all_from_miz<'a>(instance: &'a mut DCEInstance) -> Result<(), anyhow::Error> {
        let new_airbases = DBAirbasesInternal::from_db_airbases(
            &DBAirbases::new_from_mission(&instance.mission)?,
            &instance.mission_warehouses,
        );

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

    fn reset_all_from_miz<'a>(instance: &'a mut DCEInstance) -> Result<(), anyhow::Error> {
        let new_airbases = DBAirbasesInternal::from_db_airbases(
            &DBAirbases::new_from_mission(&instance.mission)?,
            &instance.mission_warehouses,
        );

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

    fn reset_all_from_miz<'a>(instance: &'a mut DCEInstance) -> Result<(), anyhow::Error> {
        let new_airbases = DBAirbasesInternal::from_db_airbases(
            &DBAirbases::new_from_mission(&instance.mission)?,
            &instance.mission_warehouses,
        );

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
    use crate::{mission::Mission, serde_utils::LuaFileBased, NewFromMission};

    use super::DBAirbases;

    #[test]
    fn load_example() {
        let result = DBAirbases::from_lua_file("C:\\Users\\Ben\\Saved Games\\DCS.openbeta\\Mods\\tech\\DCE\\Missions\\Campaigns\\War over Tchad 1987-Blue-Mirage-F1EE-3-30 Lorraine\\Init\\db_airbases.lua".into(), "db_airbases".into());

        result.unwrap();
    }

    #[test]
    fn save_example() {
        let oob = DBAirbases::from_lua_file("C:\\Users\\Ben\\Saved Games\\DCS.openbeta\\Mods\\tech\\DCE\\Missions\\Campaigns\\War over Tchad 1987-Blue-Mirage-F1EE-3-30 Lorraine\\Init\\db_airbases.lua".into(), "db_airbases".into()).unwrap();
        oob.to_lua_file("db_airbases.lua".into(), "db_airbases".into())
            .unwrap();
    }

    #[test]
    fn from_miz() {
        let mission = Mission::from_miz("C:\\Users\\Ben\\Saved Games\\DCS.openbeta\\Mods\\tech\\DCE\\Missions\\Campaigns\\Falklands v1\\Init\\base_mission.miz".into()).unwrap();
        let airbases = DBAirbases::new_from_mission(&mission).unwrap();

        airbases
            .to_lua_file("db_airbases_sa.lua".into(), "db_airbases".into())
            .unwrap();
    }
}
