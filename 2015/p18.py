import aoc


def is_on(grid, i, j):
    if i < 0 or len(grid) <= i:
        return False
    if j < 0 or len(grid[i]) <= j:
        return False
    return grid[i][j] == '#'


def neighbors_on(grid, i, j):
    count = 0
    for k in range(i-1, i+2):
        for l in range(j-1, j+2):
            if k == i and j == l:
                continue
            if is_on(grid, k, l):
                count += 1
    return count


def step(grid):
    new_grid = [row[:] for row in grid]
    for i in range(len(grid)):
        for j in range(len(grid[i])):
            lit_neighbors = neighbors_on(grid, i, j)
            this_is_on = is_on(grid, i, j)
            if this_is_on:
                if lit_neighbors not in [2, 3]:
                    new_grid[i][j] = '.'
            else:
                if lit_neighbors == 3:
                    new_grid[i][j] = '#'

    new_grid[0][0] = '#'
    new_grid[0][-1] = '#'
    new_grid[-1][0] = '#'
    new_grid[-1][-1] = '#'
    return new_grid


input = aoc.get_input(18)
grid = [[char for char in line] for line in input.splitlines()]

for i in range(100):
    grid = step(grid)

num_on = sum(1 for row in grid for char in row if char == '#')
print(num_on)
