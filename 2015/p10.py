import aoc


def iterate(prev):
    next = ''
    lastCh = ''
    for ch in prev:
        if lastCh == '':
            count = 1
        elif ch == lastCh:
            count += 1
        elif ch != lastCh:
            next += str(count)
            next += lastCh
            count = 1
        lastCh = ch

    next += str(count)
    next += lastCh
    return next


input = aoc.get_input(10)
for i in range(50):
    input = iterate(input)

print(len(input))
