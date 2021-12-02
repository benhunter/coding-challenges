# https://adventofcode.com/2021/day/1

with open('day1-input.txt') as f:
    data = [int(line.rstrip()) for line in f]
    # print(data)

increase_count = 0
prev = data[0]
for curr in data[1:]:
    if curr > prev:
        increase_count += 1
    prev = curr

print(f'Part 1: {increase_count}')

increase_count_triplet = 0
prev_triplet = sum(data[:3])
for i in range(4, len(data) + 1):
    curr_triplet = sum(data[i-3: i])
    if curr_triplet > prev_triplet:
        increase_count_triplet += 1
    prev_triplet = curr_triplet

print(f'Part 2: {increase_count_triplet}')


def part1():
    return sum(map(lambda x, y: x < y, data, data[1:]))


def part2():
    return sum(map(lambda x, y: x < y, data, data[3:]))


print(part1())
print(part2())
