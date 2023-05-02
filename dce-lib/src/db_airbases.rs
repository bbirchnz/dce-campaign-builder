use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::{dce_utils::ValidateSelf, serde_utils::LuaFileBased};

use anyhow::anyhow;

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct DBAirbases(pub HashMap<String, AirBase>);

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

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct FixedAirBase {
    pub x: f64,
    pub y: f64,
    pub elevation: f64,
    #[serde(rename = "airdromeId")]
    airdrome_id: u16,
    #[serde(rename = "ATC_frequency")]
    atc_frequency: String,
    startup: Option<f64>,
    side: String,
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
    atc_frequency: String,
}

impl ValidateSelf for ShipBase {
    fn validate_self(&self) -> Result<(), anyhow::Error> {
        if self.atc_frequency.len() < 3 {
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

#[cfg(test)]
mod tests {
    use crate::serde_utils::LuaFileBased;

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
}
