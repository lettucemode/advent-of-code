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

function hylib.init_table_entry(table, key, val)
    if not table[key] then table[key] = val end
end

function hylib.deepcopy(orig)
    local orig_type = type(orig)
    local copy
    if orig_type == 'table' then
        copy = {}
        for orig_key, orig_value in next, orig, nil do
            copy[hylib.deepcopy(orig_key)] = hylib.deepcopy(orig_value)
        end
        setmetatable(copy, hylib.deepcopy(getmetatable(orig)))
    else -- number, string, boolean, etc
        copy = orig
    end
    return copy
end

function hylib.any(tbl, lambda, ...) 
    for k, v in pairs(tbl) do if lambda(v, ...) then return true end end
    return false
end

function hylib.contains(tbl, item)
    for k, v in pairs(tbl) do if v == item then return true end end
    return false
end

function hylib.where(tbl, lambda)
    local i = 0
    return function()
        while i < #tbl do
            i = i + 1
            if lambda(tbl[i]) then return tbl[i], i end
        end
    end
end

function hylib.filter(tbl, lambda)
    local ret = {}
    for k, v in pairs(tbl) do if lambda(v) then table.insert(ret, v) end end
    return ret
end

--[[
    String extensions
--]]
getmetatable('').__index['splitlines'] = function(str)
    return str:gmatch('[^\r^\n]+')
end

--[[
    Table extensions
--]]

return hylib