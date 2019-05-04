import aoc

input = aoc.get_input(1)
floor = 0
firstBasementPosition = 0

for i in range(0, len(input)):
    if input[i] == '(':
        floor = floor + 1
    elif input[i] == ')':
        floor = floor - 1
    if floor < 0 and firstBasementPosition == 0:
        firstBasementPosition = i + 1

print(floor, firstBasementPosition)
