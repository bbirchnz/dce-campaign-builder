use std::collections::HashMap;

use anyhow::anyhow;
use serde::{Deserialize, Serialize};

use crate::serde_utils::LuaFileBased;

pub type Airports = HashMap<u32, AirportSet>;

#[derive(Deserialize, Serialize, Debug, PartialEq)]
pub struct AirportSet {
    pub frequencies: Frequencies,
    pub airport: Airport,
    #[serde(rename = "standlist")]
    #[serde(default)]
    pub stands: Vec<Stand>,
}

#[derive(Deserialize, Serialize, Debug, PartialEq)]
pub struct Frequencies {
    #[serde(rename = "frequencyList")]
    pub frequency_list: Vec<u64>,
    #[serde(rename = "airdromeNumber")]
    pub airdrome_number: u32,
    pub x: f64,
    pub y: f64,
    pub height: f64,
    pub name: String,
}

#[derive(Deserialize, Serialize, Debug, PartialEq)]
pub struct Stand {
    pub x: f64,
    pub y: f64,
    pub name: String,
    pub flag: u32,
    pub crossroad_index: u32,
    pub params: HashMap<String, String>,
}

#[derive(Deserialize, Serialize, Debug, PartialEq)]
pub struct Airport {
    pub code: String,
    pub display_name: String,
    pub reference_point: Point,
}

#[derive(Deserialize, Serialize, Debug, PartialEq)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

impl LuaFileBased<'_> for Airports {}

pub fn dcs_airbases_for_theatre(theatre: &str) -> Result<Airports, anyhow::Error> {
    match theatre {
        "Falklands" => Ok(Airports::from_lua_str(
            include_str!("..\\lua\\standlist_sa.lua"),
            "airports",
        )?),
        _ => Err(anyhow!("Couldn't get DCS airbases for {theatre}")),
    }
}

#[cfg(test)]
mod tests {
    use crate::serde_utils::LuaFileBased;

    use super::Airports;

    #[test]
    fn load_example() {
        let result =
            Airports::from_lua_str(include_str!("..\\lua\\standlist_sa.lua"), "airports".into());

        result.unwrap();
    }
}
