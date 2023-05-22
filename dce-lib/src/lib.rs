use std::{
    fs::{self, File},
    io::{self, Write},
    path::Path,
};

use campaign_header::Header;
use cmp_file::CMPFile;
use conf_mod::ConfMod;
use db_airbases::DBAirbases;
use db_airbases_internal::DBAirbasesInternal;
use loadouts::Loadouts;
use loadouts_internal::LoadoutsInternal;
use mission::Mission;
use oob_air::OobAir;
use projections::{projection_from_theatre, TransverseMercator};
use serde::{Deserialize, Serialize};
use serde_utils::LuaFileBased;
use target_list::TargetList;
use target_list_internal::TargetListInternal;
use trigger::Triggers;
use zip::{write::FileOptions, ZipWriter};

pub mod campaign_header;
pub mod cmp_file;
pub mod conf_mod;
pub mod db_airbases;
pub mod db_airbases_internal;
pub mod dce_utils;
pub mod dcs_airbase_export;
pub mod loadouts;
pub mod loadouts_internal;
pub mod lua_utils;
pub mod mappable;
pub mod mission;
pub mod oob_air;
pub mod projections;
pub mod serde_utils;
pub mod target_list;
pub mod target_list_internal;
pub mod targets;
pub mod trigger;

#[derive(Deserialize, Serialize)]
pub struct DCEInstance {
    pub oob_air: OobAir,
    pub airbases: DBAirbasesInternal,
    pub mission: Mission,
    pub target_list: TargetListInternal,
    pub triggers: Triggers,
    pub loadouts: LoadoutsInternal,
    pub projection: TransverseMercator,
    pub base_path: String,
    pub campaign_header: Header,
    pub conf_mod: ConfMod,
}

impl DCEInstance {
    pub fn new(path: String) -> Result<DCEInstance, anyhow::Error> {
        let oob_air =
            OobAir::from_lua_file(format!("{}/oob_air_init.lua", path), "oob_air".into())?;

        let airbases = DBAirbasesInternal::from_db_airbases(&DBAirbases::from_lua_file(
            format!("{}/db_airbases.lua", path),
            "db_airbases".into(),
        )?);

        let mission = Mission::from_miz(&format!("{}/base_mission.miz", path))?;

        let target_list = TargetListInternal::from_target_list(&TargetList::from_lua_file(
            format!("{}/targetlist_init.lua", path),
            "targetlist".into(),
        )?);

        let triggers = Triggers::from_lua_file(
            format!("{}/camp_triggers_init.lua", path),
            "camp_triggers".into(),
        )?;

        let conf_mod =
            ConfMod::from_lua_file(format!("{}/conf_mod.lua", path), "mission_ini".into())?;

        let loadouts = LoadoutsInternal::from_loadouts(&Loadouts::from_lua_file(
            format!("{}/db_loadouts.lua", path),
            "db_loadouts".into(),
        )?);

        let projection = projection_from_theatre(&mission.theatre)?;

        let header = Header::from_lua_file(format!("{}/camp_init.lua", path), "camp".into())?;

        Ok(DCEInstance {
            oob_air,
            airbases,
            mission,
            triggers,
            target_list,
            loadouts,
            projection,
            conf_mod,
            base_path: path,
            campaign_header: header,
        })
    }

    pub fn new_from_miz(miz_file: &str) -> Result<Self, anyhow::Error> {
        let path = Path::new(&miz_file);
        let base_path = path.parent().unwrap().to_str().unwrap().to_owned();

        let mission = Mission::from_miz(miz_file)?;
        let mut oob_air = OobAir::new_from_mission(&mission)?;
        oob_air.set_player_defaults();

        Ok(DCEInstance {
            target_list: TargetListInternal::from_target_list(&TargetList::new_from_mission(
                &mission,
            )?),
            oob_air,
            projection: projection_from_theatre(&mission.theatre)?,
            base_path,
            campaign_header: Header::new_from_mission(&mission)?,
            airbases: DBAirbasesInternal::from_db_airbases(&DBAirbases::new_from_mission(
                &mission,
            )?),
            triggers: Triggers::new_from_mission(&mission)?,
            loadouts: LoadoutsInternal::from_loadouts(&Loadouts::new_from_mission(&mission)?),
            conf_mod: ConfMod::new(),
            mission,
        })
    }

    pub fn save_to_json(&self, file_name: &str) -> Result<(), anyhow::Error> {
        let f = File::create(file_name)?;
        serde_json::to_writer(f, self)?;
        Ok(())
    }

    pub fn load_from_json(file_name: &str) -> Result<Self, anyhow::Error> {
        let f = File::open(file_name)?;
        let instance = serde_json::from_reader::<File, DCEInstance>(f)?;
        Ok(instance)
    }

    /// Exports the full structure in DCE files and folder structures
    ///
    /// When given the path to DCE's campaigns folder it will create all files
    /// including cmp, miz, batch files ready to run. Good for testing as a campaign
    /// developer, but prefer `export_dce_zip` for distribution.
    ///
    /// # Errors
    ///
    /// Any file write errors or problems with serialization will be returned
    pub fn export_dce_format(&self, dir: &str) -> Result<(), anyhow::Error> {
        let base_path = Path::new(&dir);
        let camp_name = self.campaign_header.title.to_owned();
        let camp_path = base_path.join(&camp_name);

        vec!["Init", "Active", "Debug", "Images", "Debriefing", "Sounds"]
            .iter()
            .try_for_each(|d| fs::create_dir_all(camp_path.join(d)))?;

        let init_path = camp_path.join("Init");

        // create cmp file:
        CMPFile::new(camp_name.to_string()).to_lua_file(
            base_path
                .join(format!("{}.cmp", &camp_name))
                .to_string_lossy()
                .to_string(),
            "campaign".into(),
        )?;

        // create placeholder first and ongoings as copies of the base_mission
        fs::copy(
            Path::new(&self.base_path).join("base_mission.miz"),
            base_path.join(format!("{}_first.miz", &camp_name)),
        )?;
        fs::copy(
            Path::new(&self.base_path).join("base_mission.miz"),
            base_path.join(format!("{}_ongoing.miz", &camp_name)),
        )?;
        fs::copy(
            Path::new(&self.base_path).join("base_mission.miz"),
            init_path.join("base_mission.miz"),
        )?;

        // create FirstMission.bat and SkipMission.bat
        fs::write(
            camp_path.join("FirstMission.bat"),
            include_str!("../resources/FirstMission.bat"),
        )?;
        fs::write(
            camp_path.join("SkipMission.bat"),
            include_str!("../resources/SkipMission.bat"),
        )?;
        // and the sound that seem required
        fs::write(
            camp_path.join("Sounds").join("alarme.wav"),
            include_bytes!("../resources/alarme.wav"),
        )?;
        fs::write(
            init_path.join("path.bat"),
            format!(
                r#"
REM Core or Main DCS ou DCS.beta path, always end the line with \ 
set "pathDCS=C:\Program Files\Eagle Dynamics\\DCS World OpenBeta\"
REM Core or Main DCS ou DCS.beta path, always end the line with \ 
set "pathSavedGames={}\"
REM DCE ScriptMod version not any / or \ and no space before and after = 
set "versionPackageICM=NG"


REM After each change, You must launch the FirsMission.bat for it to be taken into account.
"#,
                base_path
                    .parent()
                    .expect("is campaign folder")
                    .parent()
                    .expect("is dce folder")
                    .parent()
                    .expect("is tech folder")
                    .parent()
                    .expect("is mods folder")
                    .parent()
                    .expect("is dcs saved games folder")
                    .display()
            ),
        )?;

        self.airbases.to_db_airbases().to_lua_file(
            init_path
                .join("db_airbases.lua")
                .to_string_lossy()
                .to_string(),
            "db_airbases".into(),
        )?;
        self.campaign_header.to_lua_file(
            init_path
                .join("camp_init.lua")
                .to_string_lossy()
                .to_string(),
            "camp".into(),
        )?;
        self.oob_air.to_lua_file(
            init_path
                .join("oob_air_init.lua")
                .to_string_lossy()
                .to_string(),
            "oob_air".into(),
        )?;
        self.target_list.to_target_list()?.to_lua_file(
            init_path
                .join("targetlist_init.lua")
                .to_string_lossy()
                .to_string(),
            "targetlist".into(),
        )?;
        self.triggers.to_lua_file(
            init_path
                .join("camp_triggers_init.lua")
                .to_string_lossy()
                .to_string(),
            "camp_triggers".into(),
        )?;
        self.loadouts.to_loadouts().to_lua_file(
            init_path
                .join("db_loadouts.lua")
                .to_string_lossy()
                .to_string(),
            "db_loadouts".into(),
        )?;
        self.conf_mod.to_lua_file(
            init_path.join("conf_mod.lua").to_string_lossy().to_string(),
            "mission_ini".into(),
        )?;
        Ok(())
    }

    pub fn export_dce_zip(&self, zip_file: &str) -> Result<(), anyhow::Error> {
        let file = File::create(zip_file)?;
        let mut zip = zip::ZipWriter::new(file);

        let options = zip::write::FileOptions::default();

        let camp_name = self.campaign_header.title.to_owned();

        let dce_campaigns_folder: String =
            "DCS_SavedGames_Path/Mods/tech/DCE/Missions/Campaigns/".into();
        let campaign_folder: String = dce_campaigns_folder.to_owned() + camp_name.as_str() + "/";

        // create extra empty folders
        vec!["Init", "Active", "Debug", "Images", "Debriefing", "Sounds"]
            .iter()
            .try_for_each(|d| zip.add_directory(campaign_folder.to_owned() + d, options))?;

        // create cmp file:
        CMPFile::new(self.campaign_header.title.to_owned()).add_to_zip(
            "campaign",
            &(dce_campaigns_folder.to_owned() + &format!("{}.cmp", &camp_name)),
            &mut zip,
            &options,
        )?;

        // create placeholder first and ongoings as copies of the base_mission
        copy_to_zip(
            Path::new(&self.base_path)
                .join("base_mission.miz")
                .to_str()
                .expect("Should be a valid path"),
            &(dce_campaigns_folder.to_owned() + &format!("{}_first.miz", &camp_name)),
            &mut zip,
            &options,
        )?;

        copy_to_zip(
            Path::new(&self.base_path)
                .join("base_mission.miz")
                .to_str()
                .expect("Should be a valid path"),
            &(dce_campaigns_folder.to_owned() + &format!("{}_ongoing.miz", &camp_name)),
            &mut zip,
            &options,
        )?;

        copy_to_zip(
            Path::new(&self.base_path)
                .join("base_mission.miz")
                .to_str()
                .expect("Should be a valid path"),
            &(campaign_folder.to_owned() + "init/base_mission.miz"),
            &mut zip,
            &options,
        )?;

        // create FirstMission.bat and SkipMission.bat
        zip.start_file(campaign_folder.to_owned() + "FirstMission.bat", options)?;
        zip.write(include_str!("../resources/FirstMission.bat").as_bytes())?;

        zip.start_file(campaign_folder.to_owned() + "SkipMission.bat", options)?;
        zip.write(include_str!("../resources/SkipMission.bat").as_bytes())?;

        // and the sound that seem required.
        zip.start_file(campaign_folder.to_owned() + "Sounds/alarme.wav", options)?;
        zip.write(include_bytes!("../resources/alarme.wav"))?;

        // build our lua files
        self.target_list.to_target_list()?.add_to_zip(
            "targetlist",
            &(campaign_folder.to_owned() + "init/targetlist_init.lua"),
            &mut zip,
            &options,
        )?;

        self.airbases.to_db_airbases().add_to_zip(
            "db_airbases",
            &(campaign_folder.to_owned() + "init/db_airbases.lua"),
            &mut zip,
            &options,
        )?;

        self.campaign_header.add_to_zip(
            "camp",
            &(campaign_folder.to_owned() + "init/camp_init.lua"),
            &mut zip,
            &options,
        )?;

        self.oob_air.add_to_zip(
            "oob_air",
            &(campaign_folder.to_owned() + "init/oob_air_init.lua"),
            &mut zip,
            &options,
        )?;

        self.triggers.add_to_zip(
            "camp_triggers",
            &(campaign_folder.to_owned() + "init/camp_triggers_init.lua"),
            &mut zip,
            &options,
        )?;

        self.loadouts.to_loadouts().add_to_zip(
            "db_loadouts",
            &(campaign_folder.to_owned() + "init/db_loadouts.lua"),
            &mut zip,
            &options,
        )?;

        self.conf_mod.add_to_zip(
            "mission_ini",
            &(campaign_folder.to_owned() + "init/conf_mod.lua"),
            &mut zip,
            &options,
        )?;

        zip.finish()?;

        Ok(())
    }

    pub fn validate(&self) -> Result<(), anyhow::Error> {
        Ok(())
    }

    pub fn set_mission_name(&mut self, name: String) {
        self.campaign_header.title = name;
    }
}

fn copy_to_zip<T>(
    file_path: &str,
    zip_path: &str,
    zip: &mut ZipWriter<T>,
    options: &FileOptions,
) -> Result<(), anyhow::Error>
where
    T: Write + io::Seek,
{
    let buf = fs::read(file_path)?;

    zip.start_file(zip_path, *options)?;
    zip.write(buf.as_slice())?;

    Ok(())
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
        let mut new_instance = DCEInstance::new_from_miz("C:\\Users\\Ben\\Saved Games\\DCS.openbeta\\Mods\\tech\\DCE\\Missions\\Campaigns\\Falklands v1\\Init\\base_mission.miz".into()).unwrap();
        new_instance.set_mission_name("Falklands v1".into());
        new_instance.oob_air.set_player_defaults();
        new_instance.export_dce_format("test_run\\".into()).unwrap();
    }

    #[test]
    fn json_serde() {
        let mut instance = DCEInstance::new_from_miz("C:\\Users\\Ben\\Saved Games\\DCS.openbeta\\Mods\\tech\\DCE\\Missions\\Campaigns\\Falklands v1\\Init\\base_mission.miz".into()).unwrap();
        instance.set_mission_name("Falklands v1".into());
        instance.save_to_json("test.json").unwrap();

        let second_instance = DCEInstance::load_from_json("test.json").unwrap();

        assert_eq!(&instance.mission.theatre, &second_instance.mission.theatre);
    }

    #[test]
    fn to_zip() {
        let mut new_instance =
            DCEInstance::new_from_miz("test_resources\\base_mission.miz".into()).unwrap();
        new_instance.set_mission_name("Falklands v1".into());
        new_instance.oob_air.set_player_defaults();
        new_instance
            .export_dce_zip("test_run\\test.zip".into())
            .unwrap();
    }
}
