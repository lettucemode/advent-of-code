import aoc
import copy


def get_boss(input):
    for line in input.splitlines():
        parts = line.split(' ')
        if parts[0] == 'Hit':
            hp = int(parts[2])
        elif parts[0] == 'Damage:':
            dmg = int(parts[1])
    return hp, dmg


def calculate_damage(damage, armor):
    return max(1, damage - armor)


def boss_turn(stats, active_effects):
    armor_active = 'shield' in active_effects
    dmg = calculate_damage(boss_dmg, 7 if armor_active else 0)
    stats['player_hp'] -= dmg
    return stats


def resolve_effects(stats, active_effects):
    for effect in active_effects:
        if effect == 'poison':
            stats['boss_hp'] -= 3
        elif effect == 'recharge':
            stats['player_mana'] += 101
        active_effects[effect] -= 1
    return stats, {key: active_effects[key] for key in active_effects if active_effects[key] > 0}


def player_turn(stats, active_effects, spell, cost):
    if spell == 'magic_missile':
        stats['boss_hp'] -= 4
    elif spell == 'drain':
        stats['boss_hp'] -= 2
        stats['player_hp'] += 2
    elif spell == 'shield':
        active_effects[spell] = 6
    elif spell == 'poison':
        active_effects[spell] = 6
    elif spell == 'recharge':
        active_effects[spell] = 5

    stats['player_mana'] -= cost
    stats['total_mana_spent'] += cost
    return stats, active_effects


def deep_search(stats, active_effects, spell, cost, solutions):

    stats['player_hp'] -= 1
    if stats['player_hp'] < 1:
        return solutions

    stats, active_effects = resolve_effects(stats, active_effects)
    if stats['boss_hp'] < 1:
        return solutions + [stats['total_mana_spent']]

    if stats['player_mana'] < cost:
        return solutions
    if spell in active_effects:
        return solutions

    stats, active_effects = player_turn(stats, active_effects, spell, cost)
    if stats['boss_hp'] < 1:
        return solutions + [stats['total_mana_spent']]

    if 0 < len(solutions) and min(solutions) < stats['total_mana_spent']:
        return solutions

    stats, active_effects = resolve_effects(stats, active_effects)
    if stats['boss_hp'] < 1:
        return solutions + [stats['total_mana_spent']]

    if spell != '':
        stats = boss_turn(stats, active_effects)
    if stats['player_hp'] < 1:
        return solutions

    # cast the next spell
    solutions = deep_search(copy.copy(stats), copy.copy(
        active_effects), 'magic_missile', 53, solutions)
    solutions = deep_search(copy.copy(stats), copy.copy(
        active_effects), 'drain', 73, solutions)
    solutions = deep_search(copy.copy(stats), copy.copy(
        active_effects), 'shield', 113, solutions)
    solutions = deep_search(copy.copy(stats), copy.copy(
        active_effects), 'poison', 173, solutions)
    solutions = deep_search(copy.copy(stats), copy.copy(
        active_effects), 'recharge', 229, solutions)
    return solutions


input = aoc.get_input(22)
boss_hp, boss_dmg = get_boss(input)
player_hp = 50
player_mana = 500
mana_costs = []
stats = {'player_hp': player_hp, 'player_mana': player_mana,
         'boss_hp': boss_hp, 'boss_dmg': boss_dmg, 'total_mana_spent': 0}


print(min(deep_search(stats, {}, '', 0, [])))
