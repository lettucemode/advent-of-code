local hylib = require 'hylib'

io.input('../inputs/p21.txt')
local input = io.read('*a')

function scramble(pw)
    local instructions = hylib.iter_to_table(input:splitlines())
    for i=1,#instructions do
        local words = hylib.iter_to_table(instructions[i]:gmatch('%w+'))
        if words[1] == 'swap' then
            if words[2] == 'letter' then
                pw = pw:swap(words[3], words[6])
            else
                pw = pw:swap(tonumber(words[3])+1, tonumber(words[6])+1)
            end
        elseif words[1] == 'rotate' then
            if words[2] == 'based' then
                local idx = pw:find(words[7])
                if idx >= 5 then idx = idx + 1 end
                pw = pw:rotate(idx)
            else
                local val = tonumber(words[3])
                if words[2] == 'left' then val = val * -1 end
                pw = pw:rotate(val)
            end
        elseif words[1] == 'reverse' then
            pw = pw:reverse_sub(tonumber(words[3])+1, tonumber(words[5])+1)
        elseif words[1] == 'move' then
            pw = pw:move(tonumber(words[3])+1, tonumber(words[6])+1)
        end
    end
    return pw
end


function unscramble(pw)
    local instructions = hylib.iter_to_table(input:splitlines())
    for i=#instructions,1,-1 do
        local words = hylib.iter_to_table(instructions[i]:gmatch('%w+'))
        if words[1] == 'swap' then
            if words[2] == 'letter' then
                pw = pw:swap(words[3], words[6])
            else
                pw = pw:swap(tonumber(words[3])+1, tonumber(words[6])+1)
            end
        elseif words[1] == 'rotate' then
            if words[2] == 'based' then
                local idx = pw:find(words[7]) - 1
                if idx == 0 then idx = 7
                elseif idx == 1 then idx = 7
                elseif idx == 2 then idx = 2
                elseif idx == 3 then idx = 6
                elseif idx == 4 then idx = 1
                elseif idx == 5 then idx = 5
                elseif idx == 6 then idx = 0
                elseif idx == 7 then idx = 4
                end
                pw = pw:rotate(idx)
            else
                local val = tonumber(words[3])
                if words[2] == 'right' then val = val * -1 end
                pw = pw:rotate(val)
            end
        elseif words[1] == 'reverse' then
            pw = pw:reverse_sub(tonumber(words[3])+1, tonumber(words[5])+1)
        elseif words[1] == 'move' then
            pw = pw:move(tonumber(words[6])+1, tonumber(words[3])+1)
        end
    end
    return pw
end

print(scramble('abcdefgh'))
print ('------')
print(unscramble('fbgdceah'))