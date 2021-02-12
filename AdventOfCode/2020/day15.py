# Advent of Code Day 15
# https://adventofcode.com/2020/day/15


TARGET = 2020


if __name__ == "__main__":
    # filename = "./AdventOfCode/2020/day15-input.txt"
    filename = "./AdventOfCode/2020/day15-example1-input.txt"

    with open(filename) as f:
        lines = [line.rstrip() for line in f]
    print(lines)

    # numbers = [int(x) for (x:=line.split(",")) for line in lines]
    seed_nums = [int(x) for line in lines for x in line.split(",")]
    print(seed_nums)

    nums = seed_nums.copy()
    print(nums)

    for i in range(TARGET):
        if len(nums) > i:
            continue
        # print(i)

        last_num = nums[-1]

        diff = len(nums) - prev_position  # or 0 if not in nums

        nums.append(diff)
