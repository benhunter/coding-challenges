# Advent of Code 2020 Day
# https://adventofcode.com/2020/day/23


import cProfile
from typing import List

DEBUG = False
PROFILE = True


def load_cups(filename):
    with open(filename) as f:
        lines = [line.rstrip() for line in f]
    cups = lines[0]
    cups = [int(cup) for cup in cups]
    return cups


def repr_cups(cups: List, current_posn):
    str_cups = ""
    for index, cup in enumerate(cups):
        if index == current_posn:
            str_cups += f"({cup})"
        else:
            str_cups += f" {cup} "

    return str_cups


def test_repr_cups():
    cups = [0, 1, 2, 3]
    current_posn = 0
    assert repr_cups(cups, current_posn) == "(0) 1  2  3 "

    current_posn = 2
    assert repr_cups(cups, current_posn) == " 0  1 (2) 3 "


def move(cups, moves):
    # cups list to dict
    cups_dict = {}
    for index, cup in enumerate(cups[:-1]):
        cups_dict[cup] = cups[index + 1]
    cups_dict[cups[-1]] = cups[0]
    current = cups[0]

    for _ in range(moves):
        current_next = cups_dict[cups_dict[cups_dict[cups_dict[current]]]]
        next_three = [
            cups_dict[current],
            cups_dict[cups_dict[current]],
            cups_dict[cups_dict[cups_dict[current]]],
        ]
        # check for negative dest
        # and dest not in next_three
        dest = current - 1
        while dest in next_three or dest < 1:
            if dest < 1:
                dest = max(cups_dict.keys())
            else:
                dest = dest - 1
        dest_next = cups_dict[dest]

        cups_dict[dest] = cups_dict[current]
        cups_dict[cups_dict[cups_dict[cups_dict[dest]]]] = dest_next
        cups_dict[current] = current_next
        current = current_next
    return cups_dict


def test_move_part1_example1_10_dict():
    filename = "./AdventOfCode/2020/day23-example1-input.txt"
    cups = load_cups(filename)
    cups = move(cups, 10)
    part1 = ""
    current = 1
    for _ in range(len(cups) - 1):
        part1 += str(cups[current])
        current = cups[current]
    assert part1 == "92658374"


def test_move_part1_example1_100_dict():
    filename = "./AdventOfCode/2020/day23-example1-input.txt"
    cups = load_cups(filename)
    cups = move(cups, 100)
    part1 = ""
    current = 1
    for _ in range(len(cups) - 1):
        part1 += str(cups[current])
        current = cups[current]
    assert part1 == "67384529"


def test_move_part1_input_100_dict():
    filename = "./AdventOfCode/2020/day23-input.txt"
    cups = load_cups(filename)
    cups = move(cups, 100)
    part1 = ""
    current = 1
    for _ in range(len(cups) - 1):
        part1 += str(cups[current])
        current = cups[current]
    assert part1 == "28793654"


def test_part2_example1_dict():
    filename = "./AdventOfCode/2020/day23-example1-input.txt"
    cups = load_cups(filename)
    cups += list(range(max(cups) + 1, 1000000 + 1))
    assert len(cups) == 1000000
    cups = move(cups, 10000000)
    assert cups[1] == 934001
    assert cups[cups[1]] == 159792
    part2 = cups[1] * cups[cups[1]]
    assert part2 == 149245887792


def profile_part2():
    MOVES = 10000
    filename = "./AdventOfCode/2020/day23-example1-input.txt"
    cups = load_cups(filename)
    cups += list(range(max(cups) + 1, 1000000 + 1))
    with cProfile.Profile() as pr:
        cups = move(cups, MOVES)
    pr.print_stats()


def profile_part2_dict():
    MOVES = 1000000
    filename = "./AdventOfCode/2020/day23-example1-input.txt"
    cups = load_cups(filename)
    cups += list(range(max(cups) + 1, 1000000 + 1))
    with cProfile.Profile() as pr:
        cups = move(cups, MOVES)
    pr.print_stats()


def main():
    filename = "./AdventOfCode/2020/day23-input.txt"
    orig_cups = load_cups(filename)
    cups = orig_cups.copy()
    # print(cups)

    # Part 1
    cups = move(cups, 100)

    print(f"-- final --")
    print(f"cups: {repr_cups(cups, 0)}")

    one_index = cups.index(1)
    part1 = "".join(str(cup) for cup in cups[one_index + 1 :] + cups[:one_index])
    print(f"Part 1: {part1}")  # 28793654
    assert part1 == "28793654"

    # Part 2
    cups = orig_cups.copy()
    cups += list(range(max(cups) + 1, 1000000 + 1))
    cups = move(cups, 10000000)
    part2 = cups[1] * cups[cups[1]]
    print(f"Part 2: {part2}")  # 359206768694
    assert part2 == 359206768694


if __name__ == "__main__":
    # if PROFILE:
    #     profile_part2()
    #     profile_part2_dict()
    # test_part2_example1()
    # test_move_part1_example1_10_dict()
    main()
