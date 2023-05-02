use std::fs;

use mlua::{serde::de, Lua, LuaSerdeExt};
use serde::{Deserialize, Serialize};

use crate::lua_utils::load_utils;

pub trait LuaFileBased<'a>: Deserialize<'a> + Serialize {
    fn from_lua_file(filename: String, key: String) -> Result<Self, anyhow::Error> {
        let lua = Lua::new();

        // load file:
        let lua_str = fs::read_to_string(filename)?;
        lua.load(&lua_str).exec()?;

        let oob_de = de::Deserializer::new(lua.globals().get(key)?);

        let oob = serde_path_to_error::deserialize::<de::Deserializer, Self>(oob_de)?;

        Ok(oob)
    }

    fn to_lua_file(&self, filename: String, key: String) -> Result<(), anyhow::Error> {
        let lua = Lua::new();
        // load utils:
        load_utils(&lua)?;

        lua.globals().set(key.clone(), lua.to_value(&self)?)?;

        let table = lua
            .load(&format!("TableSerialization({}, 0)", &key))
            .eval::<String>()?;

        let f: String = key + " = " + &table;

        fs::write(filename, f)?;

        Ok(())
    }
}
