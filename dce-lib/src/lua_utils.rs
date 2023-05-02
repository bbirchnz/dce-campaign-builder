use mlua::Lua;

pub fn load_utils(lua: &Lua) -> Result<(), anyhow::Error> {
    lua.load(include_str!("..\\..\\lua\\serialization.lua"))
        .exec()?;
    Ok(())
}
