local hylib = require 'hylib'

io.input('../inputs/p16.txt')
local input = io.read('*a')
local disk_size = 35651584 --272

function generate_data(start)
    local a = start
    repeat 
        local b = a:rep(1)
        b = b:reverse()
        b = b:gsub('(%d)', function(s) return s == '1' and '0' or '1' end)
        a = a..'0'..b
    until disk_size <= a:len()
    return a:sub(1, disk_size)
end

function get_checksum(data)
    local cs
    local d = {}
    data:gsub('.', function(c) table.insert(d, c) end)

    repeat
        cs = {}
        for i = 1, #d, 2 do
            if d[i] == d[i+1] then
                cs[#cs+1] = '1'
            else
                cs[#cs+1] = '0'
            end
        end
        d = cs
    until #cs % 2 == 1
    return table.concat(cs)
end

local filler_data = generate_data(input)
local checksum = get_checksum(filler_data)
print(checksum)