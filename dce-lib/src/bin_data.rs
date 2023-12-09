use std::{fs, fmt::Debug};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct BinData {
    pub images: Vec<BinItem>,
    pub template_miz: BinItem,
    pub sounds: Vec<BinItem>,
}

#[derive(Serialize, Deserialize, Clone, PartialEq)]
pub struct BinItem {
    pub name: String,
    #[serde(with = "base64")]
    pub data: Vec<u8>,
}

impl Debug for BinItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("BinItem").field("name", &self.name).field("data", &self.data.len()).finish()
    }
}

impl BinItem {
    pub fn new_from_file(name: &str, path: &str) -> Result<BinItem, anyhow::Error> {
        Ok(BinItem {
            name: name.into(),
            data: fs::read(path)?,
        })
    }
    pub fn from_stored_resource(name: &str, data: &[u8]) -> BinItem {
        BinItem {
            name: name.into(),
            data: data.to_vec(),
        }
    }
}
mod base64 {
    use ::base64::engine::general_purpose::STANDARD_NO_PAD;
    use base64::Engine;

    // From https://users.rust-lang.org/t/serialize-a-vec-u8-to-json-as-base64/57781
    use serde::{Deserialize, Serialize};
    use serde::{Deserializer, Serializer};

    pub fn serialize<S: Serializer>(v: &Vec<u8>, s: S) -> Result<S::Ok, S::Error> {
        let base64 = STANDARD_NO_PAD.encode(v);
        String::serialize(&base64, s)
    }

    pub fn deserialize<'de, D: Deserializer<'de>>(d: D) -> Result<Vec<u8>, D::Error> {
        let base64 = String::deserialize(d)?;
        STANDARD_NO_PAD
            .decode(base64.as_bytes())
            .map_err(serde::de::Error::custom)
    }
}
