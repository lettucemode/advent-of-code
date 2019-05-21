import aoc
import math

input = aoc.get_input(20)


def get_divisors(n):
    i = 1
    while i <= math.sqrt(n):
        if n % i == 0:
            yield i
            if i != int(n / i):
                yield int(n / i)
        i += 1


def update_elf_use_counts(counts, new_uses):
    for elf_no in new_uses:
        if elf_no in counts:
            counts[elf_no] += 1
        else:
            counts[elf_no] = 1


elf_use_counts = {}
house_no = 1
while True:
    elves_might_visit = get_divisors(house_no)
    elves_visited = list(
        filter(lambda x: elf_use_counts.get(x, 0) < 50, elves_might_visit))
    update_elf_use_counts(elf_use_counts, elves_visited)

    num_presents = sum(elves_visited) * 11
    print(house_no, num_presents)
    if int(input) <= num_presents:
        break
    house_no += 1
