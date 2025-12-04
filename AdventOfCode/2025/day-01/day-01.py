

def get_lines(file="day-01.in"):
    with open(file) as f:
        lines = [line.rstrip() for line in f]
        # print(lines)
        return lines

def part1(lines):
    dial = 50
    zeros = 0

    for l in lines:
        direction = l[0]
        dist = int(l[1:])
        # print(direction, dist)

        if direction == 'L':
            dial -= dist
        else:
            dial += dist

        dial = dial % 100
        # print(dial)

        if dial == 0:
            zeros += 1

    return zeros

def part2(lines):
    dial = 50
    zeros = 0

    for l in lines:
        direction = l[0]
        dist = int(l[1:])
        # print(f"Start at {dial}. Go {direction} {dist}")

        if dist >= 100: # or dist <= 0:
            rotations = abs(dist) // 100
            zeros += rotations
            dist = dist % 100

        if direction == 'L':
            if dial - dist <= 0 and dial != 0:
                zeros += 1
            dial -= dist
        else:
            if dial + dist > 99:
                zeros += 1
            dial += dist

        dial = dial % 100

        # print(f"After {direction} {dist}, at {dial}, {zeros=}")

    return zeros

print(f"Test Part 1: {part1(get_lines("day-01.test.in"))}")
print(f"Test Part 2: {part2(get_lines("day-01.test.in"))}")

print(f"Part 1: {part1(get_lines())}") # 1021
print(f"Part 2: {part2(get_lines())}") # 5933

