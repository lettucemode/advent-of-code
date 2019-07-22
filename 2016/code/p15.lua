local hylib = require 'hylib'

io.input('../inputs/p15.txt')
local input = io.read('*a')

function setup_discs(data)
    local d = {}
    for line in data:splitlines() do
        local nums = {}
        for num in line:gmatch('%d+') do
            nums[#nums+1] = tonumber(num)
        end
        d[#d+1] = {disc=nums[1],len=nums[2],pos=nums[4]}
    end
    d[#d+1] = {disc=7,len=11,pos=0}
    return d
end

local discs = setup_discs(input)
local time = -1
local done = false
while not done do
    done = true
    time = time + 1
    for i, disc in ipairs(discs) do
        done = done and ((time + disc.pos + i) % disc.len == 0)
    end
end
print(time)