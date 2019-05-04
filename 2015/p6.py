import aoc

input = aoc.get_input(6)
grid = [[0 for i in range(1000)] for i in range(1000)]


def turnOn(light):
    # first is part 1, second is part 2
    # return 1
    return light + 1


def turnOff(light):
    # return 0
    return max(0, light - 1)


def toggle(light):
    # return (light + 1) % 2
    return light + 2


def getChangeFunc(line):
    if 'turn on' in line:
        return turnOn
    if 'turn off' in line:
        return turnOff
    if 'toggle' in line:
        return toggle
    return lambda x: x


def getCoordinates(line):
    linne = line.replace('turn on', '').replace(
        'turn off', '').replace('toggle', '')
    parts = linne.split(' ')
    start = parts[1].split(',')
    end = parts[3].split(',')
    return (int(start[0]), int(start[1]), int(end[0]), int(end[1]))


for line in input.splitlines():
    startx, starty, endx, endy = getCoordinates(line)
    func = getChangeFunc(line)
    for i in range(startx, endx + 1):
        for k in range(starty, endy + 1):
            grid[i][k] = func(grid[i][k])

total = sum(map(sum, grid))
print(total)
