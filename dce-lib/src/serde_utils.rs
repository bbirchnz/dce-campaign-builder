use std::{
    fmt::Display,
    fs,
    io::{self, Write},
    str::FromStr,
};

use crate::lua_utils::load_utils;
use mlua::{serde::de, Lua, LuaSerdeExt};
use serde::de::Error;
use serde::ser::StdError;
use serde::{Deserialize, Deserializer, Serialize};
use zip::{write::FileOptions, ZipWriter};

pub trait LuaFileBased<'a>: Deserialize<'a> + Serialize {
    fn from_lua_file(filename: String, key: &str) -> Result<Self, anyhow::Error> {
        // load file:
        let lua_str = fs::read_to_string(filename)?;
        Self::from_lua_str(&lua_str, key)
    }

    fn from_lua_str(lua_str: &str, key: &str) -> Result<Self, anyhow::Error> {
        let lua = Lua::new();
        lua.load(lua_str).exec()?;

        let oob_de = de::Deserializer::new(lua.globals().get(key)?);

        let oob = serde_path_to_error::deserialize::<de::Deserializer, Self>(oob_de)?;

        Ok(oob)
    }

    fn to_lua_file(&self, filename: String, key: &str) -> Result<(), anyhow::Error> {
        fs::write(filename, self.to_lua_str(key)?)?;

        Ok(())
    }

    fn to_lua_str(&self, key: &str) -> Result<String, anyhow::Error> {
        let lua = Lua::new();

        load_utils(&lua)?;

        lua.globals().set(key.to_owned(), lua.to_value(&self)?)?;

        let table = lua
            .load(&format!("TableSerialization({}, 0)", &key))
            .eval::<String>()?;

        Ok(key.to_owned() + " = " + &table)
    }

    fn add_to_zip<T>(
        &self,
        key: &str,
        path: &str,
        zip: &mut ZipWriter<T>,
        zip_options: &FileOptions,
    ) -> Result<(), anyhow::Error>
    where
        T: Write + io::Seek,
    {
        let s = self.to_lua_str(key)?;
        zip.start_file(path, *zip_options)?;
        let _ = zip.write(s.as_bytes());

        Ok(())
    }
}

pub fn deserialize_as_string_regardless<'de, D>(deserializer: D) -> Result<String, D::Error>
where
    D: serde::de::Deserializer<'de>,
{
    #[derive(Deserialize)]
    #[serde(untagged)]
    enum IntOrString<'a> {
        Str(&'a str), // attempt no-copy deserialization
        String(String),
        Int(i32),
    }

    let s = match IntOrString::deserialize(deserializer) {
        Ok(field) => match field {
            IntOrString::Str(s) => return Ok(s.to_owned()),
            IntOrString::String(s) => return Ok(s),
            IntOrString::Int(i) => return Ok(i.to_string()),
        },
        Err(err) => {
            return Err(err);
        }
    };
}

pub fn deserialize_as_number_regardless<'de, D, T>(deserializer: D) -> Result<T, D::Error>
where
    D: serde::de::Deserializer<'de>,
    T: FromStr + serde::Deserialize<'de>,
    <T as FromStr>::Err: std::fmt::Display + std::fmt::Debug,
{
    #[derive(Deserialize)]
    #[serde(untagged)]
    enum NumberOrString<'a, T> {
        Str(&'a str), // attempt no-copy deserialization
        String(String),
        Number(T),
    }

    let s = match NumberOrString::deserialize(deserializer) {
        Ok(field) => match field {
            NumberOrString::Str(s) => {
                return s
                    .parse::<T>()
                    .map_err(|e| D::Error::custom(format!("Failed to parse string: {}", e)))
            }
            NumberOrString::String(s) => {
                return s
                    .parse::<T>()
                    .map_err(|e| D::Error::custom(format!("Failed to parse string: {}", e)))
            }
            NumberOrString::Number(n) => return Ok(n),
        },
        Err(err) => {
            return Err(err);
        }
    };
}
