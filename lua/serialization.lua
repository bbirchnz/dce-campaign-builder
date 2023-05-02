--function to turn a table into a string
function TableSerializationAG(t, i)
    local text = "{\n"
    local tab = ""
    for n = 1, i + 1 do --controls the indent for the current text line
        tab = tab .. "\t"
    end
    for k, v in pairs(t) do
        if type(k) == "string" then
            text = text .. tab .. "['" .. k .. "'] = "
        else
            text = text .. tab .. "[" .. k .. "] = "
        end
        if type(v) == "string" then
            text = text .. "'" .. v .. "',\n"
        elseif type(v) == "number" then
            text = text .. v .. ",\n"
        elseif type(v) == "table" then
            text = text .. TableSerializationAG(v, i + 1)
        elseif type(v) == "boolean" then
            if v == true then
                text = text .. "true,\n"
            else
                text = text .. "false,\n"
            end
        elseif type(v) == "function" then
            text = text .. v .. ",\n"
        elseif v == nil then
            text = text .. "nil,\n"
        end
    end
    tab = ""
    for n = 1, i do --indent for closing bracket is one less then previous text line
        tab = tab .. "\t"
    end
    if i == 0 then
        text = text .. tab .. "}\n"  --the last bracket should not be followed by an comma
    else
        text = text .. tab .. "},\n" --all brackets with indent higher than 0 are followed by a comma
    end
    return text
end

--function to turn a table into a string
function TableSerialization(t, i)
    local tab1 = ""
    for n = 1, i do --controls the indent for the current text line
        tab1 = tab1 .. "\t"
    end

    local text = "\n" .. tab1 .. "{\n"

    local tab = ""
    for n = 1, i + 1 do --controls the indent for the current text line
        tab = tab .. "\t"
    end


    for k, v in pairs(t) do
        if type(k) == "string" then
            text = text .. tab .. '["' .. k .. '"] = '
        else
            text = text .. tab .. "[" .. k .. "] = "
        end
        if type(v) == "string" then
            text = text .. '"' .. v .. '",\n'
        elseif type(v) == "number" then
            text = text .. v .. ",\n"
        elseif type(v) == "table" then
            text = text .. TableSerialization(v, i + 1)
        elseif type(v) == "boolean" then
            if v == true then
                text = text .. "true,\n"
            else
                text = text .. "false,\n"
            end
        elseif type(v) == "function" then
            text = text .. v .. ",\n"
        elseif v == nil then
            text = text .. "nil,\n"
        elseif type(v) == "userdata" then
            text = text .. "nil,\n"
        end
    end
    tab = ""
    for n = 1, i do --indent for closing bracket is one less then previous text line
        tab = tab .. "\t"
    end
    if i == 0 then
        text = text .. tab .. "}\n"  --the last bracket should not be followed by an comma
    else
        text = text .. tab .. "},\n" --all brackets with indent higher than 0 are followed by a comma
    end
    return text
end
