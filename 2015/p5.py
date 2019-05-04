# nice strings have:
# PHASE 1
# at least three vowels
# at least one letter that appears twice in a row
# does not contain ab, cd, pq, or xy
# PHASE 2
# a pair of any two letters that appears in the string at least twice
# at least one letter that repeats with exactly one letter in between

import aoc

vowels = ['a', 'e', 'i', 'o', 'u']


def hasBadText(line):
    return 'ab' in line or 'cd' in line or 'pq' in line or 'xy' in line


def hasThreeVowels(line):
    count = 0
    for char in line:
        if char in vowels:
            count += 1
    return 3 <= count


def hasDoubleLetter(line):
    lastTok = ''
    for char in line:
        if char == lastTok:
            return True
        lastTok = char
    return False


def hasRepeatingPair(line):
    for i in range(0, len(line) - 1):
        pair = line[i:i+2]
        if 1 < line.count(pair):
            return True
    return False


def hasSplitPair(line):
    lastTok = ''
    lastlastTok = ''
    for char in line:
        if char == lastlastTok:
            return True
        lastlastTok = lastTok
        lastTok = char
    return False


def isNice(line):
    # return not hasBadText(line) and hasDoubleLetter(line) and hasThreeVowels(line)
    return hasRepeatingPair(line) and hasSplitPair(line)


input = aoc.get_input(5)
nice = 0

for line in input.splitlines():
    if isNice(line):
        nice += 1

print(nice)
