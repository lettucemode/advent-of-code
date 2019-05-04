import aoc

input = aoc.get_input(8)


def getMemoryLength(line):
    newline = line.encode("utf-8").decode("unicode-escape")
    return len(newline) - 2


def getEncodedLength(line):
    newline = '"' + line.replace('\\', '\\\\').replace('"', '\\"') + '"'
    print(line, newline, len(line), len(newline))
    return len(newline)


total = 0
for line in input.splitlines():
    #total += len(line) - getMemoryLength(line)
    total += getEncodedLength(line) - len(line)

print(total)
