use std::{collections::HashMap, fs::File, io::Read};

use serde::{Deserialize, Serialize};
use zip::ZipArchive;

use crate::serde_utils::LuaFileBased;

#[derive(Serialize, Deserialize)]
pub struct Warehouses {
    pub airports: HashMap<u32, Warehouse>,
}

#[derive(Serialize, Deserialize)]
pub struct Warehouse {
    pub coalition: String,
}

impl LuaFileBased<'_> for Warehouses {}

impl Warehouses {
    pub fn from_miz(miz_filename: &str) -> Result<Warehouses, anyhow::Error> {
        let zipfile = File::open(miz_filename)?;
        let mut archive = ZipArchive::new(zipfile)?;

        let mut warehouses: String = Default::default();

        archive
            .by_name("warehouses")?
            .read_to_string(&mut warehouses)?;

        Warehouses::from_lua_str(&warehouses, "warehouses")
    }
}
