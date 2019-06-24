local hylib = require 'hylib'
local md5 = require 'md5'

io.input('../inputs/p5.txt')
local input = io.read('*a')

local password = {}
local index = 0
local found = 0
while found < 8 do
    local hash = md5.sumhexa(input..index)
    if hash:sub(1, 5) == '00000' then
        local pos = tonumber(hash:sub(6,6))
        if pos and pos > -1 and pos < 8 and not password[pos+1] then
            print(hash)
            password[pos+1] = hash:sub(7,7)
            found = found + 1
        end
    end
    index = index + 1
end

local pw = ''
for i, v in ipairs(password) do
    pw = pw..v
end
print(pw)

--[[
    part 1:
    000004c52f7523dcea0ae987bb4bb7ae
    000005b6777c6a6a5a72d3593ee1bade
    0000049c67c129f031d6d2712e3e011d
    00000307d284ec5fe32c12546f61d675
    00000c4b121a0b7dceb8f719e3e5b9d1
    00000101e84b5e967cba0ba19c7e7e00
    00000574e86e49ffc208b84e771f3487
    00000439e8d28b6d251a65563b7c09d1
    4543c154

    part 2:
]]