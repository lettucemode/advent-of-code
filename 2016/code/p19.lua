local hylib = require 'hylib'

io.input('../inputs/p19.txt')
local input = io.read('*a')

local total_elves = 50000-- tonumber(input)
local remaining_elves = total_elves
local circle = {value=1}
local n = circle
for i = 2, total_elves do 
    n.next = {value=i}
    n = n.next
end
n.next = circle
print('done with setup')

local cur_elf = circle
repeat
    -- part 1
    --cur_elf.next = cur_elf.next.next

    -- part 2    
    local distance = remaining_elves // 2
    local t_elf = cur_elf
    for i = 2, distance do t_elf = t_elf.next end
    t_elf.next = t_elf.next.next

    remaining_elves = remaining_elves - 1
    --print(remaining_elves)
    cur_elf = cur_elf.next
until cur_elf.value == cur_elf.next.value

print(cur_elf.value)