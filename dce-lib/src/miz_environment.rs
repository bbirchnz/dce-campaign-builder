use serde::{Deserialize, Serialize};

use crate::{mission::Mission, mission_warehouses::Warehouses};

/// Container for all pieces of the miz environment
/// Including the basic mission lua, warehouses, possibly dictionary in future
#[derive(Deserialize, Serialize)]
pub struct MizEnvironment {
    pub mission: Mission,
    pub warehouses: Warehouses,
}

impl MizEnvironment {
    pub fn from_miz(miz_filename: &str) -> Result<MizEnvironment, anyhow::Error> {
        Ok(MizEnvironment {
            mission: Mission::from_miz(miz_filename)?,
            warehouses: Warehouses::from_miz(miz_filename)?,
        })
    }
}
