/// Projections and airbase exports generated using methods from [PyDCS](https://github.com/pydcs/dcs)
use std::collections::HashMap;

use anyhow::anyhow;
use serde::{Deserialize, Serialize};

use crate::{dcs_airbase_export::Airport, serde_utils::LuaFileBased};

pub type Beacons = HashMap<u32, Beacon>;

#[derive(Deserialize, Serialize, Debug, PartialEq)]
pub struct Beacon {
    #[serde(rename = "type")]
    pub _type: u32,
    #[serde(rename = "beaconId")]
    pub id: String,
    #[serde(rename = "sceneObjects")]
    pub scene_objects: Vec<String>,
    #[serde(rename = "chartOffsetX")]
    pub chart_offset_x: Option<f64>,
    pub display_name: String,
    pub position: Vec<f64>,
    #[serde(rename = "positionGeo")]
    pub position_geo: LatLon,
    pub callsign: String,
    pub direction: f64,
    pub frequency: Option<u32>,
    pub channel: Option<u32>,
}

#[derive(Deserialize, Serialize, Debug, PartialEq)]
pub struct LatLon {
    pub longitude: f64,
    pub latitude: f64,
}

impl LuaFileBased<'_> for Beacons {}

const TACAN_TYPE: u32 = 4;
const TACAN_TYPE2: u32 = 5;

pub fn tacan_for_airport(beacons: &Beacons, airport: &Airport) -> Option<String> {
    let airport_beacon_ids = airport
        .beacons
        .iter()
        .map(|b| b.id.to_owned())
        .collect::<Vec<_>>();

    if let Some((_, beacon)) = beacons
        .iter()
        .filter(|(_, b)| {
            airport_beacon_ids.iter().any(|b_id| &b.id == b_id)
                && (b._type == TACAN_TYPE || b._type == TACAN_TYPE2)
                && b.channel.is_some()
        })
        .next()
    {
        return Some(format!("{}X", beacon.channel.unwrap()));
    }

    None
}

pub fn dcs_beacons_for_theatre(theatre: &str) -> Result<Beacons, anyhow::Error> {
    match theatre {
        "Falklands" => Ok(Beacons::from_lua_str(
            include_str!("..\\lua\\standlist_sa.lua"),
            "beacons",
        )?),
        "Caucasus" => Ok(Beacons::from_lua_str(
            include_str!("..\\lua\\standlist_cauc.lua"),
            "beacons",
        )?),
        "MarianaIslands" => Ok(Beacons::from_lua_str(
            include_str!("..\\lua\\standlist_mar.lua"),
            "beacons",
        )?),
        "Nevada" => Ok(Beacons::from_lua_str(
            include_str!("..\\lua\\standlist_nv.lua"),
            "beacons",
        )?),
        "Normandy" => Ok(Beacons::from_lua_str(
            include_str!("..\\lua\\standlist_norm.lua"),
            "beacons",
        )?),
        "PersianGulf" => Ok(Beacons::from_lua_str(
            include_str!("..\\lua\\standlist_pg.lua"),
            "beacons",
        )?),
        "Syria" => Ok(Beacons::from_lua_str(
            include_str!("..\\lua\\standlist_sy.lua"),
            "beacons",
        )?),
        "TheChannel" => Ok(Beacons::from_lua_str(
            include_str!("..\\lua\\standlist_ch.lua"),
            "beacons",
        )?),
        "SinaiMap" => Ok(Beacons::from_lua_str(
            include_str!("..\\lua\\standlist_si.lua"),
            "beacons",
        )?),
        _ => Err(anyhow!("Couldn't get DCS beacons for {theatre}")),
    }
}

#[cfg(test)]
mod tests {
    use crate::serde_utils::LuaFileBased;

    use super::Beacons;

    #[test]
    fn load_example() {
        let result =
            Beacons::from_lua_str(include_str!("..\\lua\\standlist_sa.lua"), "beacons".into());

        result.unwrap();
    }
}
