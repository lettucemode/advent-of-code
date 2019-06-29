local hylib = require 'hylib'

io.input('../inputs/p9.txt')
local input = io.read('*a')

local count = 0
local multipliers = {}

--part 1
function expand(str)
    local i = 1
    while i <= #str do
        c = str:sub(i,i)
        if c:find('%a+') then
            count = count + 1
            i = i + 1
        else
            local _, j, len, reps = str:find('(%d+)x(%d+)', i)
            count = count + len * reps
            if j == nil then break end
            i = j + 2 + len
        end
    end
end

--part 2
function smart_count(str)
    local i = 1
    while i <= #str do

        local c = str:sub(i,i)
        if c:find('%a+') then
            worth = 1
            for k, mult in pairs(multipliers) do
                worth = worth * mult.val
                mult.remaining = mult.remaining - 1
                if mult.remaining <= 0 then multipliers[k] = nil end
            end
            count = count + worth
            i = i + 1
        else
            local _, j, len, reps = str:find('(%d+)x(%d+)', i)
            for k, mult in pairs(multipliers) do
                mult.remaining = mult.remaining - (j + 2 - i)
                if mult.remaining <= 0 then multipliers[k] = nil end
            end
            table.insert(multipliers, {val=reps, remaining=len})
            i = j + 2
        end
        
    end
end

smart_count(input)
print(string.format('%20.0f', count))