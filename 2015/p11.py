import aoc

a = 97
z = 122


def ascii_increment(codes, i):
    codes[i] += 1
    if codes[i] > z:
        codes[i] = a
        ascii_increment(codes, i - 1)


def next_password(password):
    codes = list(map(lambda x: ord(x), password))
    ascii_increment(codes, -1)
    return ''.join(list(map(lambda x: chr(x), codes)))


def req_one(password):
    for i in range(len(password) - 2):
        if ord(password[i]) + 1 == ord(password[i+1]) and ord(password[i+1]) + 1 == ord(password[i+2]):
            return True
    return False


def req_two(password):
    return 'i' not in password and 'o' not in password and 'l' not in password


def req_three(password):
    firstFind = ''
    for i in range(len(password) - 1):
        if password[i] == password[i+1]:
            if firstFind == '':
                firstFind = password[i]
            elif firstFind != password[i]:
                return True
    return False


input = aoc.get_input(11)

print(input)
while not req_one(input) or not req_two(input) or not req_three(input):
    input = next_password(input)
print(input)

input = next_password(input)

while not req_one(input) or not req_two(input) or not req_three(input):
    input = next_password(input)
print(input)
