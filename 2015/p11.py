import aoc

a = 97
z = 122


def ascii_increment(codes, i):
    codes[i] += 1
    if codes[i] > z:
        codes[i] = a
        ascii_increment(codes, i - 1)


def next_password(pw):
    codes = list(map(lambda x: ord(x), pw))
    ascii_increment(codes, -1)
    return ''.join(list(map(lambda x: chr(x), codes)))


def req_one


input = aoc.get_input(11)

print(input)
for i in range(51):
    input = next_password(input)
print(input)
