local hylib = require 'hylib'

io.input('../inputs/p19.txt')
local input = io.read('*a')

local orig_elves = tonumber(input)
local cur_elves = orig_elves
local elves = {}
for i = 1, orig_elves do elves[i] = i end

function get_elf_around(cur_elf)
    local idx = ((cur_elf + 1) % orig_elves) + 1
    while not elves[idx] do
        idx = ((idx + 1) % orig_elves) + 1
    end
    return idx
end

function get_elf_across(cur_elf)
    local distance = cur_elves // 2
    local target_elf = cur_elf
    for i = 1, distance do target_elf = get_elf_around(target_elf) end
    return target_elf
end

local cur_elf = 1
repeat
    local target_elf = get_elf_around(cur_elf)
    elves[cur_elf] = elves[cur_elf] + elves[target_elf]
    elves[target_elf] = nil
    cur_elves = cur_elves - 1
    
    local last_elf = cur_elf
    cur_elf = get_elf_around(cur_elf)

until last_elf == cur_elf

print(cur_elf)