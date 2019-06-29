local hylib = require 'hylib'

io.input('../inputs/p8.txt')
local input = io.read('*a')

function create_grid()
    g = {}
    for i = 1, 6 do
        g[i] = {}
        for j = 1, 50 do
            g[i][j] = 0        
        end
    end
    return g
end

function rotate_row(row, offset, old_grid)
    new_grid = create_grid()
    for r = 1, #grid do
        for c = 1, #grid[r] do
            if r == row then
                src = c - (offset % 50)
                if src < 1 then src = src + 50 end
                new_grid[r][c] = old_grid[r][src]
            else
                new_grid[r][c] = old_grid[r][c]
            end
        end
    end
    return new_grid
end

function rotate_col(col, offset, old_grid)
    new_grid = create_grid()
    for r = 1, #grid do
        for c = 1, #grid[r] do
            if c == col then
                src = r - (offset % 6)
                if src < 1 then src = src + 6 end
                new_grid[r][c] = old_grid[src][c]
            else
                new_grid[r][c] = old_grid[r][c]
            end
        end
    end
    return new_grid
end

grid = create_grid()
for line in input:splitlines() do
    local vals = line:gmatch('%d+')
    local n1 = vals()
    local n2 = vals()
    if line:find('rect') then
        for r = 1, n1 do
            for c = 1, n2 do
                grid[c][r] = 1
            end
        end
    elseif line:find('row y') then
        grid = rotate_row(n1 + 1, n2, grid)
    elseif line:find('column x') then
        grid = rotate_col(n1 + 1, n2, grid)
    end
end

count = 0
for i = 1, #grid do
    str = ''
    for j = 1, #grid[i] do
        str = str..(grid[i][j] == 1 and '1' or ' ')
        if grid[i][j] == 1 then count = count + 1 end
    end
    print(str)
end
print(count)