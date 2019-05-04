import aoc

input = aoc.get_input(7)
network = {}


def getNetworkVal(key):
    if key.isnumeric():
        return int(key)
    if key in network:
        return int(network[key])
    return None


def getOperation(instruction):
    instruct = str(instruction).split('->')[0].strip()
    parts = instruct.split(' ')

    if len(parts) == 1:
        return ('WIRE', getNetworkVal(parts[0]))

    if len(parts) == 2:
        return (parts[0], getNetworkVal(parts[1]))

    if len(parts) == 3:
        return (parts[1], getNetworkVal(parts[0]), getNetworkVal(parts[2]))


def getOutput(instruction):
    return str(instruction).split('->')[1].strip()


def doOperation(operands):
    type = operands[0]

    if type == 'WIRE':
        return operands[1]

    if type == 'NOT':
        if operands[1] is None:
            return None
        return ~operands[1]

    if type == 'AND':
        if operands[1] is None or operands[2] is None:
            return None
        return operands[1] & operands[2]

    if type == 'OR':
        if operands[1] is None or operands[2] is None:
            return None
        return operands[1] | operands[2]

    if type == 'LSHIFT':
        if operands[1] is None or operands[2] is None:
            return None
        return operands[1] << operands[2]

    if type == 'RSHIFT':
        if operands[1] is None or operands[2] is None:
            return None
        return operands[1] >> operands[2]


prePassNetworkSize = -1
iters = 0
while prePassNetworkSize != len(network):
    prePassNetworkSize = len(network)
    iters += 1

    for line in input.splitlines():
        op = getOperation(line)
        result = doOperation(op)
        if result is not None:
            network[getOutput(line)] = result

print(len(input), iters, network['a'])

# part 2
a = network['a']
network = {}
network['b'] = a

prePassNetworkSize = -1
iters = 0
while prePassNetworkSize != len(network):
    prePassNetworkSize = len(network)
    iters += 1

    for line in input.splitlines():
        output = getOutput(line)
        if output == 'b':
            continue

        op = getOperation(line)
        result = doOperation(op)
        if result is not None:
            network[output] = result

print(len(input), iters, network['a'])

# todo: do this with classes instead
