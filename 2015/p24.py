import aoc
import itertools
from functools import reduce

input = aoc.get_input(24)
weights = [int(line) for line in input.splitlines()]
target_weight = sum(weights) / 4
min_length_found = 100
quantum_entanglement = 999999999999999999999
done = False

for i in range(len(weights)):
    for g1 in itertools.combinations(weights, i):
        if sum(g1) == target_weight:
            done = True
            if len(g1) < min_length_found:
                min_length_found = len(g1)
                quantum_entanglement = reduce(lambda x, y: x * y, g1)
            elif len(g1) == min_length_found:
                quantum_entanglement = min(
                    quantum_entanglement, reduce(lambda x, y: x * y, g1))
    if done:
        break

# for i in range(len(weights)):
#     for g1 in itertools.combinations(weights, i):
#         if sum(g1) == target_weight:
#             remaining_weights = [
#                 weight for weight in weights if weight not in g1]
#             for j in range(len(remaining_weights)):
#                 for g2 in itertools.combinations(remaining_weights, j):
#                     if sum(g2) == target_weight:
#                         g3 = [
#                             weight for weight in remaining_weights if weight not in g2]
#                         if sum(g3) == target_weight:
#                             for g in [g1, g2, g3]:
#                                 if len(g) < min_length_found:
#                                     min_length_found = len(g)
#                                     quantum_entanglement = reduce(
#                                         lambda x, y: x * y, g)
#                                 elif len(g) == min_length_found:
#                                     quantum_entanglement = min(
#                                         quantum_entanglement, reduce(lambda x, y: x * y, g))

print(min_length_found, quantum_entanglement)
