# Advent of Code 2020 Day
# https://adventofcode.com/2020/


import itertools
import math
import numpy as np

from collections import namedtuple
from pprint import pformat, pprint
from typing import List, Tuple


USE_EXAMPLE1 = False
DEBUG = False


Tile = namedtuple("Tile", "number, data")
Tile.__repr__ = lambda self: f"Tile: {self.number}\n{pformat(self.data)}"


def count_edge_matches(tile_one: Tile, tile_two: Tile):
    assert type(tile_one) is Tile
    assert type(tile_two) is Tile

    first_tile_faces = [
        tile_one.data[0],
        tile_one.data[-1],
        tile_one.data[:, 0],
        tile_one.data[:, -1],
    ]
    nparray_first_tile_flipped = np.flip(tile_one.data)
    first_tile_faces += [
        nparray_first_tile_flipped[0],
        nparray_first_tile_flipped[-1],
        nparray_first_tile_flipped[:, 0],
        nparray_first_tile_flipped[:, -1],
    ]
    second_tile_faces = [
        tile_two.data[0],
        tile_two.data[-1],
        tile_two.data[:, 0],
        tile_two.data[:, -1],
    ]

    matches = [
        face_one
        for face_one, face_two in itertools.product(first_tile_faces, second_tile_faces)
        if np.array_equal(face_one, face_two)
    ]
    return len(matches)
    # for face in first_tile:
    #     pass


def test_count_edge_match():
    tile_one = Tile(0, np.array([[0, 1], [2, 3]], dtype=object))
    tile_two = Tile(1, np.array([[0, 1], [4, 5]], dtype=object))
    assert count_edge_matches(tile_one, tile_two) == 1


def is_face_matches_tile(face: np.ndarray, tile: Tile):
    # determine whether face matches anywhere on tile,
    # including after rotating and flipping tile
    assert type(face) is np.ndarray
    assert type(tile) is Tile

    tile_faces = [
        tile.data[0],
        tile.data[-1],
        tile.data[:, 0],
        tile.data[:, -1],
    ]
    nparray_tile_flipped = np.flip(tile.data)
    tile_faces += [
        nparray_tile_flipped[0],
        nparray_tile_flipped[-1],
        nparray_tile_flipped[:, 0],
        nparray_tile_flipped[:, -1],
    ]

    matches = [
        face_one
        for face_one, face_two in itertools.product([face], tile_faces)
        if np.array_equal(face_one, face_two)
    ]
    return bool(len(matches))


def is_face_matches_face(face_one: np.ndarray, face_two: np.ndarray):
    assert type(face_one) is np.ndarray
    assert type(face_two) is np.ndarray

    # with flipping
    # result = np.array_equal(face_one, face_two) or np.array_equal(
    #     face_one, np.flip(face_two)
    # )

    # without flipping
    result = np.array_equal(face_one, face_two)
    return result


def count_all_edge_matches(tile: Tile, tiles: List[Tile]):
    count = 0
    for candidate_tile in tiles:
        if tile.number == candidate_tile.number:
            # don't check a tile against itself
            continue
        # count the matching edges
        count += count_edge_matches(tile, candidate_tile)
        if DEBUG and count > 0:
            print(count)
    return count


def find_corner_pieces(tiles: List[Tile]):
    # count matching faces for every tile
    # corner tiles have only 2 matching faces
    # all other tiles have more
    corner_pieces = []
    for tile in tiles:
        match_count = count_all_edge_matches(tile, tiles)
        if match_count == 2:
            corner_pieces.append(tile)
    return corner_pieces


def find_next_match(known_tile: Tile, candidate_tiles: List[Tile]):
    assert type(known_tile) is Tile
    # from candidate_tiles, find a tile that has a matching edge with known_tile
    for candidate_tile in candidate_tiles:
        if count_edge_matches(known_tile, candidate_tile) > 0:
            return candidate_tile
    raise RuntimeError("Did not find a next match.")


def yield_next_match(known_tile: Tile, candidate_tiles: List[Tile]):
    assert type(known_tile) is Tile
    # from candidate_tiles, find a tile that has a matching edge with known_tile
    for candidate_tile in candidate_tiles:
        if count_edge_matches(known_tile, candidate_tile) > 0:
            yield candidate_tile


def product(values):
    # see also: math.prod
    ret = 1
    ret = [ret := ret * v for v in values][-1]
    return ret


def generate_nparray_orientation(npa: np.array):
    # generator to provide all orientations (rotations and flips) for 2-Dimensial np.array
    # Usage:
    #   for orientation in generate_nparray_orientation(candidate_nparray):
    #       print(orientation)
    orientations = [
        npa,
        np.rot90(npa, k=1),
        np.rot90(npa, k=2),
        np.rot90(npa, k=3),
    ]
    npa_flipped = np.flip(npa, axis=0)
    orientations += [
        npa_flipped,
        np.rot90(npa_flipped, k=1),
        np.rot90(npa_flipped, k=2),
        np.rot90(npa_flipped, k=3),
    ]
    yield from orientations


def generate_tile_orientation(tile: Tile):
    # generator to provide all orientations (rotations and flips) for tile
    # Usage:
    #   for orientation in generate_tile_orientation(candidate_tile):
    #       print(orientation)
    orientations = [
        tile.data,
        np.rot90(tile.data, k=1),
        np.rot90(tile.data, k=2),
        np.rot90(tile.data, k=3),
    ]
    tile_data_flipped = np.flip(tile.data, axis=0)
    orientations += [
        tile_data_flipped,
        np.rot90(tile_data_flipped, k=1),
        np.rot90(tile_data_flipped, k=2),
        np.rot90(tile_data_flipped, k=3),
    ]
    yield from orientations


def yield_next_solution(solution: List[List[Tile]], position: Tuple):
    print(solution)
    print(position)


def test_yield_next_solution():
    yield_next_solution("solution string", "position string")


def is_tile_matches_neighbors(index_y: int, index_x: int, solution: List[List[Tile]]):
    """Neighbors can be None"""
    if solution[index_y][index_x] is None:
        return True
    # Up
    if index_y > 0:
        if solution[index_y - 1][index_x]:
            tile_face_up = solution[index_y][index_x].data[0]
            neighbor_face_down = solution[index_y - 1][index_x].data[-1]
            if not is_face_matches_face(tile_face_up, neighbor_face_down):
                return False
    # Down
    if index_y < (len(solution) - 1):
        if solution[index_y + 1][index_x]:
            tile_face_down = solution[index_y][index_x].data[-1]
            neighbor_face_up = solution[index_y + 1][index_x].data[0]
            if not is_face_matches_face(tile_face_down, neighbor_face_up):
                return False
    # Left
    if index_x > 0:
        if solution[index_y][index_x - 1]:
            tile_face_left = solution[index_y][index_x].data[:, 0]
            neighbor_face_right = solution[index_y][index_x - 1].data[:, -1]
            if not is_face_matches_face(tile_face_left, neighbor_face_right):
                return False
    # Right
    if index_x < (len(solution[0]) - 1):
        if solution[index_y][index_x + 1]:
            tile_face_right = solution[index_y][index_x].data[:, -1]
            neighbor_face_left = solution[index_y][index_x + 1].data[:, 0]
            if not is_face_matches_face(tile_face_right, neighbor_face_left):
                return False
    return True


def is_partial_solution_valid(solution: List[List[Tile]]):
    # Check a partial solution. None is allowed where a Tile has not been placed yet.
    for y_index in range(len(solution)):
        for x_index in range(len(solution[0])):
            if solution[y_index][x_index] is None:
                continue
            if not is_tile_matches_neighbors(y_index, x_index, solution):
                return False
    return True


def repr_solution_tiles(solution: List[List[Tile]]) -> str:
    s = ""
    for y_index, solution_row in enumerate(solution):
        for y_tile_index in range(len(solution[0][0].data)):
            for x_index, tile in enumerate(solution_row):
                if solution[y_index][x_index]:
                    s += "".join(solution[y_index][x_index].data[y_tile_index])
                    s += " "
                else:
                    s += "-" * len(solution[0][0].data[0])
                    s += " "
            s += "\n"
        s += "\n"
    return s


def repr_solution(solution: List[List[Tile]]) -> str:
    s = ""
    for y_index, solution_row in enumerate(solution):
        for y_tile_index in range(1, len(solution[0][0].data) - 1):
            for x_index, tile in enumerate(solution_row):
                if solution[y_index][x_index]:
                    s += "".join(solution[y_index][x_index].data[y_tile_index][1:-1])
                else:
                    s += "-" * len(solution[0][0].data[0][1:-1])
            s += "\n"
    return s


def list_str_solution(solution: List[List[Tile]]) -> List[str]:
    lines = []
    for y_index, solution_row in enumerate(solution):
        for y_tile_index in range(1, len(solution[0][0].data) - 1):
            line = ""
            for x_index, tile in enumerate(solution_row):
                if solution[y_index][x_index]:
                    line += "".join(solution[y_index][x_index].data[y_tile_index][1:-1])
                else:
                    line += "-" * len(solution[0][0].data[0][1:-1])
            lines.append(line)
    return lines


def match_2d(pattern_2d: np.array, string_2d: np.array):
    matches = []
    for y_index in range(len(string_2d) - len(pattern_2d) + 1):
        for x_index in range(len(string_2d[0]) - len(pattern_2d[0]) + 1):
            next_candidate = False
            candidate_str = string_2d[
                y_index : y_index + len(pattern_2d),
                x_index : x_index + len(pattern_2d[0]),
            ]
            for y_candidate in range(len(pattern_2d)):
                for x_candidate in range(len(pattern_2d[0])):
                    # only looking for "#" in pattern_2d
                    if pattern_2d[y_candidate][x_candidate] != "#":
                        continue
                    if (
                        pattern_2d[y_candidate][x_candidate]
                        != candidate_str[y_candidate][x_candidate]
                    ):
                        next_candidate = True
                        break
                    else:
                        continue
                if next_candidate:
                    break
            if not next_candidate:
                matches.append((y_index, x_index))
    return matches


def test_match_2d():
    monster = ["                  # ", "#    ##    ##    ###", " #  #  #  #  #  #   "]
    sea = ["# .               # ", "#    ##    ##    ###", " #  #  #  #  #  #   "]

    matches = match_2d(monster, monster)
    assert matches == [(0, 0)]
    matches = match_2d(monster, sea)
    assert matches == [(0, 0)]


def flipud_2d_str(string_2d: List[str]) -> List[str]:
    return string_2d[::-1]


def test_flipud_2d_str():
    s = flipud_2d_str(["12", "34"])
    assert s == ["34", "12"]


def list_str_to_nparray(list_str: List[str]) -> np.array:
    return np.array([[c for c in s] for s in list_str])


if __name__ == "__main__":
    if USE_EXAMPLE1:
        filename = "./AdventOfCode/2020/day20-example1-input.txt"
    else:
        filename = "./AdventOfCode/2020/day20-input.txt"

    with open(filename) as f:
        tiles = f.read().split("\n\n")
        # lines = [line.rstrip() for line in f]
    for t_index, candidate_tile in enumerate(tiles):
        candidate_tile = candidate_tile.split("\n")
        number = int(candidate_tile[0].split()[1][:-1])
        data = np.array([[char for char in row] for row in candidate_tile[1:]])
        tiles[t_index] = Tile(number, data)
    orig_tiles = tiles.copy()
    if DEBUG:
        pprint(tiles)
    print(f"Loaded {len(tiles)} tiles")
    print(f"Each tile is {len(tiles[0].data)} rows, {len(tiles[0].data[0])} columns")

    # np array rotations
    # https://numpy.org/doc/stable/reference/generated/numpy.rot90.html#numpy.rot90
    # print(tiles[0].data)
    # print(np.rot90(tiles[0].data))  # rotate counter clockwise
    # print(np.rot90(tiles[0].data, axes=(1,0)))  # rotate clockwise
    # print(np.rot90(tiles[0].data, k=0))  # rotate counter clockwise 0 times
    # note that rotations return views, not new arrays
    # flip too, not just rotate

    # Part 1
    # find the corners by counting the matching edges of each tile.
    # corners have only two matching edges

    corners = find_corner_pieces(tiles)
    corner_ids = [corner.number for corner in corners]
    part1 = product(corner_ids)
    print(f"Part 1 {part1}")  # 68781323018729
    if USE_EXAMPLE1:
        assert part1 == 20899048083289
    else:
        assert part1 == 68781323018729

    # Part 2
    dimension = math.isqrt(len(tiles))
    solution = [[None for _ in range(dimension)] for _ in range(dimension)]
    # print(solution)

    assert is_partial_solution_valid(solution)

    # start the solution with one of the corners found previously
    solution[0][0] = corners[0]  # can be flipped/rotated
    # tiles will only hold tiles that are not in solution yet
    tiles.remove(corners[0])
    # print(solution)
    assert is_partial_solution_valid(solution)

    # place solution[0][1]
    #   find a matching tile
    candidate_tile = find_next_match(solution[0][0], tiles)
    # print(f"candidate_tile: {candidate_tile}")
    #   orient the corner. Which face matches?
    # Options
    # 1. could make this a tuple that also carries the "index" for how to rotate
    # 2. or carries the rotated tile with each face
    # 3. or just send the rotations and check the desired face below
    # tile_faces = [
    #     tile.data[0],  # top
    #     tile.data[-1],  # bottom
    #     tile.data[:, 0],  # left
    #     tile.data[:, -1],  # right
    # ]
    # tile_rotations = [
    #     tile.data,
    #     np.rot90(tile.data, k=1),
    #     np.rot90(tile.data, k=2),
    #     np.rot90(tile.data, k=3),
    # ]
    # for face in tile_faces:
    #     if is_edge_match(face, candidate_tile):
    #         print(f"Face {face} matched candidate {candidate_tile}")

    # in tile_rotations we are looking for the right face to match
    # for orientation in tile_rotations:
    y_index = 0
    x_index = 0
    for orientation in generate_tile_orientation(solution[y_index][x_index]):
        if is_face_matches_tile(orientation[:, -1], candidate_tile):
            solution[y_index][x_index] = Tile(
                solution[y_index][x_index].number, orientation
            )
            # print("matched orientation")
            break

    assert is_partial_solution_valid(solution)

    # orient the candidate match and place it
    for orientation in generate_tile_orientation(candidate_tile):
        # compare left face of solved tile to right face of candidate_tile in all possible orientations
        if is_face_matches_face(
            solution[y_index][x_index].data[:, -1], orientation[:, 0]
        ):
            # print(f"Placing candidate tile {candidate_tile.number}")
            solution[y_index][x_index + 1] = Tile(candidate_tile.number, orientation)
            # remove the matching candidate from tiles
            tiles.remove(candidate_tile)
            break

    assert is_partial_solution_valid(solution)

    y_index = 1
    x_index = 0
    candidate_tile = find_next_match(solution[y_index - 1][x_index], tiles)
    # does row 0 need to flip?
    # does candidate match to top or bottom of solution[0][0]?
    needs_flip = False
    # compare top face of solution[0][0] to candidate_tile
    up_neighbor = solution[0][0]
    if is_face_matches_tile(up_neighbor.data[0], candidate_tile):
        needs_flip = True
    if needs_flip:
        for x_index, tile in enumerate(solution[0]):
            if solution[0][x_index]:
                flipped_data = np.flipud(tile.data)  # flip up down
                solution[0][x_index] = Tile(tile.number, flipped_data)
    # orient candidate_tile to tile above
    # for orientation in orientation_generator(candidate_tile):
    #     if is_face_matches_tile(orientation[0], solution[0][0]):
    #         print(orientation[0])
    #     if is_face_matches_face(orientation[0], solution[0][0].data[-1]):
    #         print(orientation[0])

    for orientation in generate_tile_orientation(candidate_tile):
        if is_face_matches_face(up_neighbor.data[-1], orientation[0]):
            print(f"Placing candidate tile {candidate_tile.number}")
            solution[y_index][x_index] = Tile(candidate_tile.number, orientation)
            # remove candidate match from tiles
            tiles.remove(candidate_tile)
            break

    assert is_partial_solution_valid(solution)
    # after the first corner, and it's neighbors have been placed
    # the solution cannot be flipped

    # solve first row
    y_index = 0
    for x_index, tile in enumerate(solution[y_index]):
        if tile:
            continue
        # print(f"{x_index} {tile}")

        left_neighbor = solution[y_index][x_index - 1]
        for candidate_tile in yield_next_match(left_neighbor, tiles):
            # find the right orientation for candidate_tile to left_neighbor
            for orientation in generate_tile_orientation(candidate_tile):
                if is_face_matches_face(left_neighbor.data[:, -1], orientation[:, 0]):
                    # print(f"Placing candidate tile {candidate_tile.number}")
                    solution[y_index][x_index] = Tile(
                        candidate_tile.number, orientation
                    )
                    # remove candidate match from tiles
                    tiles.remove(candidate_tile)
                    break

            if solution[y_index][x_index] is not None:
                break

        assert solution[y_index][x_index] is not None
        assert is_partial_solution_valid(solution)
    # print(f"Solution:\n{solution}")
    # print(repr_solution(solution))
    assert is_partial_solution_valid(solution)
    # print()

    # solve other rows. if the left neighbor is empty or we are on the left edge of solution,
    # look up to place tile
    for y_index, solution_row in enumerate(solution):
        for x_index, tile in enumerate(solution[y_index]):
            if tile:
                continue
            if x_index > 0:
                # we are not on left edge of solution
                left_neighbor = solution[y_index][x_index - 1]
                for candidate_tile in yield_next_match(left_neighbor, tiles):
                    # find the right orientation for candidate_tile to left_neighbor
                    # and to up_neighbor
                    for orientation in generate_tile_orientation(candidate_tile):
                        if is_face_matches_face(
                            left_neighbor.data[:, -1], orientation[:, 0]
                        ):
                            # print(f"Placing candidate tile {candidate_tile.number}")
                            solution[y_index][x_index] = Tile(
                                candidate_tile.number, orientation
                            )
                            if not is_partial_solution_valid(solution):
                                # keep trying orientations
                                continue
                            # this is the right orientation with all neighbors
                            # remove candidate match from tiles
                            tiles.remove(candidate_tile)
                            break
                    if solution[y_index][x_index] is not None:
                        break
                assert solution[y_index][x_index] is not None
                assert is_partial_solution_valid(solution)
            elif x_index == 0:
                # on left edge of solution, look at up neighbor
                up_neighbor = solution[y_index - 1][x_index]
                for candidate_tile in yield_next_match(up_neighbor, tiles):
                    for orientation in generate_tile_orientation(candidate_tile):
                        if is_face_matches_face(up_neighbor.data[-1], orientation[0]):
                            # print(f"Placing candidate tile {candidate_tile.number}")
                            solution[y_index][x_index] = Tile(
                                candidate_tile.number, orientation
                            )
                            if not is_partial_solution_valid(solution):
                                # keep trying orientations
                                continue
                            # remove candidate match from tiles
                            tiles.remove(candidate_tile)
                            break
                    if solution[y_index][x_index] is not None:
                        break
                assert solution[y_index][x_index] is not None
                assert is_partial_solution_valid(solution)

    print(repr_solution_tiles(solution))
    str_solution = repr_solution(solution)
    print(str_solution)

    monster = ["                  # ", "#    ##    ##    ###", " #  #  #  #  #  #   "]
    nparray_monster = list_str_to_nparray(monster)

    # need to rotate and flip str_solution to get matches

    nparray_solution = list_str_to_nparray(list_str_solution(solution))
    print(nparray_solution)

    # matches = match_2d(monster, list_str_solution(solution))
    # print(matches)

    for orientation in generate_nparray_orientation(nparray_solution):
        matches = match_2d(nparray_monster, orientation)
        if len(matches) > 0:
            break
    print(orientation)
    print(matches)

    # count "#" minus (count "#" in monster * len(matches))
    pound_in_orientation = len(
        [char for row in orientation for char in row if char == "#"]
    )
    pound_in_monster = len(
        [char for row in nparray_monster for char in row if char == "#"]
    )
    part2 = pound_in_orientation - (len(matches) * pound_in_monster)
    print(part2)
    assert part2 == 1629
