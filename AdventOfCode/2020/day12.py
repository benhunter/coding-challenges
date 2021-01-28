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


if __name__ == "__main__":
    filepath = ".\\AdventOfCode\\2020\\day12-input.txt"
    with open(filepath) as f:
        instructions = [line.strip() for line in f]

    print(instructions)

    # state
    state_north = 0
    state_east = 0
    state_heading = 90

    for ins in instructions:
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
            instructions.append(heading_to_dir(state_heading) + str(value))
        elif command == "L":
            state_heading = (state_heading - value) % 360
        elif command == "R":
            state_heading = (state_heading + value) % 360
    print(f"North: {state_north} East: {state_east} Heading:{state_heading}")
    print(f"Part 1: {abs(state_north) + abs(state_east)}")
