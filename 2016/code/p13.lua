local hylib = require 'hylib'
local bit32 = require 'bit32'
local PriorityQueue = require 'pq'

io.input('../inputs/p13.txt')
local input = io.read('*a')
local fav_number = tonumber(input)
local WALL = '#'
local SPACE = '.'

function get_coord(x, y)
    local sum = x*x + 3*x + 2*x*y + y + y*y + fav_number

    local bit_count = 0
    for i = 0, 32 do if bit32.btest(2^i, sum) then bit_count = bit_count + 1 end end

    return bit_count % 2 == 0 and SPACE or WALL
end

function heuristic(source, target)
    return math.abs(source.x - target.x) + math.abs(source.y - target.y)
end

function neighbors(node)
    local idxs = { {x=node.x-1,y=node.y}, {x=node.x+1,y=node.y},
                   {x=node.x,y=node.y-1}, {x=node.x,y=node.y+1} }
    local i = 0
    return function()
        while i <= #idxs do
            i = i + 1
            if i <= #idxs and 0 <= idxs[i].x and 0 <= idxs[i].y then return idxs[i] end
        end
    end
end

function get_key(node)
    return node.x..','..node.y
end

local target = {x=31,y=39}
local tkey = get_key(target)
local frontier = PriorityQueue()
frontier:put({x=1,y=1}, 0)
local closed_list = {}
local came_from = {}
local cost_so_far = {}
cost_so_far['1,1'] = 0

while not frontier:empty() do
    local current = frontier:pop()
    local ckey = get_key(current)
    --if current.x == target.x and current.y == target.y then break end

    for neighbor in neighbors(current) do
        local nkey = get_key(neighbor)
        if get_coord(neighbor.x, neighbor.y) == SPACE then 
            local new_cost = cost_so_far[ckey] + 1
            --if not cost_so_far[nkey] or new_cost < cost_so_far[nkey] then
            if not cost_so_far[nkey] and new_cost <= 50 then
                cost_so_far[nkey] = new_cost
                local priority = new_cost + heuristic(target, neighbor)
                frontier:put(neighbor, priority)
                came_from[nkey] = ckey
            end
        end
    end
end

--print('Path length to target --> '..cost_so_far[tkey])
print(hylib.table_length(cost_so_far))