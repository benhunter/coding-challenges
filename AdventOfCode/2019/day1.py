# https://adventofcode.com/2019/day/1

with open('day1-input.txt') as f:
    data = [int(line.rstrip()) for line in f]
    # print(data)

fuel = sum([module // 3 - 2 for module in data])

print(f'Part 1: {fuel}')  # 3363033

for module_mass in data:
    module_fuel = module_mass // 3 - 2
