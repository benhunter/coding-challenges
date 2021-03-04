# Advent of Code 2020 Day
# https://adventofcode.com/2020/


import itertools
import math
import numpy as np

from pprint import pformat, pprint
from collections import namedtuple


DEBUG = False


Tile = namedtuple("Tile", "number, data")
Tile.__repr__ = lambda self: f"Tile: {self.number}\n{pformat(self.data)}"


def count_edge_matches(tile_one, tile_two):
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


def is_face_matches_face(face_one, face_two):
    assert type(face_one) is np.ndarray
    assert type(face_two) is np.ndarray

    # with flipping
    # result = np.array_equal(face_one, face_two) or np.array_equal(
    #     face_one, np.flip(face_two)
    # )

    # without flipping
    result = np.array_equal(face_one, face_two)
    return result


def count_all_edge_matches(tile, tiles):
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


def find_next_match(tile, tiles):
    for candidate_tile in tiles:
        if count_edge_matches(tile, candidate_tile) > 0:
            return candidate_tile
    raise RuntimeError("Did not find a next match.")


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
    # flip too, not just rotate

    # Part 1
    # find the corners by counting the matching edges of each tile.
    # corners have only two matching edges

    corners = find_corner_pieces(tiles)
    corner_ids = [corner.number for corner in corners]
    part1 = product(corner_ids)
    print(f"Part 1 {part1}")  # 68781323018729
    assert part1 == 68781323018729

    # Part 2
    dimension = math.isqrt(len(tiles))
    solution = [[None for _ in range(dimension)] for _ in range(dimension)]
    print(solution)

    # start the solution with one of the corners found previously
    solution[0][0] = corners[0]  # can be flipped/rotated
    # tiles will only hold tiles that are not in solution yet
    tiles.remove(corners[0])
    print(solution)

    # place solution[0][1]
    #   find a matching tile
    candidate_tile = find_next_match(solution[0][0], tiles)
    print(f"candidate_tile: {candidate_tile}")
    #   orient the corner. Which face matches?

    tile = solution[0][0]

    # Options
    # 1. could make this a tuple that also carries the "index" for how to rotate
    # 2. or carries the rotated tile with each face
    # 3. or just send the rotations and check the desired face below
    tile_faces = [
        tile.data[0],  # top
        tile.data[-1],  # bottom
        tile.data[:, 0],  # left
        tile.data[:, -1],  # right
    ]
    tile_rotations = [
        tile.data,
        np.rot90(tile.data, k=1),
        np.rot90(tile.data, k=2),
        np.rot90(tile.data, k=3),
    ]
    # for face in tile_faces:
    #     if is_edge_match(face, candidate_tile):
    #         print(f"Face {face} matched candidate {candidate_tile}")

    # in tile_rotations we are looking for the right face to match
    for orientation in tile_rotations:
        if is_face_matches_tile(orientation[:, -1], candidate_tile):
            # tile.data = rotation
            tile = Tile(tile.number, orientation)
    #   orient the candidate match and place it
    candidate_rotations = [
        candidate_tile.data,
        np.rot90(candidate_tile.data, k=1),
        np.rot90(candidate_tile.data, k=2),
        np.rot90(candidate_tile.data, k=3),
    ]
    candidate_flipped = np.flip(candidate_tile.data, axis=0)
    candidate_flip_rotations = [
        candidate_flipped,
        np.rot90(candidate_flipped, k=1),
        np.rot90(candidate_flipped, k=2),
        np.rot90(candidate_flipped, k=3),
    ]
    candidate_orientations = candidate_rotations + candidate_flip_rotations

    for orientation in candidate_orientations:
        if is_face_matches_face(tile.data[:, -1], orientation[:, 0]):
            print(f"Placing candidate tile {candidate_tile.number}")
            solution[0][1] = Tile(candidate_tile.number, orientation)
            # remove candidate match from tiles
            tiles.remove(candidate_tile)
            break


    print(f"Solution:\n{solution}")

    # Idea for placing tiles
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
