local hylib = {}

--[[
    General methods
--]]

function hylib.nameof(thing)
    for name, v in pairs(_ENV) do if v == thing then return name end end
    return '?'
end

function hylib.iter_to_table(iter, ...)
    local t = {}
    for item in iter do
        t[#t + 1] = item
        for i, f in ipairs({...}) do
            t[#t] = f(t[#t])
        end
    end
    return t
end

function hylib.table_length(t)
    local count = 0
    for _ in pairs(t) do count = count + 1 end
    return count
end

function hylib.init_table_entry(t, k, v)
    if not t[k] then t[k] = v end
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

function hylib.any(t, lambda, ...) 
    for k, v in pairs(t) do if lambda(v, ...) then return true end end
    return false
end

function hylib.contains(t, item)
    for k, v in pairs(t) do if v == item then return true end end
    return false
end

function hylib.where(t, lambda)
    local i = 0
    return function()
        while i < #t do
            i = i + 1
            if lambda(t[i]) then return t[i], i end
        end
    end
end

function hylib.filter(t, lambda)
    local ret = {}
    for k, v in pairs(t) do if lambda(v) then table.insert(ret, v) end end
    return ret
end

--[[
    String extensions
--]]
getmetatable('').__index['splitlines'] = function(s)
    return s:gmatch('[^\r^\n]+')
end

getmetatable('').__index['swap'] = function(s, arg1, arg2)
    if type(arg1) ~= type(arg2) then return nil end
    if type(arg1) == 'string' then
        if s:find(arg1) < s:find(arg2) then
            return s:gsub('('..arg1..')(.*)('..arg2..')', '%3%2%1')
        else
            return s:gsub('('..arg2..')(.*)('..arg1..')', '%3%2%1')
        end
    elseif type(arg1) == 'number' then
        return s:swap(s:sub(arg1,arg1), s:sub(arg2,arg2))
    else return nil end
end

getmetatable('').__index['rotate'] = function(s, n)
    -- n < 0 for left, n < 0 for right
    if n == 0 or n == #s then return s end
    local ret = ''
    for i=1,#s do
        local k = (i-n-1) % #s
        ret = ret..s:sub(k+1,k+1)
    end
    return ret
end

getmetatable('').__index['move'] = function(s, from, to)
    if from == to then return s end
    local ret = ''
    local i = 1
    repeat
        if i == to then
            ret = ret..s:sub(from,from)
        elseif from <= i and i < to then
            ret = ret..s:sub(i+1,i+1)
        elseif to < i and i <= from then
            ret = ret..s:sub(i-1,i-1)
        else
            ret = ret..s:sub(i,i)
        end
        i = i + 1
    until i > #s
    return ret
end

getmetatable('').__index['reverse_sub'] = function(s, from, to)
    local rsub = s:sub(from,to):reverse()
    return s:sub(1,from-1)..rsub..s:sub(to+1,#s)
end

return hylib