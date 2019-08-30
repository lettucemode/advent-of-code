local hylib = require 'hylib'

io.input('../inputs/p22.txt')
local input = io.read('*a')

local nodes = {}
for line in input:splitlines() do
    if line:find('dev') then
        local _, _, x, y, size, used, avail = 
            line:find('x(%d+)-y(%d+)%s+(%d+)T%s+(%d+)T%s+(%d+)T%s+')
            local n = {x=tonumber(x),y=tonumber(y),
                size=tonumber(size),used=tonumber(used),avail=tonumber(avail)}
            nodes[#nodes+1] = n
    end
end

local viable_pairs = {}
for i, na in ipairs(nodes) do
    if na.used == 0 then goto continue_a end
    for k, nb in ipairs(nodes) do
        if i == k then goto continue_b end
        if na.used <= nb.avail then viable_pairs[#viable_pairs+1] = {a=i,b=k} end
        ::continue_b::
    end
    ::continue_a::
end

print(#viable_pairs)

local str = ''
local currx = 0
for i, n in ipairs(nodes) do
    if n.x ~= currx then 
        currx = n.x
        str = str..'\n'
    end
    if n.x == 31 and n.y == 0 then
        str = str..'G'
    elseif n.x == 0 and n.y == 0 then
        str = str..'H'
    elseif n.used > 94 then 
        str = str..'#' 
    elseif n.used == 0 then
        str = str..'_'
    else
        str = str..'.' 
    end
end
print(str)

-- by manual counting, answer is 195