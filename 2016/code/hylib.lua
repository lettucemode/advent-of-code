local hylib = {}

--[[
    General methods
--]]

function hylib.nameof(val)
    for name, v in pairs(_ENV) do if v == val then return name end end
    return '?'
end

function hylib.iter_to_table(iter, ...)
    local arr = {}
    for item in iter do
        arr[#arr + 1] = item
        for i, f in ipairs({...}) do
            arr[#arr] = f(arr[#arr])
        end
    end
    return arr
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