import aoc


def move(x, y, symbol):
    if symbol == '<':
        x = x - 1
    elif symbol == '^':
        y = y + 1
    elif symbol == '>':
        x = x + 1
    elif symbol == 'v':
        y = y - 1
    return x, y


input = aoc.get_input(3)
x1 = 0
y1 = 0
x2 = 0
y2 = 0
realSanta = True
visits = {}

visits['00'] = 2

for dir in input:
    if realSanta:
        x1, y1 = move(x1, y1, dir)
        key = f'{x1}{y1}'
        if key in visits:
            visits[key] += 1
        else:
            visits[key] = 1
    else:
        x2, y2 = move(x2, y2, dir)
        key = f'{x2}{y2}'
        if key in visits:
            visits[key] += 1
        else:
            visits[key] = 1
    realSanta = not realSanta

print(len(visits))
