local hylib = require 'hylib'
local table = require 'table'

io.input('../inputs/p4.txt')
local input = io.read('*a')

function get_letter_counts(str)
    t = {}
    for c in str:gmatch('.') do
        if t[c] then
            t[c] = t[c] + 1
        else
            t[c] = 1
        end
    end
    return t
end

local sector_id_sum = 0
local real_names = {}
for line in input:splitlines() do
    local _, _, name = select(1, line:gsub('%-', '')):find('(%a+)')
    local sector_id = tonumber(line:match('%d+'))
    local _, _, checksum = line:find('%[(%a+)%]')

    local all_letter_counts = get_letter_counts(name)
    local vals = {}
    for c in checksum:gmatch('.') do
       vals[#vals + 1] = select(2, name:gsub(c, c))
    end

    for i = 1, 4 do
        if vals[i] < vals[i+1] or 
            (vals[i] == vals[i+1] and checksum:sub(i,i) >= checksum:sub(i+1,i+1)) or
            all_letter_counts[checksum:sub(i,i)] ~= vals[i] or 
            all_letter_counts[checksum:sub(i+1,i+1)] ~= vals[i+1] then
            goto continue
        end
    end

    sector_id_sum = sector_id_sum + sector_id
    real_names[name] = sector_id
    ::continue::
end

print(sector_id_sum)

-- byte vals for a-z are 97-122
local northpole_obj_storage_sector_id = 0
for k, v in pairs(real_names) do
    local rotate_val = v % 26
    local decoded_str = ''
    for c in k:gmatch('.') do
        local byte_val = c:byte() + rotate_val
        if byte_val > 122 then
            byte_val = byte_val - 26
        end
        decoded_str = decoded_str..string.char(byte_val) 
    end
    --print(decoded_str, v)
    if decoded_str:find('northpole') then
        --print('^^^  THIS ONE  ^^^')
        northpole_obj_storage_sector_id = v
    end
end
print(northpole_obj_storage_sector_id)