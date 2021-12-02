# https://adventofcode.com/2019/day/1

with open('day1-input.txt') as f:
    data = [int(line.rstrip()) for line in f]
    # print(data)

# fuel = 0
# for module in data:
#     fuel += module // 3 - 2

fuel = sum([module // 3 - 2 for module in data])

print(f'Part 1: {fuel}')  # 3363033
