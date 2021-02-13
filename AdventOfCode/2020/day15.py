# Advent of Code Day 15
# https://adventofcode.com/2020/day/15


import cProfile
from collections import defaultdict


def calc_target(TARGET, nums):
    all_prev_posn = defaultdict(lambda: -1)
    prev_position = 0

    # handle seed nums first, except the last number
    for i, x in enumerate(nums[:-1]):
        all_prev_posn[x] = i

    # iterate from the position of the last item in nums to the TARGET
    for i in range(len(nums) - 1, TARGET):
        last_num = nums[-1]
        prev_position = all_prev_posn[last_num]

        if prev_position == -1:
            diff = 0
        else:
            diff = len(nums) - prev_position - 1
        all_prev_posn[last_num] = len(nums) - 1
        nums.append(diff)
    return nums[-2]


if __name__ == "__main__":
    filename = "./AdventOfCode/2020/day15-input.txt"
    # filename = "./AdventOfCode/2020/day15-example1-input.txt"

    with open(filename) as f:
        lines = [line.rstrip() for line in f]
    # print(lines)

    # numbers = [int(x) for (x:=line.split(",")) for line in lines]
    seednums = [int(x) for line in lines for x in line.split(",")]
    # print(seednums)

    # Part 1
    TARGET = 2020
    nums = seednums.copy()
    # cProfile.run('calc_target(TARGET, nums)')  # Profiler
    part1 = calc_target(TARGET, nums)
    print(f"Part 1: {part1}")  # 468

    # Part 2
    TARGET2 = 30000000
    nums = seednums.copy()
    # cProfile.run('calc_target(TARGET2, nums)')  # Profiler

    part2 = calc_target(TARGET2, nums)
    print(f"Part 2: {part2}")  # 1801753
