local hylib = require 'hylib'
local PriorityQueue = require 'pq'

io.input('../inputs/p24.txt')
local input = io.read('*a')
local grid = {}
local locations = {}
local WALL = '#'
local OPEN = '.'

-- setup
for line in input:splitlines() do
    grid[#grid+1] = hylib.iter_to_table(line:gmatch('[%.#%d]'))
end
for i,row in ipairs(grid) do
    for k,val in ipairs(row) do
        if val:find('%d') then locations[tonumber(val)] = {x=i,y=k} end
    end
end
print('setup done')

function get_shortest_path_length(origin, destination)

    function heuristic(o, d)
        return math.abs(o.x - d.x) + math.abs(o.y - d.y)
    end

    function neighbors(coord)
        local idxs = { {x=coord.x-1,y=coord.y}, {x=coord.x+1,y=coord.y},
                    {x=coord.x,y=coord.y-1}, {x=coord.x,y=coord.y+1} }
        local i = 0
        return function()
            while i < #idxs do
                i = i + 1
                if grid[idxs[i].x][idxs[i].y]:find('[%d%.]') then 
                    return idxs[i] 
                end
            end
        end
    end

    function get_key(coord)
        return coord.x..','..coord.y
    end

    local okey = get_key(origin)
    local dkey = get_key(destination)
    local frontier = PriorityQueue()
    frontier:put(origin, 0)
    local came_from = {}
    local cost_so_far = {}
    cost_so_far[okey] = 0

    while not frontier:empty() do
        local current = frontier:pop()
        local ckey = get_key(current)

        for neighbor in neighbors(current) do
            local nkey = get_key(neighbor)
            local new_cost = cost_so_far[ckey] + 1                
            if not cost_so_far[nkey] then
                cost_so_far[nkey] = new_cost
                local priority = new_cost + heuristic(destination, neighbor)
                frontier:put(neighbor, priority)
                came_from[nkey] = ckey
            end
        end
        if cost_so_far[dkey] then break end
    end

    return cost_so_far[dkey]
end

local cur = locations[0]
locations[0].dist = 0
locations[3].dist = 0
repeat
    local min_length = 9001
    local min_idx = 9001
    for test, i in hylib.where(locations, function(x) return x.dist == nil end) do
        local length = get_shortest_path_length(cur, test)
        if length < min_length then
            min_length = length
            min_idx = i
        end
    end
    locations[min_idx].dist = min_length
    cur = locations[min_idx]
    --print(min_idx, min_length)
until not hylib.any(locations, function(x) return x.dist == nil end)

local total = 0
for i,loc in ipairs(locations) do total = total + loc.dist end
-- part 2, cheated a bit here by forcing the algo to use 3 last
total = total + get_shortest_path_length(cur, locations[3])
total = total + get_shortest_path_length(locations[3], locations[0])
print(total)

-- print(
--     get_shortest_path_length(locations[0], locations[0]),
--     get_shortest_path_length(locations[0], locations[1]),
--     get_shortest_path_length(locations[0], locations[2]),
--     get_shortest_path_length(locations[0], locations[3]),
--     get_shortest_path_length(locations[0], locations[4]),
--     get_shortest_path_length(locations[0], locations[5]),
--     get_shortest_path_length(locations[0], locations[6]),
--     get_shortest_path_length(locations[0], locations[7])
-- )
-- print(
--     get_shortest_path_length(locations[1], locations[0]),
--     get_shortest_path_length(locations[1], locations[1]),
--     get_shortest_path_length(locations[1], locations[2]),
--     get_shortest_path_length(locations[1], locations[3]),
--     get_shortest_path_length(locations[1], locations[4]),
--     get_shortest_path_length(locations[1], locations[5]),
--     get_shortest_path_length(locations[1], locations[6]),
--     get_shortest_path_length(locations[1], locations[7])
-- )
-- print(
--     get_shortest_path_length(locations[2], locations[0]),
--     get_shortest_path_length(locations[2], locations[1]),
--     get_shortest_path_length(locations[2], locations[2]),
--     get_shortest_path_length(locations[2], locations[3]),
--     get_shortest_path_length(locations[2], locations[4]),
--     get_shortest_path_length(locations[2], locations[5]),
--     get_shortest_path_length(locations[2], locations[6]),
--     get_shortest_path_length(locations[2], locations[7])
-- )
-- print(
--     get_shortest_path_length(locations[3], locations[0]),
--     get_shortest_path_length(locations[3], locations[1]),
--     get_shortest_path_length(locations[3], locations[2]),
--     get_shortest_path_length(locations[3], locations[3]),
--     get_shortest_path_length(locations[3], locations[4]),
--     get_shortest_path_length(locations[3], locations[5]),
--     get_shortest_path_length(locations[3], locations[6]),
--     get_shortest_path_length(locations[3], locations[7])
-- )
-- print(
--     get_shortest_path_length(locations[4], locations[0]),
--     get_shortest_path_length(locations[4], locations[1]),
--     get_shortest_path_length(locations[4], locations[2]),
--     get_shortest_path_length(locations[4], locations[3]),
--     get_shortest_path_length(locations[4], locations[4]),
--     get_shortest_path_length(locations[4], locations[5]),
--     get_shortest_path_length(locations[4], locations[6]),
--     get_shortest_path_length(locations[4], locations[7])
-- )
-- print(
--     get_shortest_path_length(locations[5], locations[0]),
--     get_shortest_path_length(locations[5], locations[1]),
--     get_shortest_path_length(locations[5], locations[2]),
--     get_shortest_path_length(locations[5], locations[3]),
--     get_shortest_path_length(locations[5], locations[4]),
--     get_shortest_path_length(locations[5], locations[5]),
--     get_shortest_path_length(locations[5], locations[6]),
--     get_shortest_path_length(locations[5], locations[7])
-- )
-- print(
--     get_shortest_path_length(locations[6], locations[0]),
--     get_shortest_path_length(locations[6], locations[1]),
--     get_shortest_path_length(locations[6], locations[2]),
--     get_shortest_path_length(locations[6], locations[3]),
--     get_shortest_path_length(locations[6], locations[4]),
--     get_shortest_path_length(locations[6], locations[5]),
--     get_shortest_path_length(locations[6], locations[6]),
--     get_shortest_path_length(locations[6], locations[7])
-- )
-- print(
--     get_shortest_path_length(locations[7], locations[0]),
--     get_shortest_path_length(locations[7], locations[1]),
--     get_shortest_path_length(locations[7], locations[2]),
--     get_shortest_path_length(locations[7], locations[3]),
--     get_shortest_path_length(locations[7], locations[4]),
--     get_shortest_path_length(locations[7], locations[5]),
--     get_shortest_path_length(locations[7], locations[6]),
--     get_shortest_path_length(locations[7], locations[7])
-- )