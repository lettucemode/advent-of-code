import aoc


def get_guests_grid(input):
    guests = []
    for line in input.splitlines():
        parts = line.split(' ')
        for part in parts:
            if part not in ('would', 'gain', 'lose', 'happiness', 'units', 'by', 'sitting', 'next', 'to') and not part.isnumeric() and not part.rstrip('.') in guests:
                guests.append(part.rstrip('.'))

    grid = [[0 for g in guests] for g in guests]
    for line in input.splitlines():
        parts = line.split(' ')
        val = int(parts[3])
        if parts[2] == 'lose':
            val *= -1
        p1i = guests.index(parts[0])
        p2i = guests.index(parts[10].rstrip('.'))
        grid[p1i][p2i] = val
    return guests, grid


def recurse(happiness, guests, grid, seating, total):
    if len(seating) == len(guests):
        firsti = guests.index(seating[0])
        lasti = guests.index(seating[-1])
        total += grid[firsti][lasti] + grid[lasti][firsti]
        print(seating, total)
        happiness.append(total)
        return

    cur_guest = seating[-1]
    i_guest = guests.index(cur_guest)
    for k in range(len(guests)):
        next_guest = guests[k]
        if next_guest != cur_guest and next_guest not in seating:
            new_total = total + grid[i_guest][k] + grid[k][i_guest]
            recurse(happiness, guests, grid, seating + [next_guest], new_total)


input = aoc.get_input(13)
guests, grid = get_guests_grid(input)

# add myself
guests.append('lettucemode')
for row in grid:
    row.append(0)
grid.append([0 for g in guests])

happiness = []
for guest in guests:
    recurse(happiness, guests, grid, [guest], 0)

print(happiness)
print(max(happiness))
