local hylib = require 'hylib'

io.input('../inputs/p19.txt')
local input = io.read('*a')

local total_elves = tonumber(input)
local remaining_elves = total_elves
local circle = {}
for i=0,total_elves-1 do 
    circle[i] = {value=i+1}
end
for i=0,total_elves-1 do 
    circle[i].nxt = circle[(i+1)%total_elves]
    circle[i].prv = circle[(i-1)%total_elves]
end
local cur_elf = circle[0]
local mid_elf = circle[total_elves//2]
print('done with setup')

function remove(elf)
    elf.prv.nxt = elf.nxt
    elf.nxt.prv = elf.prv
end

repeat
    -- part 1
    -- remove(cur_elf.nxt)

    -- part 2
    remove(mid_elf)
    mid_elf = mid_elf.nxt
    remaining_elves = remaining_elves - 1
    if remaining_elves % 2 == 0 then mid_elf = mid_elf.nxt end

    cur_elf = cur_elf.nxt
until cur_elf.value == cur_elf.nxt.value

print(cur_elf.value)