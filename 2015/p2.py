import aoc


def calcTotalSurfaceArea(l, w, h):
    return 2*l*w + 2*w*h + 2*l*h


def calcSlack(l, w, h):
    return min(l*w, w*h, l*h)


def getDimensions(s):
    pieces = str(s).split('x')
    l = int(pieces[0])
    w = int(pieces[1])
    h = int(pieces[2])
    return l, w, h


def calcBow(l, w, h):
    return l*w*h


def calcRibbonWrap(l, w, h):
    return min(2*l + 2*w, 2*w + 2*h, 2*l + 2*h)


input = aoc.getInput(2)
sqFeet = 0
ribbon = 0

for line in str(input).splitlines():
    l, w, h = getDimensions(line)

    surfaceArea = calcTotalSurfaceArea(l, w, h)
    sqFeet = sqFeet + surfaceArea

    slack = calcSlack(l, w, h)
    sqFeet = sqFeet + slack

    bowLength = calcBow(l, w, h)
    ribbon = ribbon + bowLength

    wrapLength = calcRibbonWrap(l, w, h)
    ribbon = ribbon + wrapLength

print(sqFeet, ribbon)
