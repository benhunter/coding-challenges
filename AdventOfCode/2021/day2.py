# https://adventofcode.com/2021/day/2

with open('day2-input.txt') as f:
    data = [line.rstrip().split() for line in f]
    data = list(map(lambda command: [command[0], int(command[1])], data))
    # print(data)

horizontal_position = 0
depth = 0

for direction, distance in data:
    if direction == "forward":
        horizontal_position += distance
    elif direction == "down":
        depth += distance
    elif direction == "up":
        depth -= distance

print(f'Part 1: {horizontal_position * depth}')

horizontal_position = 0
depth = 0
aim = 0

for direction, distance in data:
    if direction == "forward":
        horizontal_position += distance
        depth += aim * distance
    elif direction == "down":
        aim += distance
    elif direction == "up":
        aim -= distance

print(f'Part 2: {horizontal_position * depth}')
