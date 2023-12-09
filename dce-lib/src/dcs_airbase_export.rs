/// Projections and airbase exports generated using methods from [PyDCS](https://github.com/pydcs/dcs)
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

impl AirportSet {
    pub fn get_first_freq(&self) -> String {
        let items = &self.frequencies.frequency_list;
        let first = items.first();
        match first {
            Some(FrequencyItem::One(freq)) => freq.to_string(),
            Some(FrequencyItem::Many(freqs)) => freqs.first().unwrap().to_string(),
            None => "".into(),
        }
    }

    pub fn get_first_uhf_freq(&self) -> String {
        let items = &self.frequencies.frequency_list;
        let first_uhf = items
            .iter()
            .filter_map(|f| match f {
                FrequencyItem::One(f) => {
                    if f >= &225000000 && f <= &400000000 {
                        Some(f)
                    } else {
                        None
                    }
                }
                FrequencyItem::Many(fs) => fs
                    .iter()
                    .filter(|f| {
                        if f >= &&225000000 && f <= &&400000000 {
                            true
                        } else {
                            false
                        }
                    })
                    .next(),
            })
            .next();
        match first_uhf {
            Some(f) => (*f as f64 / 1000000.).to_string(),
            None => "".into(),
        }
    }
}

#[derive(Deserialize, Serialize, Debug, PartialEq)]
pub struct Frequencies {
    #[serde(rename = "frequencyList")]
    pub frequency_list: Vec<FrequencyItem>,
    #[serde(rename = "airdromeNumber")]
    pub airdrome_number: u32,
    pub x: f64,
    pub y: f64,
    pub height: f64,
    pub name: String,
}

#[derive(Deserialize, Serialize, Debug, PartialEq)]
#[serde(untagged)]
pub enum FrequencyItem {
    One(u64),
    Many(Vec<u64>),
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
    pub beacons: Vec<Beacons>,
}

#[derive(Deserialize, Serialize, Debug, PartialEq)]
pub struct Beacons {
    #[serde(rename = "beaconId")]
    pub id: String,
    #[serde(rename = "runwayId")]
    pub runway_id: Option<u32>,
    #[serde(rename = "runwaySide")]
    pub runway_side: Option<String>,
    #[serde(rename = "runwayName")]
    pub runway_name: Option<String>,
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
        "Caucasus" => Ok(Airports::from_lua_str(
            include_str!("..\\lua\\standlist_cauc.lua"),
            "airports",
        )?),
        "MarianaIslands" => Ok(Airports::from_lua_str(
            include_str!("..\\lua\\standlist_mar.lua"),
            "airports",
        )?),
        "Nevada" => Ok(Airports::from_lua_str(
            include_str!("..\\lua\\standlist_nv.lua"),
            "airports",
        )?),
        "Normandy" => Ok(Airports::from_lua_str(
            include_str!("..\\lua\\standlist_norm.lua"),
            "airports",
        )?),
        "PersianGulf" => Ok(Airports::from_lua_str(
            include_str!("..\\lua\\standlist_pg.lua"),
            "airports",
        )?),
        "Syria" => Ok(Airports::from_lua_str(
            include_str!("..\\lua\\standlist_sy.lua"),
            "airports",
        )?),
        "TheChannel" => Ok(Airports::from_lua_str(
            include_str!("..\\lua\\standlist_ch.lua"),
            "airports",
        )?),
        "SinaiMap" => Ok(Airports::from_lua_str(
            include_str!("..\\lua\\standlist_si.lua"),
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
