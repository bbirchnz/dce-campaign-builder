use db_airbases::DBAirbases;
use oob_air::OobAir;
use serde_utils::LuaFileBased;

pub mod db_airbases;
pub mod dce_utils;
pub mod lua_utils;
pub mod mission;
pub mod oob_air;
pub mod projections;
pub mod serde_utils;
pub mod target_list;

pub struct DCEInstance {
    pub oob_air: OobAir,
    pub airbases: DBAirbases,
    pub base_path: String,
}

impl DCEInstance {
    pub fn new(path: String) -> Result<DCEInstance, anyhow::Error> {
        let oob_air =
            OobAir::from_lua_file(format!("{}/oob_air_init.lua", path), "oob_air".into())?;
        let airbases =
            DBAirbases::from_lua_file(format!("{}/db_airbases.lua", path), "db_airbases".into())?;
        Ok(DCEInstance {
            oob_air,
            airbases,
            base_path: path,
        })
    }

    pub fn validate(&self) -> Result<(), anyhow::Error> {
        Ok(())
    }
}

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
