import aoc


def build_aunt_sue_list(input):
    aunt_sues = []
    for line in input.splitlines():
        parts = line.split(' ')
        sue = {
            'num': int(parts[1].rstrip(':')),
            'score': 0
        }
        for i in range(2, len(parts), 2):
            sue[parts[i].rstrip(':')] = int(parts[i+1].rstrip(','))
        aunt_sues.append(sue)
    return aunt_sues


input = aoc.get_input(16)
target_aunt_sue = {
    'children': 3,
    'cats': 7,
    'samoyeds': 2,
    'pomeranians': 3,
    'akitas': 0,
    'vizslas': 0,
    'goldfish': 5,
    'trees': 3,
    'cars': 2,
    'perfumes': 1
}

aunt_sue_list = build_aunt_sue_list(input)
for aunt_sue in aunt_sue_list:
    score = 0
    for k, v in aunt_sue.items():
        if k in target_aunt_sue:
            if k == 'cats' or k == 'trees':
                if v > target_aunt_sue[k]:
                    score += 1
            elif k == 'pomeranians' or k == 'goldfish':
                if v < target_aunt_sue[k]:
                    score += 1
            elif v == target_aunt_sue[k]:
                score += 1
    aunt_sue['score'] = score

aunt_sue_list.sort(key=lambda x: x['score'], reverse=False)
for aunt_sue in aunt_sue_list:
    print(aunt_sue)
