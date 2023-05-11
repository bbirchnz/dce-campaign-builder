-- Export all known airbases to a json object.
-- Note this doesn't include any radio information :(

local world = world
local Airbase = Airbase
local net = net

local abs = world.getAirbases()

local result = {}

for index, base in ipairs(abs) do
    local info = {}
    info.point = Airbase.getPoint(base)
    info.id = Airbase.getID(base)
    info.desc = Airbase.getDesc(base)
    info.callsign = Airbase.getCallsign(base)
    info.parking = Airbase.getParking(base)
    result[info.callsign] = info
end

local json = net.lua2json(result)

local f = assert(io.open("export_world.json", "w"))
f:write(json)
f:close()
