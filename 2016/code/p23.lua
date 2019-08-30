local hylib = require 'hylib'

io.input('../inputs/p23.txt')
local input = io.read('*a')
local INC = 'inc'
local DEC = 'dec'
local CPY = 'cpy'
local JNZ = 'jnz'
local TGL = 'tgl'
local computer = {a=1,b=0,c=0,d=0}

function get_command(str)
    local _, _, command, arg1, arg2 = str:find('(%a+)%s(-?%w+)%s(-?%w+)')
    if command == nil then
        _, _, command, arg1 = str:find('(%a+)%s(-?%w+)')
    end
    return command, arg1, arg2
end

function get_value(arg)
    if arg == nil then return nil end
    if arg:find('%d+') then 
        return tonumber(arg) 
    else
        return computer[arg]
    end
end

local instructions = hylib.iter_to_table(input:splitlines())
local i = 1
while i <= #instructions do

    local command, arg1, arg2 = get_command(instructions[i])
    local jump_val = 0
    local jump_check = 0

    if command == INC then
        if computer[arg1] == nil then goto skip end
        computer[arg1] = computer[arg1] + 1
    elseif command == DEC then
        if computer[arg1] == nil then goto skip end
        computer[arg1] = computer[arg1] - 1
    elseif command == CPY then
        if computer[arg2] == nil then goto skip end
        computer[arg2] = get_value(arg1)
    elseif command == JNZ then
        jump_check = get_value(arg1)
        if jump_check ~= 0 then jump_val = get_value(arg2) end
    elseif command == TGL then
        local offset = 0
        offset = get_value(arg1)
        if i + offset < 1 or i + offset > #instructions then goto skip end
        other_command, other_arg1, other_arg2 = get_command(instructions[i+offset])
        if other_arg2 == nil then
            if other_command == INC then
                instructions[i+offset] = DEC..' '..other_arg1
            else
                instructions[i+offset] = INC..' '..other_arg1
            end
        else
            if other_command == JNZ then
                instructions[i+offset] = CPY..' '..other_arg1..' '..other_arg2
            else
                instructions[i+offset] = JNZ..' '..other_arg1..' '..other_arg2
            end
        end
    end

    ::skip::
    if jump_check ~= 0 then i = i + jump_val else i = i + 1 end
end

print(computer.a, computer.b, computer.c, computer.d)