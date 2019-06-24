local hylib = require 'hylib'
local math = require 'math'

io.input('../inputs/p2.txt')
input = io.read('*a')

-- keypads
keypad_1 = {}
for i = 1, 3 do
    keypad_1[i] = {}
    for j = 1, 3 do
        keypad_1[i][j] = j + 3 * (i - 1)
    end
end

keypad_2 = {}
for i = 1, 5 do
    if i == 1 then
        keypad_2[i] = {'', '', 1, '', ''}
    elseif i == 2 then
        keypad_2[i] = {'', 2, 3, 4, ''}
    elseif i == 3 then
        keypad_2[i] = {5, 6, 7, 8, 9}
    elseif i == 4 then
        keypad_2[i] = {'', 'A', 'B', 'C', ''}
    elseif i == 5 then
        keypad_2[i] = {'', '', 'D', '', ''}
    end
end

-- keypad traversal methods
function decode_number_1(code)
    for c in code:gmatch('%a') do
        if c == 'U' then
            current_y = math.max(1, current_y - 1)
        elseif c == 'R' then
            current_x = math.min(3, current_x + 1)
        elseif c == 'D' then
            current_y = math.min(3, current_y + 1)
        elseif c == 'L' then
            current_x = math.max(1, current_x - 1)
        end
    end
    return keypad_1[current_y][current_x]
end
function decode_number_2(code)
    for c in code:gmatch('%a') do
        if c == 'U' then
            next_y = math.max(1, current_y - 1)
            if keypad_2[next_y][current_x] ~= '' then
                current_y = next_y
            end
        elseif c == 'R' then
            next_x = math.min(5, current_x + 1)
            if keypad_2[current_y][next_x] ~= '' then
                current_x = next_x
            end
        elseif c == 'D' then
            next_y = math.min(5, current_y + 1)
            if keypad_2[next_y][current_x] ~= '' then
                current_y = next_y
            end
        elseif c == 'L' then
            next_x = math.max(1, current_x - 1)
            if keypad_2[current_y][next_x] ~= '' then
                current_x = next_x
            end
        end
        --print(string.format('--%s', keypad_2[current_y][current_x]))
    end
    return keypad_2[current_y][current_x]
end

--main loop
current_x = 1
current_y = 3
for line in input:splitlines() do
    number = decode_number_2(line)
    print(number)
end