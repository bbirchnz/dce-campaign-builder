local function apply(side)
    new_statics = {}
    if side == "blue" then
        new_statics = new_statics_blue
        country = mission.coalition.blue.country[1]
    else
        new_statics = new_statics_red
        country = mission.coalition.red.country[1]
    end
    if type(country) ~= "table" then
        return
    end

    if country.static == nil then
        country.static = { group = {} }
    end

    for i, v in ipairs(new_statics) do
        table.insert(country.static.group, v)
    end
end

apply("red")
apply("blue")
