# Advent of Code 2020 Day
# https://adventofcode.com/2020/


import itertools
import math
import numpy as np

from pprint import pformat, pprint
from collections import namedtuple


DEBUG = False


Tile = namedtuple("Tile", "number, data")
# Tile.__repr__ = lambda self: pformat(data)


def count_edge_match(tile_one, tile_two):
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
        # if product[0] == product[1]
    ]
    return len(matches)
    # for face in first_tile:
    #     pass


def test_count_edge_match():
    tile_one = Tile(0, np.array([[0, 1], [2, 3]], dtype=object))
    tile_two = Tile(1, np.array([[0, 1], [4, 5]], dtype=object))
    assert count_edge_match(tile_one, tile_two) == 1


def count_all_edge_matches(tile, tiles):
    count = 0
    for candidate_tile in tiles:
        if tile.number == candidate_tile.number:
            # don't check a tile against itself
            continue
        # count the matching edges
        count += count_edge_match(tile, candidate_tile)
        if DEBUG and count > 0:
            print(count)
    return count


def find_corner_pieces(tiles):
    # count matching faces for every tile
    # corner tiles have only 2 matching faces
    # all other tiles have more
    corner_pieces = []
    for tile in tiles:
        match_count = count_all_edge_matches(tile, tiles)
        if match_count == 2:
            corner_pieces.append(tile)
    return corner_pieces


def product(values):
    # see also: math.prod
    ret = 1
    ret = [ret := ret * v for v in values][-1]
    return ret


if __name__ == "__main__":
    filename = "./AdventOfCode/2020/day20-input.txt"
    # filename = "./AdventOfCode/2020/day20-example1-input.txt"

    with open(filename) as f:
        tiles = f.read().split("\n\n")
        # lines = [line.rstrip() for line in f]
    for t_index, candidate_tile in enumerate(tiles):
        candidate_tile = candidate_tile.split("\n")
        number = int(candidate_tile[0].split()[1][:-1])
        data = np.array([[char for char in row] for row in candidate_tile[1:]])
        tiles[t_index] = Tile(number, data)
    orig_tiles = tiles.copy()
    # pprint(tiles)
    print(f"Loaded {len(tiles)} tiles")
    print(f"Each tile is {len(tiles[0].data)} rows, {len(tiles[0].data[0])} columns")

    # np array rotations
    # https://numpy.org/doc/stable/reference/generated/numpy.rot90.html#numpy.rot90
    print(tiles[0].data)
    # print(np.rot90(tiles[0].data))  # rotate counter clockwise
    # print(np.rot90(tiles[0].data, axes=(1,0)))  # rotate clockwise
    # print(np.rot90(tiles[0].data, k=0))  # rotate counter clockwise 0 times
    # note that rotations return views, not new arrays

    # TODO flip too, not just rotate

    dimension = math.isqrt(len(tiles))
    solution = [[None for _ in range(dimension)] for _ in range(dimension)]
    print(solution)

    solution[0][0] = tiles[0]
    print(solution)

    # test_count_edge_match()
    corners = find_corner_pieces(tiles)
    corner_ids = [corner.number for corner in corners]
    part1 = product(corner_ids)
    print(f"Part 1 {part1}")  # 68781323018729
    assert part1 == 68781323018729

    # while len(tiles) > 0:
    # for tile_index, candidate_tile in enumerate(tiles[1:]):
    #     for solved_row_idx, solved_row in enumerate(solution):
    #         for solved_column_idx, solved_tile in enumerate(solved_row):
    #             if not solved_tile:
    #                 continue
    #             orientations = [
    #                 np.rot90(candidate_tile.data, k=k) for k in range(4)
    #             ]  # rotations
    #             orientations += [
    #                 np.flip(candidate_tile.data, axis=axis) for axis in [None, 0, 1]
    #             ]  # flips
    #             orientations += [
    #                 np.flip(np.rot90(candidate_tile.data, k=1), axis=axis)
    #                 for axis in [None, 0, 1]
    #             ]  # flip the first rotation
    #             for (
    #                 candidate_orientation
    #             ) in orientations:  # if the face doesn't have an adjacent yet
    #                 for solved_face in solved_tile:
    #                     if match():
    #                         place(solution, solved_tile)
    #                         tiles.pop(
    #                             tile_index
    #                         )  # remove placed tiles from the lists of tiles
