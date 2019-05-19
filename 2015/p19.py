import aoc


def parse_input(input):
    replacements = []
    molecule = ''
    for line in input.splitlines():
        if '=>' in line:
            parts = line.split('=>')
            replacements.append((parts[0].strip(), parts[1].strip()))
        elif len(line) > 2:
            molecule = line
    return replacements, molecule


def do_replacement(replacements, molecule):
    new_molecules = set()
    for rep in replacements:
        i = 0
        while i < len(molecule):
            index = molecule.find(rep[0], i)
            if index < 0:
                break
            new_molecules.add(
                molecule[:index] + rep[1] + molecule[index+len(rep[0]):])
            i = index + 1
    return new_molecules


def do_replacement_backwards(replacements, molecule):
    new_molecules = set()
    for rep in replacements:
        i = 0
        while i < len(molecule):
            index = molecule.find(rep[1], i)
            if index < 0:
                break
            new_molecules.add(
                molecule[:index] + rep[0] + molecule[index+len(rep[1]):])
            i = index + 1
    return new_molecules


input = aoc.get_input(19)
replacements, start = parse_input(input)

molecules_so_far = set([start])
target = 'e'
new_molecules = set()
num_reps = 0
keep_going = True

while keep_going:
    for m in molecules_so_far:
        results = do_replacement_backwards(replacements, m)
        for r in results:
            if r == target:
                keep_going = False
                break
            elif len(r) <= len(m):
                new_molecules.add(r)

    num_reps += 1
    if r == target:
        break
    molecules_so_far.clear()
    molecules_so_far = [sorted(list(new_molecules), key=len)[0]]
    new_molecules.clear()

print(num_reps)
