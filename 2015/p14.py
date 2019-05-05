import aoc


def parse_reindeer_stats(input):
    reindeer = []
    for line in input.splitlines():
        parts = line.split(' ')
        name = parts[0]
        speed = int(parts[3])
        fly_duration = int(parts[6])
        rest_duration = int(parts[13])
        reindeer.append({
            'name': name, 'speed': speed, 'fly_duration': fly_duration, 'rest_duration': rest_duration})
    return reindeer


def is_flying(fly_duration, rest_duration, time):
    bounded = time % (fly_duration + rest_duration)
    return bounded < fly_duration


def race(reindeer, seconds):
    results = {r['name']: {'distance': 0, 'points': 0} for r in reindeer}
    for this_sec in range(seconds):
        for r in reindeer:
            if is_flying(r['fly_duration'], r['rest_duration'], this_sec):
                results[r['name']]['distance'] += r['speed']

        lead_value = max(
            list(map(lambda x: x['distance'], list(results.values()))))
        lead_reindeer = list(
            filter(lambda x: x['distance'] == lead_value, results.values()))
        for lr in lead_reindeer:
            lr['points'] += 1
    return results


input = aoc.get_input(14)
reindeer = parse_reindeer_stats(input)
results = race(reindeer, 2503)

distances = list(map(lambda x: x['distance'], list(results.values())))
points = list(map(lambda x: x['points'], list(results.values())))
print(distances, points)
print(max(distances), max(points))
