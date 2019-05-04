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
x = 0
y = 0
visits = {}

visits['00'] = 1

for dir in input:
    x, y = move(x, y, dir)
    key = f'{x}{y}'
    if key in visits:
        visits[key] += 1
    else:
        visits[key] = 1

print(len(visits))
