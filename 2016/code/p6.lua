local hylib = require 'hylib'

io.input('../inputs/p6.txt')
local input = io.read('*a')

local data = {}
local newline_pos = input:find('\n')
for line in input:splitlines() do
    for i = 1, newline_pos - 1 do
        if not data[i] then
            data[i] = {}
        end
        local c = line:sub(i,i)
        if data[i][c] then
            data[i][c] = data[i][c] + 1
        else
            data[i][c] = 1
        end
    end
end

result = ''
for _, counts in ipairs(data) do
    -- local kmax, vmax = '', 0
    local kmin, vmin = '', 1000000000
    for k, v in pairs(counts) do
        -- if v > vmax then
        --     kmax = k
        --     vmax = v
        -- end
        if v < vmin then
            kmin = k
            vmin = v
        end
    end
    -- result = result..kmax
    result = result..kmin
end

print(result)