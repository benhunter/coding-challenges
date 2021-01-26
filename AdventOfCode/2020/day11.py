# Advent of Code Day 11
# https://adventofcode.com/2020/day/11


def count_adj(row, column, area):
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
    return count


def count_visible(row, column, area):
    # stop checking if there is an empty chair found
    count = 0

    # up
    for r in range(row):
        if area[row - r - 1][column] == "#":
            count += 1
            break
        if area[row - r - 1][column] == "L":
            break
    # down
    for r in range(len(area) - row - 1):
        if area[row + r + 1][column] == "#":
            count += 1
            break
        if area[row + r + 1][column] == "L":
            break
    # left
    for c in range(column):
        if area[row][column - c - 1] == "#":
            count += 1
            break
        if area[row][column - c - 1] == "L":
            break
    # right
    for c in range(len(area[row]) - column - 1):
        if area[row][column + c + 1] == "#":
            count += 1
            break
        if area[row][column + c + 1] == "L":
            break
    # up left
    for dist in range(min(row, column)):
        if area[row - dist - 1][column - dist - 1] == "#":
            count += 1
            break
        if area[row - dist - 1][column - dist - 1] == "L":
            break
    # up right
    for dist in range(min(row, len(area[row]) - column - 1)):
        if area[row - dist - 1][column + dist + 1] == "#":
            count += 1
            break
        if area[row - dist - 1][column + dist + 1] == "L":
            break
    # down left
    for dist in range(min(len(area) - row - 1, column)):
        if area[row + dist + 1][column - dist - 1] == "#":
            count += 1
            break
        if area[row + dist + 1][column - dist - 1] == "L":
            break
    # down right
    for dist in range(min(len(area) - row - 1, len(area[row]) - column - 1)):
        if area[row + dist + 1][column + dist + 1] == "#":
            count += 1
            break
        if area[row + dist + 1][column + dist + 1] == "L":
            break
    return count


def test_count_visible():
    row = 4
    column = 3
    # put numbers in area instead of "." to assist debugging
    area = [
        ".......#.",
        "...#.....",
        ".#.......",
        ".........",
        "..#L....#",
        "....#....",
        ".........",
        "#........",
        "...#.....",
    ]
    count = count_visible(row, column, area)
    assert count == 8

    area = [
        "...#...#.",
        "#..L..L..",
        ".L.......",
        ".........",
        "#L.L...L#",
        ".........",
        ".L.......",
        "#..L..L..",
        "...#...#.",
    ]
    count = count_visible(row, column, area)
    assert count == 0


def test_count_visible_close():
    row = 4
    column = 3
    area = [
        ".........",
        ".........",
        ".........",
        "..###....",
        "..#L#....",
        "..###....",
        ".........",
        ".........",
        ".........",
    ]
    count = count_visible(row, column, area)
    assert count == 8


def test_count_visible_LOS_blocked():
    row = 4
    column = 3
    area = [
        ".........",
        ".........",
        ".LLLLL...",
        ".L###L...",
        ".L#L#L...",
        ".L###L...",
        ".LLLLL...",
        ".........",
        ".........",
    ]
    count = count_visible(row, column, area)
    assert count == 8


def test_count_visible_LOS_open():
    row = 4
    column = 3
    area = [
        ".........",
        ".........",
        ".#.#.#...",
        "..LLL....",
        ".#LLL#...",
        "..LLL....",
        ".#.#.#...",
        ".........",
        ".........",
    ]
    count = count_visible(row, column, area)
    assert count == 0


def update_seat(row, column, area, count_func=count_adj, part2=False):
    if area[row][column] == ".":
        return "."
    elif area[row][column] == "":
        raise RuntimeError(f"Invalid seat: row: {row}, column: {column}")

    count = count_func(row, column, area)

    # determine change
    if area[row][column] == "L" and count == 0:
        return "#"
    if area[row][column] == "#":
        if not part2 and count >= 4:
            return "L"
        elif part2 and count >= 5:
            return "L"
    return area[row][column]


def update_area(area, part2=False):
    new_area = []
    for row_index in range(len(area)):
        new_area.append([])
        for column_index in range(len(area[row_index])):
            if not part2:
                new_area[-1].append(
                    update_seat(
                        row_index, column_index, area, count_func=count_adj, part2=False
                    )
                )
            else:
                new_area[-1].append(
                    update_seat(
                        row_index,
                        column_index,
                        area,
                        count_func=count_visible,
                        part2=True,
                    )
                )

    return new_area


if __name__ == "__main__":
    filepath = ".\\AdventOfCode\\2020\\day11-input.txt"
    with open(filepath) as f:
        orig_area = [line.strip() for line in f]

    old_area = orig_area.copy()
    new_area = update_area(old_area)
    while (new_area := update_area(old_area)) != old_area:
        old_area = new_area
    #     print(False)
    # print(True)

    part1_count = sum(
        [
            1
            for row_index in range(len(new_area))
            for column_index in range(len(new_area[row_index]))
            if new_area[row_index][column_index] == "#"
        ]
    )
    print(f"Part 1: {part1_count}")  # 2289

    # Part 2
    old_area = orig_area.copy()
    new_area = update_area(old_area, part2=True)
    while (new_area := update_area(old_area, part2=True)) != old_area:
        old_area = new_area

    part2_count = sum(
        [
            1
            for row_index in range(len(new_area))
            for column_index in range(len(new_area[row_index]))
            if new_area[row_index][column_index] == "#"
        ]
    )

    print(f"Part 2: {part2_count}")  # 2059
