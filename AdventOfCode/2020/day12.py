# Advent of Code Day 12
# https://adventofcode.com/2020/day/12


def heading_to_dir(heading):
    if heading == 0:
        return "N"
    elif heading == 90:
        return "E"
    elif heading == 180:
        return "S"
    elif heading == 270:
        return "W"


def rotate_waypoint(old_north, old_east, degrees):
    # degrees in increments of 90, pos or neg
    new_north = old_north
    new_east = old_east
    degrees = (degrees + 360) % 360  # make the movement positive
    for turns in range(degrees // 90):
        old_north = new_north
        old_east = new_east
        new_north = -1 * old_east
        new_east = old_north

    return new_north, new_east


def test_rotate_waypoint():
    north, east = rotate_waypoint(10, 4, 90)
    assert north == -4 and east == 10

    north, east = rotate_waypoint(10, 4, 180)
    assert north == -10 and east == -4

    north, east = rotate_waypoint(10, 4, 270)
    assert north == 4 and east == -10

    north, east = rotate_waypoint(10, 4, 360)
    assert north == 10 and east == 4

    north, east = rotate_waypoint(10, 4, -90)
    assert north == 4 and east == -10

    north, east = rotate_waypoint(10, 4, -180)
    assert north == -10 and east == -4

    north, east = rotate_waypoint(10, 4, -270)
    assert north == -4 and east == 10


if __name__ == "__main__":
    filepath = ".\\AdventOfCode\\2020\\day12-input.txt"
    with open(filepath) as f:
        instructions = [line.strip() for line in f]

    # print(instructions)

    part1_instr = instructions.copy()

    # state
    state_north = 0
    state_east = 0
    state_heading = 90

    for ins in part1_instr:
        command = ins[0]
        value = int(ins[1:])

        if command == "N":
            state_north += value
        elif command == "S":
            state_north -= value
        elif command == "E":
            state_east += value
        elif command == "W":
            state_east -= value
        elif command == "F":
            part1_instr.append(heading_to_dir(state_heading) + str(value))
        elif command == "L":
            state_heading = (state_heading - value) % 360
        elif command == "R":
            state_heading = (state_heading + value) % 360

    # print(f"North: {state_north} East: {state_east} Heading:{state_heading}")
    print(f"Part 1: {abs(state_north) + abs(state_east)}")

    # Part 2
    part2_instr = instructions.copy()
    # state
    state_north = 0
    state_east = 0
    # state_heading = 90
    waypoint_north = 1
    waypoint_east = 10

    for ins in part2_instr:
        command = ins[0]
        value = int(ins[1:])

        if command == "N":
            waypoint_north += value
        elif command == "S":
            waypoint_north -= value
        elif command == "E":
            waypoint_east += value
        elif command == "W":
            waypoint_east -= value
        elif command == "F":
            state_north += waypoint_north * value
            state_east += waypoint_east * value
        elif command == "L":
            waypoint_north, waypoint_east = rotate_waypoint(
                waypoint_north, waypoint_east, -1 * value
            )
        elif command == "R":
            waypoint_north, waypoint_east = rotate_waypoint(
                waypoint_north, waypoint_east, value
            )

    # print(f"North: {state_north} East: {state_east} Heading:{state_heading}")
    print(f"Part 2: {abs(state_north) + abs(state_east)}")  # 6276 too low
