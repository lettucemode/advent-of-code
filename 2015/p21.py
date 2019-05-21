import aoc


class Combatant:

    def __init__(self, hp):
        self.items = []
        self.max_hitpoints = hp
        self.hitpoints = hp

    def add_item(self, item):
        self.items.append(item)

    def get_damage(self):
        return sum([item['damage'] for item in self.items])

    def get_armor(self):
        return sum([item['armor'] for item in self.items])

    def get_cost(self):
        return sum([item['cost'] for item in self.items])

    def reset_hp(self):
        self.hitpoints = self.max_hitpoints

    def reset_items(self):
        self.items.clear()


def get_boss(input):
    for line in input.splitlines():
        parts = line.split(' ')
        if parts[0] == 'Hit':
            hp = int(parts[2])
        elif parts[0] == 'Damage:':
            dmg = int(parts[1])
        elif parts[0] == 'Armor:':
            arm = int(parts[1])
    item = {'cost': 0, 'damage': dmg, 'armor': arm, 'name': 'boss'}
    boss = Combatant(hp)
    boss.add_item(item)
    return boss


def calculate_damage(damage, armor):
    return max(1, damage - armor)


def does_player_win(player, boss):
    while True:
        dmg = calculate_damage(player.get_damage(), boss.get_armor())
        boss.hitpoints -= dmg
        if boss.hitpoints < 1:
            return True

        dmg = calculate_damage(boss.get_damage(), player.get_armor())
        player.hitpoints -= dmg
        if player.hitpoints < 1:
            return False


input = aoc.get_input(21)
player_hp = 100
weapons = [
    {'name': 'dagger', 'cost': 8, 'damage': 4, 'armor': 0},
    {'name': 'shortsword', 'cost': 10, 'damage': 5, 'armor': 0},
    {'name': 'warhammer', 'cost': 25, 'damage': 6, 'armor': 0},
    {'name': 'longsword', 'cost': 40, 'damage': 7, 'armor': 0},
    {'name': 'greataxe', 'cost': 74, 'damage': 8, 'armor': 0}
]
armor = [
    {'name': 'none', 'cost': 0, 'damage': 0, 'armor': 0},
    {'name': 'leather', 'cost': 13, 'damage': 0, 'armor': 1},
    {'name': 'chainmail', 'cost': 31, 'damage': 0, 'armor': 2},
    {'name': 'splintmail', 'cost': 53, 'damage': 0, 'armor': 3},
    {'name': 'bandedmail', 'cost': 75, 'damage': 0, 'armor': 4},
    {'name': 'platemail', 'cost': 102, 'damage': 0, 'armor': 5}
]
rings = [
    {'name': 'dmg+0', 'cost': 0, 'damage': 0, 'armor': 0},
    {'name': 'dmg+1', 'cost': 25, 'damage': 1, 'armor': 0},
    {'name': 'dmg+2', 'cost': 50, 'damage': 2, 'armor': 0},
    {'name': 'dmg+3', 'cost': 100, 'damage': 3, 'armor': 0},
    {'name': 'def+0', 'cost': 0, 'damage': 0, 'armor': 0},
    {'name': 'def+1', 'cost': 20, 'damage': 0, 'armor': 1},
    {'name': 'def+2', 'cost': 40, 'damage': 0, 'armor': 2},
    {'name': 'def+3', 'cost': 80, 'damage': 0, 'armor': 3}
]

costs = []
for wep in weapons:
    for arm in armor:
        for r1 in rings:
            for r2 in rings:
                if r1 == r2:
                    continue

                player = Combatant(player_hp)
                player.add_item(wep)
                player.add_item(arm)
                player.add_item(r1)
                player.add_item(r2)
                boss = get_boss(input)

                if not does_player_win(player, boss):
                    print(list(item['name']
                               for item in player.items), player.get_cost())
                    costs.append(player.get_cost())

print(max(costs))
