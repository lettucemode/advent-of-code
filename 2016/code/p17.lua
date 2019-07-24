local hylib = require 'hylib'
local md5 = require 'md5'

io.input('../inputs/p17.txt')
local input = io.read('*a')
local target = {x=4,y=4}

function get_location(path)
    path = path:sub(#input+1)
    local x = 1
    local y = 1
    x = x + select(2, path:gsub('R', 'R'))
    x = x - select(2, path:gsub('L', 'L'))
    y = y + select(2, path:gsub('D', 'D'))
    y = y - select(2, path:gsub('U', 'U'))
    return {x=x,y=y}
end

function legal_moves(path, loc)
    if path == nil then return {} end
    local loc = get_location(path)
    local r = {}
    local hash = md5.sumhexa(path)
    if 1 < loc.y and hash:sub(1,1):find('[b-f]') then r[#r+1] = path..'U' end
    if loc.y < 4 and hash:sub(2,2):find('[b-f]') then r[#r+1] = path..'D' end
    if 1 < loc.x and hash:sub(3,3):find('[b-f]') then r[#r+1] = path..'L' end
    if loc.x < 4 and hash:sub(4,4):find('[b-f]') then r[#r+1] = path..'R' end
    return r
end

local paths = legal_moves(input)
local solved_paths = {}
while 0 < #paths do
    local next_paths = {}
    for i, path in ipairs(paths) do
        for k, new_path in pairs(legal_moves(path)) do
            local location = get_location(new_path)
            if location.x == target.x and location.y == target.y then
                solved_paths[#solved_paths+1] = new_path
            else
                next_paths[#next_paths+1] = new_path
            end
        end
    end
    paths = next_paths
end

print(solved_paths[1]:sub(#input+1),
      solved_paths[#solved_paths]:sub(#input+1):len())