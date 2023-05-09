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

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(untagged)]
pub enum AirBase {
    Fixed(FixedAirBase),
    Ship(ShipBase),
    Farp(FarpBase),
    Reserve(ReserveBase),
    AirStart(AirStartBase),
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
                    let groups = instance.mission.get_vehicle_groups();
                    let group = groups.iter().filter(|g| &g.name == name).next();
                    if let Some(ship_group) = group {
                        return Some(MapPoint::new_from_dcs(
                            ship_group.x,
                            ship_group.y,
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

#[derive(Serialize, Deserialize, Debug, PartialEq)]
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

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct ShipBase {
    unitname: String,
    startup: Option<f64>,
    #[serde(rename = "ATC_frequency")]
    atc_frequency: Option<String>,
}

// impl ShipBase {
//     pub fn to_mappable(&self, mission: Mission) -> MapPoint {
//         let miz_ship = mission.coalition
//     }
// }

impl ValidateSelf for ShipBase {
    fn validate_self(&self) -> Result<(), anyhow::Error> {
        if self.atc_frequency.is_some() && self.atc_frequency.as_ref().unwrap().len() < 3 {
            return Err(anyhow!("ATC Frequency must be set"));
        }
        Ok(())
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
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

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct AirStartBase {
    inactive: Option<bool>,
    x: f64,
    y: f64,
    elevation: f64,
    #[serde(rename = "ATC_frequency")]
    atc_frequency: String,
    #[serde(rename = "BaseAirStart")]
    base_air_start: bool,
    #[serde(rename = "airdromeId")]
    airdrome_id: Option<u16>,
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
        if self.airdrome_id.is_some() {
            return Err(anyhow!("airdromeId must be nil"));
        }

        Ok(())
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct ReserveBase {
    inactive: bool,
    x: f64,
    y: f64,
    elevation: f64,
    #[serde(rename = "ATC_frequency")]
    atc_frequency: String,
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
            .iter()
            .map(|(_, dcs_ab)| {
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
                let parts = s.name.split("_").collect::<Vec<_>>();
                if parts.len() < 2 || parts[0] != "CV" {
                    return None;
                }
                Some((
                    s.name.to_owned(),
                    AirBase::Ship(ShipBase {
                        unitname: s.name.to_owned(),
                        startup: Some(600.),
                        atc_frequency: None,
                    }),
                ))
            })
            .collect::<HashMap<_, _>>();
        fixed.extend(ships_blue);
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
