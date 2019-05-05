import aoc


def parse_ingredients(input):
    ingredients = {}
    for line in input.splitlines():
        parts = line.split(' ')
        name = parts[0].rstrip(':')
        capacity = int(parts[2].rstrip(','))
        durability = int(parts[4].rstrip(','))
        flavor = int(parts[6].rstrip(','))
        texture = int(parts[8].rstrip(','))
        calories = int(parts[10].rstrip(','))
        ingredients[name] = {'capacity': capacity, 'durability': durability,
                             'flavor': flavor, 'texture': texture, 'calories': calories}
    return ingredients


def sums_to_100():
    solutions = []
    for i in range(1, 98):
        for j in range(1, 98):
            for k in range(1, 98):
                for l in range(1, 98):
                    if i + j + k + l == 100:
                        solutions.append([i, j, k, l])
    return solutions


input = aoc.get_input(15)
ingredients = parse_ingredients(input)
sums = sums_to_100()

results = []
for sum in sums:
    capacity = 0
    durability = 0
    flavor = 0
    texture = 0
    calories = 0

    ings = list(ingredients.values())
    for i in range(len(sum)):
        capacity += sum[i] * ings[i]['capacity']
        durability += sum[i] * ings[i]['durability']
        flavor += sum[i] * ings[i]['flavor']
        texture += sum[i] * ings[i]['texture']
        calories += sum[i] * ings[i]['calories']

    capacity = max(0, capacity)
    durability = max(0, durability)
    flavor = max(0, flavor)
    texture = max(0, texture)
    total_score = capacity * durability * flavor * texture
    if total_score > 0:
        results.append((total_score, calories))

scores = list(map(lambda x: x[0], results))
calories_filter = list(map(lambda x: x[0], list(
    filter(lambda x: x[1] == 500, results))))
print(max(scores))
print(max(calories_filter))
