# Problem: https://adventofcode.com/2021/day/8

from pprint import pprint

'''
 +0:6    *1:2    +2:5    +3:5    *4:4
 aaaa    ....    aaaa    aaaa    ....
b    c  .    c  .    c  .    c  b    c
b    c  .    c  .    c  .    c  b    c
 ....    ....    dddd    dddd    dddd
e    f  .    f  e    .  .    f  .    f
e    f  .    f  e    .  .    f  .    f
 gggg    ....    gggg    gggg    ....

 +5:5     6:6    *7:3    *8:7     9:6
 aaaa    aaaa    aaaa    aaaa    aaaa
b    .  b    .  .    c  b    c  b    c
b    .  b    .  .    c  b    c  b    c
 dddd    dddd    ....    dddd    dddd
.    f  e    f  .    f  e    f  .    f
.    f  e    f  .    f  e    f  .    f
 gggg    gggg    ....    gggg    gggg
'''
with open('8-input.txt') as f:
    # with open('8-input-test.txt') as f:
    data = [[x.strip().split() for x in line.split('|')] for line in f]
    # print(data)

# Simple data:
simple_data = 'acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab | cdfeb fcadb cdfeb cdbaf'


def part1(data):
    count = 0
    for patterns, output in data:
        # print(f'patterns {patterns}')
        # print(f'output {output}')
        count += len(list(filter(lambda x: len(x) == 2, output)))  # 1
        count += len(list(filter(lambda x: len(x) == 4, output)))  # 4
        count += len(list(filter(lambda x: len(x) == 3, output)))  # 7
        count += len(list(filter(lambda x: len(x) == 7, output)))  # 8
    return count


# print(f'Part 1: {part1(data)}')

# Part 2

zero = 'abcefg'
one = 'cf'
two = 'acdeg'
three = 'acdfg'
four = 'bcdf'
five = 'abdfg'
six = 'abdefg'
seven = 'acf'
eight = 'abcdefg'
nine = 'abcdfg'

simple_data = [[[set(c for c in word) for word in x.strip().split()] for x in simple_data.split('|')]]
print(f'simple_data {simple_data}')

for patterns, output in simple_data:
    print(f'patterns {patterns}')
    print(f'len of patterns {len(patterns)}')
    print(f'output {output}')

    pattern_1 = list(filter(lambda x: len(x) == 2, patterns))[0]  # 1, c or f
    print(f'pattern_1 {pattern_1}')
    pattern_4 = list(filter(lambda x: len(x) == 4, patterns))[0]  # 4
    print(f'pattern_4 {pattern_4}')
    pattern_7 = list(filter(lambda x: len(x) == 3, patterns))[0]  # 7
    print(f'pattern_7 {pattern_7}')
    pattern_8 = list(filter(lambda x: len(x) == 7, patterns))[0]  # 8
    print(f'pattern_8 {pattern_8}')

    unknown_patterns = list(filter(lambda x: x not in [pattern_1, pattern_4, pattern_7, pattern_8], patterns))
    print(f'unknown_patterns {unknown_patterns}')
    print(f'len of unknown_patterns {len(unknown_patterns)}')

    # segment_a = list(filter(lambda c: c not in pattern_one, [c for c in pattern_seven]))[0]
    segment_a = pattern_7 - pattern_1
    # print(f'Segment A: {next(iter(segment_a))}')
    # print(f'Segment A: {segment_a}')

    # possible_segment_b = [c for c in pattern_four if c not in pattern_one]
    possible_segment_b = pattern_4 - pattern_1
    # print(f'Possible segment B: {possible_segment_b}')

    possible_segment_c = pattern_1
    # print(f'Possible segment C: {possible_segment_c}')

    # segment_d will not be in pattern_zero
    possible_pattern_zero = list(filter(lambda x: len(x) == 6, unknown_patterns))
    print(f'Possible pattern zero: {possible_pattern_zero}')
    # pattern_zero = list(filter(lambda x: len(x) == 6, patterns))
    # print(f'pattern_zero {pattern_zero}')

    possible_segment_e = pattern_8 - pattern_4 - pattern_7
    # print(f'Possible segment E: {possible_segment_e}')
    possible_segment_g = possible_segment_e
    # print(f'Possible segment G: {possible_segment_g}')

    possible_pattern_2_3_5 = list(filter(lambda x: len(x) == 5, unknown_patterns))
    # print(f'Possible pattern 3: {possible_pattern_2_3_5}')

    for p in possible_pattern_2_3_5:
        possible_minus_4_7 = p - pattern_4 - pattern_7
        # print(f'p - pattern_four - pattern_seven {possible_minus_4_7}')
        if len(possible_minus_4_7) == 2:
            pattern_2 = p
            unknown_patterns.remove(p)
        if len(possible_minus_4_7) == 1:
            segment_g = possible_minus_4_7

    print(f'pattern_2 {pattern_2}')
    # print(f'Segment G: {segment_g}')

    possible_pattern_3_5 = list(filter(lambda x: len(x) == 5, unknown_patterns))
    # print(f'Possible pattern 3 or 5: {possible_pattern_3_5}')

    for p in possible_pattern_3_5:
        possible_minus_2 = p - pattern_2
        print(f'p - pattern_two {possible_minus_2}')
        if len(possible_minus_2) == 2:
            pattern_5 = p
            unknown_patterns.remove(p)
        if len(possible_minus_2) == 1:
            pattern_3 = p
            unknown_patterns.remove(p)
    print(f'len unknown_patterns {len(unknown_patterns)}')
    assert len(unknown_patterns) == 3
    print(f'pattern_5 {pattern_5}')
    print(f'pattern_3 {pattern_3}')

    possible_pattern_6 = list(filter(lambda x: len(x) == 6, unknown_patterns))
    print(f'possible_pattern_6 {possible_pattern_6}')
    for p in possible_pattern_6:
        possible_minus_5 = p - pattern_5
        if len(possible_minus_5) == 1:
            pattern_6 = p
            unknown_patterns.remove(p)

    print(f'len unknown_patterns {len(unknown_patterns)}')
    assert len(unknown_patterns) == 2

    print(f'pattern_6 {pattern_6}')
    print(f'unknown_patterns {unknown_patterns}')



    # pattern_9 has segment d, but not e. Patterns: 5 + 4 = 9
    pattern_9 = pattern_5 | pattern_4
    print(f'pattern_9 {pattern_9}')
    unknown_patterns.remove(pattern_9)
    # pattern_0 does not have segment d, but does have e
    pattern_0 = unknown_patterns[0]
    print(f'pattern_0 {pattern_0}')


# TODO: generate all possible patterns and filter until only one possible combination remains

part2 = 0
# print(f'Part 2: {part2}')
