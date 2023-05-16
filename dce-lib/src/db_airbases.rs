use itertools::Itertools;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::{
    dce_utils::ValidateSelf,
    dcs_airbase_export::dcs_airbases_for_theatre,
    mappable::{MapPoint, Mappables},
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

impl ValidateSelf for AirBase {
    fn validate_self(&self) -> Result<(), anyhow::Error> {
        match self {
            AirBase::Fixed(a) => a.validate_self(),
            AirBase::Ship(a) => a.validate_self(),
            AirBase::Farp(a) => a.validate_self(),
            AirBase::Reserve(a) => a.validate_self(),
            AirBase::AirStart(a) => a.validate_self(),
        }
    }
}

pub fn get_by_name(airbases: &DBAirbases, name: &str) -> Option<AirBase> {
    airbases.iter().find_map(|(k, v)| {
        if k == name {
            return Some(v.to_owned());
        }
        None
    })
}

impl Mappables for DBAirbases {
    fn to_mappables(&self, instance: &DCEInstance) -> Vec<crate::mappable::MapPoint> {
        self.iter()
            .filter_map(|(name, ab)| match ab {
                AirBase::Fixed(fixed) => Some(MapPoint::new_from_dcs(
                    fixed.x,
                    fixed.y,
                    name.to_owned(),
                    fixed.side.to_owned(),
                    "FixedAirBase".into(),
                    &instance.projection,
                )),
                AirBase::Ship(_) => {
                    let groups = instance.mission.get_ship_groups();
                    let unit = groups
                        .iter()
                        .flat_map(|g| g.units.as_slice())
                        .find(|s| s.name == *name);
                    if let Some(unit) = unit {
                        return Some(MapPoint::new_from_dcs(
                            unit.x,
                            unit.y,
                            name.to_owned(),
                            "blue".to_owned(),
                            "ShipAirBase".to_owned(),
                            &instance.projection,
                        ));
                    }
                    None
                }
                AirBase::Farp(_) => None,
                AirBase::Reserve(_) => None,
                AirBase::AirStart(_) => None,
            })
            .collect()
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct FixedAirBase {
    pub x: f64,
    pub y: f64,
    pub elevation: f64,
    #[serde(rename = "airdromeId")]
    airdrome_id: u32,
    #[serde(rename = "ATC_frequency")]
    atc_frequency: String,
    startup: Option<f64>,
    pub side: String,
    divert: bool,
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
}

impl ValidateSelf for FixedAirBase {
    fn validate_self(&self) -> Result<(), anyhow::Error> {
        if self.atc_frequency.len() < 3 {
            return Err(anyhow!("ATC Frequency must be set"));
        }
        Ok(())
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct ShipBase {
    pub unitname: String,
    pub startup: Option<f64>,
    #[serde(rename = "ATC_frequency")]
    pub atc_frequency: Option<String>,
    pub side: String,
    #[serde(rename = "LimitedParkNb")]
    pub limited_park_number: u16,
}

impl ValidateSelf for ShipBase {
    fn validate_self(&self) -> Result<(), anyhow::Error> {
        if self.atc_frequency.is_some() && self.atc_frequency.as_ref().unwrap().len() < 3 {
            return Err(anyhow!("ATC Frequency must be set"));
        }
        Ok(())
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
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
}

impl ValidateSelf for FarpBase {
    fn validate_self(&self) -> Result<(), anyhow::Error> {
        if self.atc_frequency.len() < 3 {
            return Err(anyhow!("ATC Frequency must be set"));
        }
        Ok(())
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct AirStartBase {
    #[serde(default)]
    inactive: bool,
    x: f64,
    y: f64,
    elevation: f64,
    #[serde(rename = "ATC_frequency")]
    atc_frequency: String,
    #[serde(rename = "BaseAirStart")]
    base_air_start: bool,
    // #[serde(rename = "airdromeId")]
    // airdrome_id: Option<u16>,
    pub side: String,
}

impl ValidateSelf for AirStartBase {
    fn validate_self(&self) -> Result<(), anyhow::Error> {
        if self.elevation != 0.0 {
            return Err(anyhow!("elevation must = 0.0"));
        }
        if !self.atc_frequency.is_empty() {
            return Err(anyhow!("ATC_frequency must be an empty string"));
        }
        if !self.base_air_start {
            return Err(anyhow!("BaseAirStart must be true"));
        }
        // if self.airdrome_id.is_some() {
        //     return Err(anyhow!("airdromeId must be nil"));
        // }

        Ok(())
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct ReserveBase {
    inactive: bool,
    x: f64,
    y: f64,
    elevation: f64,
    #[serde(rename = "ATC_frequency")]
    atc_frequency: String,
    pub side: String,
}

impl ValidateSelf for ReserveBase {
    fn validate_self(&self) -> Result<(), anyhow::Error> {
        if self.x != 9999999999.0 {
            return Err(anyhow!("x must = 9999999999.0"));
        }
        if self.y != 9999999999.0 {
            return Err(anyhow!("y must = 9999999999.0"));
        }
        if self.elevation != 0.0 {
            return Err(anyhow!("elevation must = 0.0"));
        }
        if !self.atc_frequency.is_empty() {
            return Err(anyhow!("ATC_frequency must be an empty string"));
        }

        Ok(())
    }
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
                        startup: Some(600.),
                        side: "red".into(),
                        divert: false,
                        vor: None,
                        ndb: None,
                        tacan: None,
                        ils: None,
                        limited_park_number: dcs_ab.stands.len() as u16,
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
                    side: "red".into(),
                }),
            ))
        });

        fixed.extend(ships_blue);
        fixed.extend(air_starts);
        Ok(fixed)
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
