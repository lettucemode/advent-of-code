local hylib = require 'hylib'

io.input('../inputs/p21.txt')
local input = io.read('*a')
local password = 'abcdefgh'

local instructions = hylib.iter_to_table(input:splitlines())
for i=1,#instructions do
    local words = hylib.iter_to_table(instructions[i]:gmatch('%w+'))
    if words[1] == 'swap' then
        if words[2] == 'letter' then
            password = password:swap(words[3], words[6])
        else
            password = password:swap(tonumber(words[3])+1, tonumber(words[6])+1)
        end
    elseif words[1] == 'rotate' then
        if words[2] == 'based' then
            local idx = password:find(words[7])
            if idx >= 5 then idx = idx + 1 end
            password = password:rotate(idx)
        else
            local val = tonumber(words[3])
            if words[2] == 'left' then val = val * -1 end
            password = password:rotate(val)
        end
    elseif words[1] == 'reverse' then
        password = password:reverse_sub(tonumber(words[3])+1, tonumber(words[5])+1)
    elseif words[1] == 'move' then
        password = password:move(tonumber(words[3])+1, tonumber(words[6])+1)
    end
    print(password)
end