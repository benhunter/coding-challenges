# Problem: https://adventofcode.com/2021/day/6

from collections import defaultdict

from pprint import pprint

# with open('6-input-test.txt') as f:
with open('6-input.txt') as f:
    data = f.readline().strip().split(',')
    data = list(map(int, data))
    # print(data)


def simulate_day(fishes):
    updated_fishes = fishes.copy()
    new_fishes = 0
    for i, fish in enumerate(updated_fishes):
        if fish == 0:
            updated_fishes[i] = 6
            new_fishes += 1
        else:
            updated_fishes[i] = fish - 1
    new_fishes = [8] * new_fishes
    updated_fishes.extend(new_fishes)
    return updated_fishes


days = 80
updated_fishes = data.copy()
for n in range(days):
    updated_fishes = simulate_day(updated_fishes)
    # print(f'Day {n + 1}: {len(updated_fishes)} fish {updated_fishes}')
    # print(f'Day {n + 1}')

part1 = len(updated_fishes)
print(f'Part 1: {part1}')  # 80 days: 362639, test: 5934


# Part 2
# Make a dict holding the number of fish with each value
def make_dict(fishes):
    fishes_dict = defaultdict(int)
    for fish in data:
        if fish in fishes_dict:
            fishes_dict[fish] += 1
        else:
            fishes_dict[fish] = 1
    return fishes_dict


def simulate_day_dict(fishes_dict):
    new_fish = 0
    if 0 in fishes_dict:
        new_fish = fishes_dict[0]

    for day in range(1, 9):
        fishes_dict[day - 1] = fishes_dict[day]

    fishes_dict[6] += new_fish
    fishes_dict[8] = new_fish

    return fishes_dict


def days(days, fishes_dict):
    for day in range(days):
        fishes_dict = simulate_day_dict(fishes_dict)
        print(f'Day {day + 1} Part 2: count: {sum(fishes_dict.values())} fish: {fishes_dict}')
        pprint(fishes_dict)
    return fishes_dict

d = 256
part2 = days(d, make_dict(data))
part2 = sum(part2.values())
print(f'Part 2: {part2}')  # 1639854996917, test: 26984457539
