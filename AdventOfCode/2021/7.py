# Problem: https://adventofcode.com/2021/day/7

with open('7-input.txt') as f:
# with open('7-input-test.txt') as f:
    data = f.readline().strip().split(',')
    data = list(map(int, data))
    print(data)

part1 = min([sum(abs(posn - x) for x in data) for posn in range(min(data), max(data) + 1)])
print(f'Part 1: {part1}')  # Part 1: 349769

# Part 2
part2 = min([sum(abs((posn - x) * (1 + abs(posn - x)) / 2) for x in data) for posn in range(min(data), max(data) + 1)])
print(f'Part 2: {part2}')  # Part 2: 99540554