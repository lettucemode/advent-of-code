local hylib = require 'hylib'

io.input('../inputs/p12.txt')
local input = io.read('*a')
local INC = 'inc'
local DEC = 'dec'
local CPY = 'cpy'
local JNZ = 'jnz'

local computer = {a=0,b=0,c=0,d=0}
local instructions = hylib.iter_to_table(input:splitlines())
local i = 1
while i <= #instructions do

    local line = instructions[i]
    local _, _, command, arg1, arg2 = line:find('(%a+)%s(-?%w+)%s(-?%w+)')
    if command == nil then
        _, _, command, arg1 = line:find('(%a+)%s(-?%w+)')
    end

    if command == INC then
        computer[arg1] = computer[arg1] + 1
        i = i + 1
    elseif command == DEC then
        computer[arg1] = computer[arg1] - 1
        i = i + 1
    elseif command == CPY then
        if arg1:find('%d+') then
            computer[arg2] = tonumber(arg1)
        else
            computer[arg2] = computer[arg1]
        end
        i = i + 1
    elseif command == JNZ then
        local jump_check = 0
        if arg1:find('%d+') then
            jump_check = tonumber(arg1)
        else
            jump_check = computer[arg1]
        end
        if jump_check ~= 0 then
            i = i + tonumber(arg2)
        else
            i = i + 1
        end
    end
end

print(computer.a, computer.b, computer.c, computer.d)