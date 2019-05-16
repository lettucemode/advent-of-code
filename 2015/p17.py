import aoc
import itertools

input = aoc.get_input(17)
containers = []
combinations = 0
count = 0

for line in input.splitlines():
    containers.append(int(line))

lengths = [0 for i in range(len(containers))]

for i in range(len(containers)):
    for combo in itertools.combinations(containers, i):
        if (sum(combo)) == 150:
            print(combo)
            lengths[i] += 1
            count += 1

print(count)
print(lengths)
