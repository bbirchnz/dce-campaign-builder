use std::{collections::HashMap, fs::File, io::Read};

use zip::ZipArchive;

use crate::serde_utils::LuaFileBased;

impl LuaFileBased<'_> for MizDict {}

pub type MizDict = HashMap<String, String>;

pub fn dict_from_miz(miz_filename: &str) -> Result<MizDict, anyhow::Error> {
    let zipfile = File::open(miz_filename)?;
    let mut archive = ZipArchive::new(zipfile)?;

    let mut dictionary: String = Default::default();

    archive
        .by_name("l10n/DEFAULT/dictionary")?
        .read_to_string(&mut dictionary)?;

    MizDict::from_lua_str(&dictionary, "dictionary")
}

#[cfg(test)]
mod tests {
    use crate::mission_dictionary::dict_from_miz;

    #[test]
    fn load_from_miz() {
        let loaded = dict_from_miz("test_resources\\base_mission.miz".into()).unwrap();

        let test_key = "DictKey_sortie_5";

        assert!(loaded.contains_key(test_key));
        assert_eq!(loaded[test_key], "Falklands v1".to_string());
    }
}
