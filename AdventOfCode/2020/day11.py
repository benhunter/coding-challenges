# Advent of Code Day 11
# https://adventofcode.com/2020/day/11


def update_seat(row, column, area):
    if area[row][column] == ".":
        return "."
    elif area[row][column] == "":
        raise RuntimeError(f"Invalid seat: row: {row}, column: {column}")

    count = 0
    # count row above
    if row > 0:
        if column > 0:
            if area[row - 1][column - 1] == "#":
                count += 1
        if area[row - 1][column] == "#":
            count += 1
        if column < (len(area[row]) - 1):
            if area[row - 1][column + 1] == "#":
                count += 1
    # count own row
    if column > 0:
        if area[row][column - 1] == "#":
            count += 1
    if column < (len(area[row]) - 1):
        if area[row][column + 1] == "#":
            count += 1
    # count row below
    if row < (len(area) - 1):
        if column > 0:
            if area[row + 1][column - 1] == "#":
                count += 1
        if area[row + 1][column] == "#":
            count += 1
        if column < (len(area[row]) - 1):
            if area[row + 1][column + 1] == "#":
                count += 1

    # determine change
    if area[row][column] == "L" and count == 0:
        return "#"
    if area[row][column] == "#" and count >= 4:
        return "L"
    return area[row][column]


def update_area(area):
    new_area = []
    for row_index in range(len(area)):
        new_area.append([])
        for column_index in range(len(area[row_index])):
            new_area[-1].append(update_seat(row_index, column_index, area))
    return new_area


filepath = ".\\AdventOfCode\\2020\\day11-input.txt"
with open(filepath) as f:
    orig_area = [line.strip() for line in f]

old_area = orig_area
new_area = update_area(old_area)
while ((new_area := update_area(old_area)) != old_area):
    old_area = new_area
#     print(False)
# print(True)

count = 0
for row_index in range(len(new_area)):
    for column_index in range(len(new_area[row_index])):
        if new_area[row_index][column_index] == "#":
            count += 1
print(f"Part 1: {count}")
