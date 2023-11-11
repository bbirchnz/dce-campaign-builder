use serde::{Deserialize, Serialize};

use crate::{
    mission::Mission,
    mission_dictionary::{dict_from_miz, MizDict},
    mission_warehouses::Warehouses,
};

/// Container for all pieces of the miz environment
/// Including the basic mission lua, warehouses, dictionary
#[derive(Deserialize, Serialize)]
pub struct MizEnvironment {
    pub mission: Mission,
    pub warehouses: Warehouses,
    pub dictionary_default: MizDict,
}

impl MizEnvironment {
    pub fn from_miz(miz_filename: &str) -> Result<MizEnvironment, anyhow::Error> {
        Ok(MizEnvironment {
            mission: Mission::from_miz(miz_filename)?,
            warehouses: Warehouses::from_miz(miz_filename)?,
            dictionary_default: dict_from_miz(miz_filename)?,
        })
    }

    pub fn dict_str<'a>(self: &'a MizEnvironment, as_read: &'a str) -> &str {
        if as_read.starts_with("DictKey") {
            return self.dictionary_default[as_read].as_str();
        }
        as_read
    }
}
