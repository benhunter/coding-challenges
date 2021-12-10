# Problem:

# with open('X-input.txt') as f:
with open('X-input-test.txt') as f:
    data = [line.rstrip() for line in f]
    # print(data)

for line in data:
    print(f"number: {line}")

part1 = 0
print(f'Part 1: {part1}')

# Part 2

part2 = 0
print(f'Part 2: {part2}')
