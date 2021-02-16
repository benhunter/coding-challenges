# Advent of Code 2020 Day 17
# https://adventofcode.com/2020/17


from pprint import pformat
from typing import DefaultDict


STAY_ACTIVE_RULE = [2, 3]
BECOME_ACTIVE_RULE = [3]


class ConwayState:
    def __init__(self, filename):
        with open(filename) as f:
            initial_state_xy = [line.rstrip() for line in f]
        print(initial_state_xy)

        # boundaries in x.y.z
        self._x = [0, len(initial_state_xy[0])]
        self._y = [0, len(initial_state_xy)]
        self._z = [-1, 2]

        z_generator = lambda: [
            "." * abs(self._x[0] - self._x[1])
            for y in range(abs(self._y[0] - self._y[1]))
        ]
        self._state = DefaultDict(z_generator)  # Z layers
        self._state[0] = initial_state_xy

        # check before growing. Naive solution would be to always grow instead of checking.
        if self._is_needtogrow():
            self._grow()

    def __repr__(self):
        # return f"X: {self._x} Y: {self._y} Z: {self._z}\nState:\n" + pformat(self._state)
        # Iterating Z layers forces the z_generator to run if needed.
        return f"X: {self._x} Y: {self._y} Z: {self._z}\nState:\n{pformat([self._state[z] for z in range(*self._z)])}"

    def _is_needtogrow(self):
        # does the x,y,z box need to expand?
        # yes, if there is an active cube on an edge

        # remove?
        # is_grow = False

        # check bottom
        for index_y, val_y in enumerate(self._state[self._z[0]]):
            for index_x in range(abs(self._x[0] - self._x[1])):
                if val_y[index_x] == "#":
                    return True
        # check top
        for index_y, val_y in enumerate(self._state[self._z[1]]):
            for index_x in range(abs(self._x[0] - self._x[1])):
                if val_y[index_x] == "#":
                    return True
        # check x and y sides (left and right, top and bottom) for middle layers
        for index_z in range(self._z[0] + 1, self._z[1] - 1):
            for index_y, val_y in enumerate(self._state[index_z]):
                for index_x in range(abs(self._x[0] - self._x[1])):
                    if (
                        index_x == self._x[0]
                        or index_x == self._x[1]
                        or index_y == self._y[0]
                        or index_y == self._y[1]
                    ):
                        if val_y[index_x] == "#":
                            return True
        return False

    def _grow(self):
        # update boundaries
        self._x = [self._x[0] - 1, self._x[1] + 1]
        self._y = [self._y[0] - 1, self._y[1] + 1]
        self._z = [self._z[0] - 1, self._z[1] + 1]

        # expand existing x,y (left and right, top and bottom)
        for index_z in range(self._z[0] + 1, self._z[1] - 1):
            for index_y, val_y in enumerate(self._state[index_z]):
                pass


def count_neighbors(state, coord):
    return 0


if __name__ == "__main__":
    # filename = "./AdventOfCode/2020/day17-input.txt"
    filename = "./AdventOfCode/2020/day17-example1-input.txt"

    state = ConwayState(filename)
    print(state)
