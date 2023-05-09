use mlua::Lua;

pub fn load_utils(lua: &Lua) -> Result<(), anyhow::Error> {
    lua.load(include_str!("..\\lua\\serialization.lua"))
        .exec()?;
    Ok(())
}

pub fn load_trigger_mocks(lua: &Lua) -> Result<(), anyhow::Error> {
    lua.load(include_str!("..\\lua\\trigger_mocks.lua"))
        .exec()?;
    Ok(())
}
