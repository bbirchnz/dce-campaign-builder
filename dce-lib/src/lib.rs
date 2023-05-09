use std::{
    fs,
    path::{Path},
};

use campaign_header::Header;
use db_airbases::DBAirbases;
use mission::Mission;
use oob_air::OobAir;
use projections::{projection_from_theatre, TransverseMercator};
use serde_utils::LuaFileBased;
use target_list::TargetList;
use trigger::Triggers;

pub mod campaign_header;
pub mod db_airbases;
pub mod dce_utils;
pub mod dcs_airbase_export;
pub mod lua_utils;
pub mod mappable;
pub mod mission;
pub mod oob_air;
pub mod projections;
pub mod serde_utils;
pub mod target_list;
pub mod trigger;
pub mod loadouts;

pub struct DCEInstance {
    pub oob_air: OobAir,
    pub airbases: DBAirbases,
    pub mission: Mission,
    pub target_list: TargetList,
    pub triggers: Triggers,
    pub projection: TransverseMercator,
    pub base_path: String,
    pub campaign_header: Header,
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

        let triggers = Triggers::from_lua_file(
            format!("{}/camp_triggers_init.lua", path).into(),
            "camp_triggers".into(),
        )?;

        let projection = projection_from_theatre(&mission.theatre)?;

        let header = Header::from_lua_file(format!("{}/camp_init.lua", path), "camp".into())?;

        Ok(DCEInstance {
            oob_air,
            airbases,
            mission,
            triggers,
            target_list,
            projection,
            base_path: path,
            campaign_header: header,
        })
    }

    pub fn new_from_miz(miz_file: String) -> Result<Self, anyhow::Error> {
        let path = Path::new(&miz_file);
        let base_path = path.parent().unwrap().to_str().unwrap().to_owned();

        let mission = Mission::from_miz(miz_file)?;

        Ok(DCEInstance {
            oob_air: OobAir::new_from_mission(&mission)?,
            target_list: TargetList::new_from_mission(&mission)?,
            projection: projection_from_theatre(&mission.theatre)?,
            base_path,
            campaign_header: Header::new_from_mission(&mission)?,
            airbases: DBAirbases::new_from_mission(&mission)?,
            triggers: Triggers::new_from_mission(&mission)?,
            mission,
        })
    }

    pub fn generate_lua(self, dir: String) -> Result<(), anyhow::Error> {
        let path = Path::new(&dir);
        fs::create_dir_all(path)?;
        self.airbases.to_lua_file(
            path.join("db_airbases.lua")
                .to_string_lossy()
                .to_string(),
            "db_airbases".into(),
        )?;
        self.campaign_header.to_lua_file(
            path.join("camp_init.lua")
                .to_string_lossy()
                .to_string(),
            "camp".into(),
        )?;
        self.oob_air.to_lua_file(
            path.join("oob_air_init.lua")
                .to_string_lossy()
                .to_string(),
            "oob_air".into(),
        )?;
        self.target_list.to_lua_file(
            path.join("targetlist_init.lua")
                .to_string_lossy()
                .to_string(),
            "targetlist".into(),
        )?;
        self.triggers.to_lua_file(
            path.join("camp_triggers_init.lua")
                .to_string_lossy()
                .to_string(),
            "camp_triggers".into(),
        )?;
        Ok(())
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

    #[test]
    fn load_from_miz_and_generate() {
        let new_instance = DCEInstance::new_from_miz("C:\\Users\\Ben\\Saved Games\\DCS.openbeta\\Mods\\tech\\DCE\\Missions\\Campaigns\\Falklands v1\\Init\\base_mission.miz".into()).unwrap();

        new_instance.generate_lua("test_run\\".into()).unwrap();
    }
}
