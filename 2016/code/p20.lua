local hylib = require 'hylib'

io.input('../inputs/p20.txt')
local input = io.read('*a')

local ranges = hylib.iter_to_table(input:splitlines())
table.sort(ranges, function(first, second)
    local num1 = tonumber(select(3, first:find('(%d+)')))
    local num2 = tonumber(select(3, second:find('(%d+)')))
    return num1 < num2
end)

local open_ips = {}
local test = 0
for i, r in ipairs(ranges) do 
    local _, _, low, high = r:find('(%d+)-(%d+)')
    low = tonumber(low)
    high = tonumber(high)

    if test < low then 
        open_ips[#open_ips+1] = test
        test = high + 1
    elseif test <= high then 
        test = high + 1
    end
end

for i, v in ipairs(open_ips) do print(i, v) end