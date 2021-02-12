# Advent of Code
# https://adventofcode.com/2020/


if __name__ == "__main__":
    filename = "./AdventOfCode/2020/day-input.txt"
    # filename = "./AdventOfCode/2020/day-example1-input.txt"

    with open(filename) as f:
        lines = [line.rstrip() for line in f]
    print(lines)
