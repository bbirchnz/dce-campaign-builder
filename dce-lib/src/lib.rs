use anyhow::anyhow;
use db_airbases::DBAirbases;
use mission::Mission;
use oob_air::OobAir;
use projections::{TransverseMercator, PG, SA};
use serde_utils::LuaFileBased;
use target_list::TargetList;

pub mod db_airbases;
pub mod dce_utils;
pub mod lua_utils;
pub mod mappable;
pub mod mission;
pub mod oob_air;
pub mod projections;
pub mod serde_utils;
pub mod target_list;

pub struct DCEInstance {
    pub oob_air: OobAir,
    pub airbases: DBAirbases,
    pub mission: Mission,
    pub target_list: TargetList,
    pub projection: TransverseMercator,
    pub base_path: String,
}

impl DCEInstance {
    pub fn new(path: String) -> Result<DCEInstance, anyhow::Error> {
        let oob_air =
            OobAir::from_lua_file(format!("{}/oob_air_init.lua", path), "oob_air".into())?;
        let airbases =
            DBAirbases::from_lua_file(format!("{}/db_airbases.lua", path), "db_airbases".into())?;

        let mission = Mission::from_miz(format!("{}/base_mission.miz", path).into())?;

        let target_list = TargetList::from_lua_file(
            format!("{}/targetlist_init.lua", path).into(),
            "targetlist".into(),
        )?;

        let projection = match &*mission.theatre {
            "PersianGulf" => PG,
            "SouthAtlantic" => SA,
            _ => {
                return Err(anyhow!(
                    "TransverseMercator not known for {}",
                    mission.theatre
                ))
            }
        };

        Ok(DCEInstance {
            oob_air,
            airbases,
            mission,
            target_list,
            projection,
            base_path: path,
        })
    }

    pub fn validate(&self) -> Result<(), anyhow::Error> {
        Ok(())
    }
}

trait NewFromMission {
    fn new_from_mission(mission: &Mission) -> Result<Self, anyhow::Error>
    where
        Self: Sized;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn load_init() {
        DCEInstance::new("C:\\Users\\Ben\\Saved Games\\DCS.openbeta\\Mods\\tech\\DCE\\Missions\\Campaigns\\War over Tchad 1987-Blue-Mirage-F1EE-3-30 Lorraine\\Init".into()).unwrap();
    }
}
