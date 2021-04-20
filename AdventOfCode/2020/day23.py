# Advent of Code 2020 Day
# https://adventofcode.com/2020/day/23


import cProfile


DEBUG = False
PROFILE = True


# def next_three(cups, current_posn):
#     for count in range(3):
#         pass
#     return


# def test_next_three_no_wrap():
#     cups = [0, 1, 2, 3]
#     current_posn = 0

#     assert next_three(cups, current_posn) == [1, 2, 3]


# def test_next_three_wrap():
#     cups = [0, 1, 2, 3]
#     current_posn = 2

#     assert next_three(cups, current_posn) == [3, 0, 1]


def load_cups(filename):
    with open(filename) as f:
        lines = [line.rstrip() for line in f]
    cups = lines[0]
    cups = [int(cup) for cup in cups]
    return cups


def repr_cups(cups, current_posn):
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


def select_destination(cups, current_value, next_three):
    # The crab selects a destination cup: the cup with a label equal to the current
    # cup's label minus one. If this would select one of the cups that was just
    # picked up, the crab will keep subtracting one until it finds a cup that
    # wasn't just picked up. If at any point in this process the value goes below
    # the lowest value on any cup's label, it wraps around to the highest value on
    # any cup's label instead.
    destination_value = current_value
    for _ in range(len(cups)):
        destination_value -= 1
        if destination_value in next_three:
            continue
        elif destination_value < 1:
            destination_value = max(cups)
        try:
            destination_posn = cups.index(destination_value)
            break
        except ValueError:
            continue
    if DEBUG:
        print(f"destination: {destination_value}\n")
    return destination_posn


def move(cups, moves):
    current_posn = 0
    current_value = cups[current_posn]
    len_cups = len(cups)
    move_count = 1
    for _ in range(moves):
        if PROFILE and move_count % 1000 == 0:
            print(f"move count: {move_count}")
        if DEBUG:
            print(f"-- move {move_count} --")
            print(f"cups: {repr_cups(cups, current_posn)}")
        index = current_posn + 1
        next_three = []
        if index + 3 < len_cups:
            next_three = cups[index : index + 3]
            del cups[index : index + 3]
        else:
            for i in range(3):
                if index >= len(cups):
                    index = 0
                next_three.append(cups.pop(index))
        if DEBUG:
            print(f"pick up: {', '.join(str(cup) for cup in next_three)}")
        destination_posn = select_destination(cups, current_value, next_three)
        cups[destination_posn + 1 : destination_posn + 1] = next_three
        move_count += 1
        if current_posn > destination_posn:
            # will change if next_three
            # was inserted before current_posn
            current_posn += 3
        current_posn = (current_posn + 1) if (current_posn + 1) < len_cups else 0
        current_value = cups[current_posn]
    return cups


def move_dict(cups, moves, current):
    # cups list to dict
    cups_dict = {}
    for index, cup in enumerate(cups[:-1]):
        cups_dict[cup] = cups[index + 1]
    cups_dict[cups[-1]] = cups[0]

    for _ in range(moves):
        current_next = cups_dict[cups_dict[cups_dict[cups_dict[current]]]]
        next_three = [
            cups_dict[current],
            cups_dict[cups_dict[current]],
            cups_dict[cups_dict[cups_dict[current]]]
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


def test_move_part1_example1_10():
    filename = "./AdventOfCode/2020/day23-example1-input.txt"
    cups = load_cups(filename)
    cups = move(cups, 10)
    one_index = cups.index(1)
    part1 = "".join(str(cup) for cup in cups[one_index + 1 :] + cups[:one_index])
    assert part1 == "92658374"


def test_move_part1_example1_10_dict():
    filename = "./AdventOfCode/2020/day23-example1-input.txt"
    cups = load_cups(filename)
    # cups = move(cups, 10)
    cups = move_dict(cups, 10, cups[0])
    # one_index = cups[1]
    # part1 = "".join(str(cup) for cup in cups[one_index + 1 :] + cups[:one_index])
    part1 = ""
    current = 1
    for _ in range(len(cups.keys()) - 1):
        part1 += str(cups[current])
        current = cups[current]
    assert part1 == "92658374"


def test_move_part1_example1_100():
    filename = "./AdventOfCode/2020/day23-example1-input.txt"
    cups = load_cups(filename)
    cups = move(cups, 100)
    one_index = cups.index(1)
    part1 = "".join(str(cup) for cup in cups[one_index + 1 :] + cups[:one_index])
    assert part1 == "67384529"


def test_move_part1_input_100():
    filename = "./AdventOfCode/2020/day23-input.txt"
    cups = load_cups(filename)
    cups = move(cups, 100)
    one_index = cups.index(1)
    part1 = "".join(str(cup) for cup in cups[one_index + 1 :] + cups[:one_index])
    assert part1 == "28793654"


def test_part2_example1():
    filename = "./AdventOfCode/2020/day23-example1-input.txt"
    cups = load_cups(filename)
    cups += list(range(max(cups) + 1, 1000000))
    # cups = move(cups, 10000000)
    one_index = cups.index(1)
    assert cups[one_index + 1] == 934001
    assert cups[one_index + 2] == 159792
    part2 = cups[one_index + 1] * cups[one_index + 2]
    assert part2 == 149245887792


def profile_part2():
    MOVES = 10000
    filename = "./AdventOfCode/2020/day23-example1-input.txt"
    cups = load_cups(filename)
    cups += list(range(max(cups) + 1, 1000000))
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

    # Part 2
    # cups = orig_cups.copy()
    # cups += list(range(max(cups) + 1,1000000))
    # print(cups)


if __name__ == "__main__":
    # if PROFILE:
    #     profile_part2()
    # test_part2_example1()
    # main()

    test_move_part1_example1_10_dict()
