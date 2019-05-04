import aoc


def getNodes(data):
    nlist = []
    for line in data.splitlines():
        parts = line.split(' ')
        for p in parts:
            if p != 'to' and p != '=' and not p.isnumeric():
                if not p in nlist:
                    nlist.append(p)
    return nlist


def buildGraph(input, nodes):
    graph = [[0 for x in range(len(nodes))] for y in range(len(nodes))]
    for line in input.splitlines():
        parts = line.split(' ')
        loc1 = parts[0]
        loc2 = parts[2]
        weight = parts[4]
        i = nodes.index(loc1)
        k = nodes.index(loc2)
        graph[i][k] = int(weight)
        graph[k][i] = int(weight)
    return graph


def recurse(pathTotals, graph, nodes, curPath, curTotal):
    if len(curPath) == len(nodes):
        pathTotals.append(curTotal)
        return

    index = nodes.index(curPath[-1])
    for i in range(len(graph[index])):
        if graph[index][i] > 0 and nodes[i] not in curPath:
            recurse(pathTotals, graph, nodes, curPath +
                    [nodes[i]], curTotal + graph[index][i])


input = aoc.getInput(9)
nodes = getNodes(input)
graph = buildGraph(input, nodes)

pathTotals = []
for i in range(len(nodes)):
    recurse(pathTotals, graph, nodes, [nodes[i]], 0)

print(min(pathTotals), max(pathTotals))
