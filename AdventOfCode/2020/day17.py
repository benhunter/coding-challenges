# Advent of Code 2020 Day 17
# https://adventofcode.com/2020/17


from copy import deepcopy
from pprint import pformat
from typing import DefaultDict


class ConwayState:
    STAY_ACTIVE_RULE = [2, 3]
    BECOME_ACTIVE_RULE = [3]

    def __init__(self, filename):
        with open(filename) as f:
            initial_state_xy = [line.rstrip() for line in f]
        print(initial_state_xy)

        self._history = []

        # boundaries in x,y,z = [low, high], low is inclusive, high is exclusive
        self._x = [0, len(initial_state_xy[0])]
        self._y = [0, len(initial_state_xy)]
        self._z = [0, 1]

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
        # return f"X: {self._x} Y: {self._y} Z: {self._z}\nState:\n{pformat([self._state[z] for z in range(*self._z)])}"
        # return f"X: {self._x} Y: {self._y} Z: {self._z}\nState:\n{str([self._state[z] for z in range(*self._z)])}"
        str_state = f"X: {self._x} Y: {self._y} Z: {self._z}\n"
        for index_z in range(self._z[0], self._z[1]):
            str_state += f"z={index_z}\n"
            for index_y, val_y in enumerate(self._state[index_z]):
                str_state += f"{val_y}\n"
            str_state += "\n"
        return str_state

    def _is_needtogrow(self):
        # does the x,y,z box need to expand?
        # return True if there is an active cube on an edge
        self._history.append(self._state.copy())

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

        for index_z in range(self._z[0] + 1, self._z[1] - 1):
            for index_y, val_y in enumerate(self._state[index_z]):
                # expand existing x (left and right)
                self._state[index_z][index_y] = (
                    "." + self._state[index_z][index_y] + "."
                )

            # expand existing y (top and bottom)
            self._state[index_z].insert(0, "." * abs(self._x[0] - self._x[1]))
            self._state[index_z].append("." * abs(self._x[0] - self._x[1]))

    def step(self):
        # iterate through every position
        # check the number of neighbors
        # activate based on the rules
        # build new state, don't modify the current
        new_state = deepcopy(self._state)
        for index_z in range(self._z[0], self._z[1]):
            for index_y in range(abs(self._y[0] - self._y[1])):
                for index_x in range(abs(self._x[0] - self._x[1])):
                    neighbor_count = self._count_neighbors(index_x, index_y, index_z)
                    print(
                        f"index: {index_z} {index_y} {index_x} state: {self._state[index_z][index_y][index_x]} neighbors: {neighbor_count}"
                    )
                    if self._state[index_z][index_y][index_x] == "#":
                        if neighbor_count in ConwayState.STAY_ACTIVE_RULE:
                            # Stay alive
                            continue
                        else:
                            # Kill
                            new_state[index_z][index_y] = (
                                self._state[index_z][index_y][:index_x]
                                + "."
                                + self._state[index_z][index_y][index_x + 1 :]
                            )
                            # self._state[index_z][index_y][index_x] = "."
                    elif self._state[index_z][index_y][index_x] == ".":
                        if neighbor_count in ConwayState.BECOME_ACTIVE_RULE:
                            # Come alive
                            new_state[index_z][index_y] = (
                                self._state[index_z][index_y][:index_x]
                                + "#"
                                + self._state[index_z][index_y][index_x + 1 :]
                            )
                            # self._state[index_z][index_y][index_x] = "#"
        self._history.append(self._state.copy())
        self._state = new_state

    def _count_neighbors(self, x, y, z):
        """x, y, z are indexes (meaning x and y are 0 based), not coordinates"""

        # raise NotImplementedError  # TODO z below and above not working

        count = 0
        # z layer below, above
        if z > self._z[0]:
            for index_y, val_y in enumerate(self._state[z - 1][max(0, y - 1):min(self._y[1], y + 2)]):
                count += val_y[max(0, x - 1):min(len(val_y), x + 2)].count("#")
        if z < self._z[1] - 1:
            for index_y, val_y in enumerate(self._state[z + 1][max(0, y - 1):min(self._y[1], y + 2)]):
                count += val_y[max(0, x - 1):min(len(val_y), x + 2)].count("#")
        # y line top, bottom
        if y > self._y[0]:
            count += self._state[z][y - 1][max(0, x - 1):min(len(val_y), x + 2)].count("#")
        if y < self._y[1] - 1:
            count += self._state[z][y + 1][max(0, x - 1):min(len(val_y), x + 2)].count("#")
        # x left, right
        if x > self._x[0] and self._state[z][y][x - 1] == "#":
            count += 1
        if x < self._x[1] - 1 and self._state[z][y][x + 1] == "#":
            count += 1
        return count


if __name__ == "__main__":
    # filename = "./AdventOfCode/2020/day17-input.txt"
    filename = "./AdventOfCode/2020/day17-example1-input.txt"
    # filename = "./AdventOfCode/2020/day17-example2-input.txt"

    state = ConwayState(filename)
    print(state)

    state.step()
    print(state)