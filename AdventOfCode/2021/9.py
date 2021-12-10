# Problem: https://adventofcode.com/2021/day/9


# with open('9-input.txt') as f:
with open('9-input-test.txt') as f:
    data = [[int(c) for c in line.rstrip()] for line in f]
    # print(data)


# Return true if data[y][x] is the lowest of its neighbors.
def lowest(data, x, y):
    if x > 0 & data[y][x] >= data[y][x-1]:
        return False
    if x < len(data[y]) - 1 & data[y][x] >= data[y][x+1]:
        return False
    if y > 0 & data[y][x] >= data[y-1][x]:
        return False
    if y < len(data) - 1 & data[y][x] >= data[y+1][x]:
        return False
    return True


lows = []
for y, line in enumerate(data):
    print(f"number: {line}")
    for x, n in enumerate(line):
        print(f"x: {x}, y: {y}, n: {n}")
        if lowest(data, x, y):
            lows.append((x, y))
print(f"lows: {lows}")

part1 = 0
print(f'Part 1: {part1}')

# Part 2

part2 = 0
print(f'Part 2: {part2}')
