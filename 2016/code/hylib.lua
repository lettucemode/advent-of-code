local hylib = {}

--[[
    General methods
--]]

function nameof(val)
    for name, v in pairs(_ENV) do if v == val then return name end end
    return '?'
end

--[[
    String extensions
--]]
function splitlines(str)
    return str:gmatch('[^\r^\n]+')
end

local str_mt = getmetatable('')
str_mt.__index['splitlines'] = splitlines

return hylib