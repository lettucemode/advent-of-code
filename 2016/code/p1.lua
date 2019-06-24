io.input('../inputs/p1.txt')
input = io.read('*a')

direction = 0
x = 0
y = 0
path = {}

for c in input:gmatch('%w+') do
    if c:sub(1, 1) == 'R' then
        direction = (direction + 1) % 4
    elseif c:sub(1, 1) == 'L' then
        direction = (direction + 3) % 4
    end

    val = tonumber(c:sub(2))
    for i = 1, val do
        if direction == 0 then
            y = y + 1
        elseif direction == 1 then
            x = x + 1
        elseif direction == 2 then
            y = y - 1
        elseif direction == 3 then
            x = x - 1
        end
        path[x..y] = path[x..y] == nil and 1 or path[x..y] + 1
        
        -- part 2
        if path[x..y] == 2 then goto DONE end
    end
end

::DONE::
print(x, y)
print(math.abs(x) + math.abs(y))