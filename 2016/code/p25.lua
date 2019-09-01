local hylib = require 'hylib'

io.input('../inputs/p25.txt')
local input = io.read('*a')
local INC = 'inc'
local DEC = 'dec'
local CPY = 'cpy'
local JNZ = 'jnz'
local TGL = 'tgl'
local OUT = 'out'

function simulate_clock_signal(val_for_a)

    local computer = {a=val_for_a,b=0,c=0,d=0}
    local past_output = {}

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

    function check_output(new_output_val)
        past_output[#past_output+1] = new_output_val
        local prev_val = nil
        for i,v in ipairs(past_output) do
            if prev_val == nil then 
                prev_val = v
            elseif prev_val ~= v then
                prev_val = v
            elseif prev_val == v then
                return false
            end
        end
        return true
    end

    local instructions = hylib.iter_to_table(input:splitlines())
    local i = 1
    local out_counter = 0
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
        elseif command == OUT then
            out_counter = out_counter + 1
            local passing = check_output(get_value(arg1))
            if not passing or out_counter > 50 then return passing end
        end

        ::skip::
        if jump_check ~= 0 then i = i + jump_val else i = i + 1 end
    end

end

for i=1,10000 do
    local found_it = simulate_clock_signal(i)
    if found_it then print(i) break 
    else print(i, found_it) end
end