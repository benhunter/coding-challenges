# https://adventofcode.com/2021/day/3

from collections import defaultdict

with open('day3-input.txt') as f:
# with open('day3-input-test.txt') as f:
    data = [line.rstrip() for line in f]
    # print(data)


def count_bits(data):
    zeros = defaultdict(int)
    ones = defaultdict(int)
    for number in data:
        for posn, value in enumerate(number):
            if value == '0':
                zeros[posn] += 1
            else:
                ones[posn] += 1
    return zeros, ones


zeros, ones = count_bits(data)
# print(f'Zeros: {zeros}')
# print(f'Ones: {ones}')


def calc_gamma_epsilon(zeros, ones):
    gamma = ''
    epsilon = ''
    for posn in range(len(data[0])):
        # print(f'{posn} {zeros[posn]}')
        gamma += '0' if zeros[posn] > ones[posn] else '1'
        epsilon += '1' if zeros[posn] > ones[posn] else '0'
    return gamma, epsilon


gamma, epsilon = calc_gamma_epsilon(zeros, ones)
# print(f'Gamma: {gamma}, {int(gamma, 2)}')
# print(f'Epsilon: {epsilon}')
print(f'Part 1: {int(gamma, 2) * int(epsilon, 2)}')

# Part 2
# Oxygen filter - use gamma, unless tie, use '1'
# CO2 filter - use epsilon, unless tie, use '0'

possible_oxygen = data.copy()
# print(f'Possible oxygen: {possible_oxygen}')
for posn in range(len(data[0])):
    if len(possible_oxygen) == 1:
        break
    gamma, epsilon = calc_gamma_epsilon(*count_bits(possible_oxygen))
    filter_value = '1' if gamma[posn] == epsilon[posn] else gamma[posn]
    # print(f'Filter value: {filter_value}')
    possible_oxygen = list(filter(lambda number: number[posn] == filter_value, possible_oxygen))
# print(f'Oxygen: {possible_oxygen[0]}, {int(possible_oxygen[0], 2)}')

possible_co2 = data.copy()
for posn in range(len(data[0])):
    if len(possible_co2) == 1:
        break
    gamma, epsilon = calc_gamma_epsilon(*count_bits(possible_co2))
    filter_value = '0' if gamma[posn] == epsilon[posn] else epsilon[posn]
    # print(f'Filter value: {filter_value}')
    possible_co2 = list(filter(lambda number: number[posn] == filter_value, possible_co2))
# print(f'CO2: {possible_co2[0]}, {int(possible_co2[0], 2)}')
print(f'Part 2: {int(possible_co2[0], 2) * int(possible_oxygen[0], 2)}')




