import aoc


def next_code(code):
    return (code * 252533) % 33554393


def calc_code_order(row, col):
    order = 1 + sum(range(row))
    for i in range(col - 1):
        order += row + i + 1
    return order


input = aoc.get_input(25)
row = int(input.split(' ')[16].rstrip(','))
col = int(input.split(' ')[18].strip('.'))
code = 20151125

order = calc_code_order(row, col)
for i in range(order - 1):
    code = next_code(code)
print(code)
