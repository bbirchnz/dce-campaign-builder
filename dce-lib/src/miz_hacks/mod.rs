//! Collection of patches and so on the modify miz files

use std::io::{Cursor, Read, Write};

use itertools::Itertools;
use mlua::{Lua, LuaSerdeExt};
use zip::{ZipArchive, ZipWriter};

use crate::{
    lua_utils::load_utils,
    mission::{Mission, Route, StaticGroup, StaticGroupPoint, StaticUnit},
    serde_utils::LuaFileBased,
};

/// Create a new static group for a farp from zones named:
/// <SIDE>_FARP_<Name>_<number> e.g. BLUE_FARP_Mt Death_1
///
/// Uses invisible farps, this is designed to be used to allow use of
/// custom constructed farps, or helipads in the maps (especially sinai!)
/// base_freq = first frequency. Each farp will get increments of 0.2 mhz above.
/// Suggest starting at 128.00
pub fn zones_to_farps(miz_zip: &[u8], base_freq: f64) -> Result<Vec<u8>, anyhow::Error> {
    let mut mission_string: String = Default::default();

    let mut miz = ZipArchive::new(Cursor::new(miz_zip))?;

    miz.by_name("mission")?
        .read_to_string(&mut mission_string)?;

    let mission = Mission::from_lua_str(mission_string.as_str(), "mission")?;

    let mut next_unit_id = mission.get_max_unit_id() + 1;
    let mut next_group_id = mission.get_max_group_id() + 1;
    let mut next_freq = base_freq;
    let mut next_callsign_id = 1; // London, Dallas, Paris,...

    // find and farp zones:
    let mut red_farp_groups: Vec<StaticGroup> = Vec::default();
    let mut blue_farp_groups: Vec<StaticGroup> = Vec::default();

    for (key, group) in &mission
        .triggers
        .zones
        .iter()
        .filter(|z| z.name.contains("_FARP_"))
        .group_by(|z| z.name.split('_').collect_vec()[2].to_owned())
    {
        let zones = group.collect::<Vec<_>>();

        let first_unit = zones.first().unwrap(); // we'll definitely have one, otherwise it wouldn't group
        let group = StaticGroup {
            heading: 0.,
            group_id: next_group_id,
            hidden: true,
            x: first_unit.x,
            y: first_unit.y,
            name: key.to_owned(),
            dead: false,
            route: Route {
                points: vec![StaticGroupPoint {
                    alt: 0.,
                    _type: "".into(),
                    name: "".into(),
                    x: first_unit.x,
                    y: first_unit.y,
                    speed: 0.,
                    formation_template: "".into(),
                    action: "".into(),
                }],
            },
            units: zones
                .iter()
                .map(|z| {
                    let unit = StaticUnit {
                        category: "Heliports".into(),
                        shape_name: Some("invisiblefarp".into()),
                        _type: "Invisible FARP".into(),
                        unit_id: next_unit_id,
                        rate: Some(100),
                        x: z.x,
                        y: z.y,
                        name: z.name.to_owned(),
                        heading: 0.,
                        heliport_callsign_id: Some(next_callsign_id), // London
                        heliport_modulation: Some(0),                 // AM=0
                        heliport_frequency: Some(format!("{next_freq:.1}")),
                    };
                    next_unit_id += 1;

                    unit
                })
                .collect(),
        };
        next_callsign_id += 1;
        next_group_id += 1;
        next_freq += 0.2;

        if first_unit.name.starts_with("BLUE") {
            blue_farp_groups.push(group);
        } else {
            red_farp_groups.push(group);
        }
    }

    // apply them to the lua mission
    let lua_mission = Lua::new();
    lua_mission.load(&mission_string).exec()?;
    lua_mission.globals().set(
        "new_statics_blue".to_owned(),
        lua_mission.to_value(&blue_farp_groups)?,
    )?;
    lua_mission.globals().set(
        "new_statics_red".to_owned(),
        lua_mission.to_value(&red_farp_groups)?,
    )?;

    lua_mission
        .load(include_str!("../../lua/add_statics_to_mission.lua"))
        .exec()?;

    // save mission back into miz
    load_utils(&lua_mission)?;

    let lua_str = "mission = ".to_owned()
        + lua_mission
            .load(&format!("TableSerialization({}, 0)", "mission"))
            .eval::<String>()?
            .as_str();

    let mut new_zip_data: Vec<u8> = Vec::default();
    {
        let mut zip = ZipWriter::new(Cursor::new(&mut new_zip_data));
        let options =
            zip::write::FileOptions::default().compression_method(zip::CompressionMethod::Stored);
        zip.start_file("mission", options)?;
        let _ = zip.write(lua_str.as_bytes());

        let file_names = miz.file_names().map(|s| s.to_owned()).collect::<Vec<_>>();

        for name in file_names {
            if name == "mission" {
                continue;
            }
            let mut data = miz.by_name(&name)?;
            if data.is_dir() {
                zip.add_directory(data.name(), options)?;
            }
            if data.is_file() {
                let mut buffer: Vec<u8> = Vec::default();
                data.read_to_end(&mut buffer)?;

                zip.start_file(data.name(), options)?;
                let _ = zip.write(&buffer);
            }
        }
    }

    Ok(new_zip_data)
}

#[cfg(test)]
mod tests {
    use std::{
        fs::File,
        io::{Read, Write},
    };

    use super::zones_to_farps;

    #[test]
    fn read_add_write() {
        let mut original = File::open("test_resources\\farp_test.miz").unwrap();
        let mut buffer = Vec::default();
        original.read_to_end(&mut buffer).unwrap();

        let new_content = zones_to_farps(&buffer, 128.00).unwrap();

        let mut new_file = File::create("test_resources\\farp_test_new.miz").unwrap();
        new_file.write_all(&new_content).unwrap();
    }
}
