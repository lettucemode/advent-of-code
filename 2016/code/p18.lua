local hylib = require 'hylib'

io.input('../inputs/p18.txt')
local SAFE = '.'
local TRAP = '^'
local input = io.read('*a')
local width = input:len()
local num_rows = 400000
local tiles = {{}}
for i = 1, width do tiles[1][i] = input:sub(i,i) end

function get_tile(row, col)
    local left = (col == 1) and SAFE or tiles[row-1][col-1]
    local center = tiles[row-1][col]
    local right = (col == width) and SAFE or tiles[row-1][col+1]

    if (left == TRAP and center == TRAP and right == SAFE) or
        (left == SAFE and center == TRAP and right == TRAP) or
        (left == TRAP and center == SAFE and right == SAFE) or
        (left == SAFE and center == SAFE and right == TRAP)
    then return TRAP else return SAFE end
end

local safe_count = select(2, input:gsub('%.', '.'))
for r = 2, num_rows do
    tiles[r] = {}
    for c = 1, width do
        tiles[r][c] = get_tile(r, c)
        if tiles[r][c] == SAFE then safe_count = safe_count + 1 end
    end
    --print(table.concat(tiles[r]))
end

print(safe_count)