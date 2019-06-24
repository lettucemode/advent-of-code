local hylib = require 'hylib'

io.input('../inputs/p3.txt')
input = io.read('*a')
data = {}

function is_triangle(n1, n2, n3)
    return n1 + n2 > n3 and n1 + n3 > n2 and n2 + n3 > n1
end

count_1 = 0
for line in input:splitlines() do
    n1, n2, n3 = unpack(
        hylib.iter_to_table(line:gmatch('%d+'), tonumber)
    )
    if is_triangle(n1, n2, n3) then
        count_1 = count_1 + 1
    end
    data[#data + 1] = {n1, n2, n3}
end

count_2 = 0
for i = 1, #data, 3 do
    for j = 1, 3 do
        if is_triangle(data[i][j], data[i+1][j], data[i+2][j]) then
            count_2 = count_2 + 1
        end
    end
end

print(count_1, count_2)