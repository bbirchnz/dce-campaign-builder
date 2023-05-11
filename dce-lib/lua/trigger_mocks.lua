Return = {}
Action = {}
GroundTarget = {
    blue = { percent = 100 },
    red = { percent = 100 }
}

db_airbases = {
    mt = {
        __index = function(table, key)
            return { inactive = false }
        end
    }
}

setmetatable(db_airbases, db_airbases.mt)


function Return.Time()
    return 1
end

function Return.Day()
    return 1
end

function Return.Month()
    return 1
end

function Return.Year()
    return 2001
end

function Return.Mission()
    return 1
end

function Return.AirUnitActive(unit_name)
    return 1
end

function Return.AirUnitReady(unit_name)
    return 1
end

function Return.AirUnitAlive(unit_name)
    return 1
end

function Return.AirUnitBase(unit_name)
    return "A Base"
end

function Return.AirUnitPlayer(unit_name)
    return true
end

function Return.TargetAlive(unit_name)
    return 1
end

function Return.UnitDead(unit_name)
    return true
end

function Return.GroupHidden(unit_name)
    return true
end

function Return.GroupProbability(group_name)
    return 1
end

function Return.ShipGroupInPoly(GroupName, PolyZonesTable) --		(ADD) return boolean whether ship group is in polygon (ADD)
    return true
end

function Return.PlaceLogistic(AirbaseName)
end

function Return.CampFlag(flag)
    return 1
end

function Action.None()
end --void action

function Action.Text(text)
end --add briefing text

function Action.TextPlayMission(arg)
end --add trigger text to briefing text of this mission only if it is playable

function Action.SetCampFlag(flag, value)
end --set campagn flag to value

function Action.AddCampFlag(flag, value)
end --add or subtract to campaign flag

function Action.AddImage(filename)
end                                --add briefing picture

function Action.CampaignEnd(state) -- win/loss/draw
end                                --end campaign

function Action.TargetActive(TargetName, boolean)
end --set target active/inactive

function Action.AirUnitActive(UnitName, boolean)
end --set unit active/inactive

function Action.SideBase(side, BaseName)
end --change le camp d'une base, ATTENTION, deplacer les unites avant--Action.SideBase("blue", "Incirlik Airbase")

function Action.AirUnitBase(UnitName, BaseName)
end --set unit base

function Action.AirUnitPlayer(UnitName, boolean)
end --set unit playable

function Action.AirUnitReinforce(SourceUnitName, DestinationUnitName, destNumber)
end --old transfer solution

function Action.AirUnitReinforce(SourceUnitName, DestinationUnitName)
end --intermediate solution that does not require the number of aircraft to be transferred, since this number is in the reserve squadron

function Action.AirUnitReinforce(DestinationUnitName)
end --simplified solution, but you need the line "reserve = number" in the active squadron

function Action.AirUnitRepair()
end --repair damaged aircraft in all air units

function Action.GroundUnitRepair()
end -- (ADD) M19.f : Repair Ground

function Action.AddGroundTargetIntel(sideName)
end --add ground target intel updates to briefing

function Action.GroupHidden(GroupName, boolean)
end --change vehicle/ship group hidden status

function Action.GroupProbability(GroupName, number)
end --change vehicle/ship group probability status function due to the way stats are reset for a new playrun upon completing a FirstMission, groups probability changed by trigger in first mission will not be carried over to second mission! Repeat trigger on second mission or use the trigger from mission 2 on only for flawless function.

function Action.GroupMove(GroupName, ZoneName)
end -- (ADD) move vehicle group to refpoint (See the DC_CheckTriggers.lua file for more explanation)

function Action.GroupSlave(GroupName, master, bearing, distance)
end -- (ADD)

function Action.ShipMission(GroupName, WPtable, CruiseSpeed, PatrolSpeed, StartTime)
end -- (ADD) assign and run a movement mission to a ship group (See the DC_CheckTriggers.lua file for more explanation)

function Action.TemplateActive(TabFile)
end
