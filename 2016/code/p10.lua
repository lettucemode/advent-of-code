local hylib = require 'hylib'

io.input('../inputs/p10.txt')
local input = io.read('*a')

function init_entry(table, no, val)
    if not table[no] then table[no] = val end
end

function give_val_to_bot(bot, val)    
    if bot.v1 then bot.v2 = val else bot.v1 = val end
end

function build_bot_network(instructions)
    local bot_network = {}
    for line in instructions:splitlines() do
        local nums = line:gmatch('%d+')
        local n1 = tonumber(nums())
        local n2 = tonumber(nums())
        if line:find('value') then
            init_entry(bot_network, n2, {})
            give_val_to_bot(bot_network[n2], n1)
        else
            local n3 = tonumber(nums())
            init_entry(bot_network, n1, {})
            for i = n2, n3, n3 - n2 do
                _, _, dest = line:find('(%a+) '..string.format('%d', i))
                if i == n2 then
                    bot_network[n1].low_type = dest
                    bot_network[n1].low_targ = i
                else
                    bot_network[n1].high_type = dest
                    bot_network[n1].high_targ = i
                end
            end
        end
    end
    return bot_network
end

local data = { output = {}, bot = build_bot_network(input) }
while true do
    local num_continued = 0
    for k, b in pairs(data.bot) do
        if b.done then 
            num_continued = num_continued + 1
            goto continue 
        end

        if b.v1 and b.v2 then
            local low_val = math.min(b.v1, b.v2)
            local high_val = math.max(b.v1, b.v2)
            if b.low_type == 'output' then
                data[b.low_type][b.low_targ] = low_val
            else
                give_val_to_bot(data[b.low_type][b.low_targ], low_val)
            end
            if b.high_type == 'output' then
                data[b.high_type][b.high_targ] = high_val
            else
                give_val_to_bot(data[b.high_type][b.high_targ], high_val)
            end
            b.done = 1
        end

        --part 1 solution
        if b.v1 == 61 and b.v2 == 17 then
            print(string.format("THE WINNING BOT IS %d", k))
        end

        ::continue::
    end

    if num_continued == #data.bot then break end
end

print(data.output[0] * data.output[1] * data.output[2])