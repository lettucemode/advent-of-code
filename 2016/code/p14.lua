local hylib = require 'hylib'
local md5 = require 'md5'

io.input('../inputs/p14.txt')
local input = io.read('*a')

local memo_hashes = {}
function get_hash(key)
    if memo_hashes[key] then return memo_hashes[key] end
    local h = key
    for i = 1, 2017 do h = md5.sumhexa(h) end
    memo_hashes[key] = h
    return h
end

function has_seq(s, l)
    local last_c = ''
    local streak = 1
    for i = 1, #s do
        local c = s:sub(i,i)
        if c == last_c then
            streak = streak + 1
            if l <= streak then return c end
        else
            last_c = c
            streak = 1
        end
    end
    return nil
end

local salt = input
local keys = {}
local idx = -1
while #keys < 64 do

    idx = idx + 1
    print(idx)
    local hash = get_hash(salt..idx)

    local ch = has_seq(hash, 3)
    if ch then
        for j = idx+1, idx+1000 do
            print('j --> '..j)
            local future_hash = get_hash(salt..j)
            if future_hash:find(ch..ch..ch..ch..ch) then
                keys[#keys+1] = {idx=idx,hash=hash,j=j,fu=future_hash}
                break
            end
        end
    end
end

for i, v in ipairs(keys) do print(i, v.idx, v.hash, v.j, v.fu) end
