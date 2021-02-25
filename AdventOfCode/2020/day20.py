# Advent of Code 2020 Day
# https://adventofcode.com/2020/


import math
import numpy as np

from pprint import pformat, pprint
from collections import namedtuple

Tile = namedtuple("Tile", "number, data")
# Tile.__repr__ = lambda self: pformat(data)

if __name__ == "__main__":
    # filename = "./AdventOfCode/2020/day20-input.txt"
    filename = "./AdventOfCode/2020/day20-example1-input.txt"

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

    # while len(tiles) > 0:
    for tile_index, candidate_tile in enumerate(tiles[1:]):
        for solved_row_idx, solved_row in enumerate(solution):
            for solved_column_idx, solved_tile in enumerate(solved_row):
                if not solved_tile:
                    continue
                orientations = [np.rot90(candidate_tile.data, k=i) for i in range(4)]  # rotations 
                orientations += [np.flip]  # flips
                for candidate_orientation in orientations:  # if the face doesn't have an adjacent yet
                    for solved_face in solved_tile:
                        if match():
                            place(solution, solved_tile)
                            tiles.pop(tile_index)  # remove placed tiles from the lists of tiles
