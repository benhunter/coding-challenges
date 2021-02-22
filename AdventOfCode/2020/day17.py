# Advent of Code 2020 Day 17
# https://adventofcode.com/2020/17


from copy import deepcopy
from pprint import pformat
from typing import DefaultDict


DEBUG = False


class ConwayState:
    STAY_ACTIVE_RULE = [2, 3]
    BECOME_ACTIVE_RULE = [3]

    def __init__(self, filename):
        with open(filename) as f:
            initial_state_xy = [line.rstrip() for line in f]
        if DEBUG:
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
                str_state += f"{index_y} {val_y}\n"
            str_state += "\n"
        return str_state

    def _is_needtogrow(self):
        # does the x,y,z box need to expand?
        # return True if there is an active cube on an edge

        # check bottom
        for index_y, val_y in enumerate(self._state[self._z[0]]):
            for index_x in range(abs(self._x[0] - self._x[1])):
                if val_y[index_x] == "#":
                    return True
        # check top
        for index_y, val_y in enumerate(self._state[self._z[1] - 1]):
            for index_x in range(abs(self._x[0] - self._x[1])):
                if val_y[index_x] == "#":
                    return True
        # check x and y sides (left and right, top and bottom) for middle layers
        for index_z in range(self._z[0] + 1, self._z[1] - 2):
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
        if DEBUG:
            print(f"Growing: currently x:{self._x} y:{self._y} z:{self._z}")
            # print(self)
            print("Now growing...")
        self._history.append(self._state.copy())
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
        if DEBUG:
            print("Stepping")
        new_state = deepcopy(self._state)
        for index_z in range(self._z[0], self._z[1]):
            if DEBUG:
                print(f"index_z {index_z}")
            for index_y in range(abs(self._y[0] - self._y[1])):
                if DEBUG:
                    print(f"index_y {index_y} ", end="")
                for index_x in range(abs(self._x[0] - self._x[1])):
                    neighbor_count = self._count_neighbors(index_x, index_y, index_z)

                    if DEBUG:
                        pass
                        # print(
                        #     f"index: x:{index_x} y:{index_y} z:{index_z} state: {new_state[index_z][index_y][index_x]} neighbors: {neighbor_count}"
                        # )
                    if new_state[index_z][index_y][index_x] == "#":
                        if neighbor_count in ConwayState.STAY_ACTIVE_RULE:
                            # Stay alive
                            if DEBUG:
                                print(f"#", end="")
                            continue
                        else:
                            # Kill
                            new_state[index_z][index_y] = (
                                new_state[index_z][index_y][:index_x]
                                + "."
                                + new_state[index_z][index_y][index_x + 1 :]
                            )
                            if DEBUG:
                                print(f".", end="")
                            # self._state[index_z][index_y][index_x] = "."
                    elif new_state[index_z][index_y][index_x] == ".":
                        if neighbor_count in ConwayState.BECOME_ACTIVE_RULE:
                            # Come alive
                            new_state[index_z][index_y] = (
                                new_state[index_z][index_y][:index_x]
                                + "#"
                                + new_state[index_z][index_y][index_x + 1 :]
                            )
                            if DEBUG:
                                print(f"#", end="")
                            # self._state[index_z][index_y][index_x] = "#"
                        else:
                            # Stay dead
                            if DEBUG:
                                print(f".", end="")
                if DEBUG:
                    print(f"")
        self._history.append(deepcopy(self._state))
        self._state = new_state
        if self._is_needtogrow():
            self._grow()

    def _count_neighbors(self, x, y, z):
        """x, y, z are indexes (meaning x and y are 0 based), not coordinates
            Counting here was really tricky to fix because originally I treated
        x and y as coordinates instead of calculating the absolute difference.
        (First code was using self._x[0] and _x[1] directly)
        """

        count = 0
        # z layer below
        if z > self._z[0]:
            for index_y, val_y in enumerate(
                self._state[z - 1][
                    max(0, y - 1) : min(abs(self._y[0] - self._y[1]), y + 2)
                ]
            ):
                count += val_y[max(0, x - 1) : min(len(val_y), x + 2)].count("#")
        # z layer above
        if z < self._z[1] - 1:
            for index_y, val_y in enumerate(
                self._state[z + 1][
                    max(0, y - 1) : min(abs(self._y[0] - self._y[1]), y + 2)
                ]
            ):
                count += val_y[max(0, x - 1) : min(len(val_y), x + 2)].count("#")
        # y line top, bottom
        if y > 0:
            count += self._state[z][y - 1][
                max(0, x - 1) : min(abs(self._x[0] - self._x[1]), x + 2)
            ].count("#")
        if y < abs(self._y[0] - self._y[1]) - 1:
            count += self._state[z][y + 1][
                max(0, x - 1) : min(abs(self._x[0] - self._x[1]), x + 2)
            ].count("#")
        # x left, right
        if x > 0 and self._state[z][y][x - 1] == "#":
            count += 1
        if x < abs(self._x[0] - self._x[1]) - 1 and self._state[z][y][x + 1] == "#":
            count += 1
        return count

    def count(self):
        count = 0
        for key_z, value_z in self._state.items():
            for value_y in value_z:
                count += value_y.count("#")
        return count


class ConwayStatePart2:
    STAY_ACTIVE_RULE = [2, 3]
    BECOME_ACTIVE_RULE = [3]

    def __init__(self, filename):
        with open(filename) as f:
            initial_state_xy = [line.rstrip() for line in f]
        if DEBUG:
            print(initial_state_xy)

        self._history = []

        # boundaries in x,y,z = [low, high], low is inclusive, high is exclusive
        self._x = [0, len(initial_state_xy[0])]
        self._y = [0, len(initial_state_xy)]
        self._z = [0, 1]
        self._w = [0, 1]

        z_generator = lambda: [
            "." * abs(self._x[0] - self._x[1])
            for y in range(abs(self._y[0] - self._y[1]))
        ]

        w_generator = lambda: DefaultDict(z_generator)
        self._state = DefaultDict(w_generator)  # W layers of Z layers
        self._state[0][0] = initial_state_xy

        # check before growing. Naive solution would be to always grow instead of checking.
        if self._is_needtogrow():
            self._grow()

    def __repr__(self):
        # return f"X: {self._x} Y: {self._y} Z: {self._z}\nState:\n" + pformat(self._state)
        # Iterating Z layers forces the z_generator to run if needed.
        str_state = f"X: {self._x} Y: {self._y} Z: {self._z} W: {self._w}\n"
        for index_w in range(self._w[0], self._w[1]):
            for index_z in range(self._z[0], self._z[1]):
                str_state += f"z={index_z} w={index_w}\n"
                for index_y, val_y in enumerate(self._state[index_w][index_z]):
                    str_state += f"{index_y} {val_y}\n"
                str_state += "\n"
        return str_state

    def _is_needtogrow(self):
        # does the x,y,z box need to expand?
        # return True if there is an active cube on an edge

        for index_w in range(self._w[0], self._w[1]):
            # check bottom
            for index_y, val_y in enumerate(self._state[index_w][self._z[0]]):
                for index_x in range(abs(self._x[0] - self._x[1])):
                    if val_y[index_x] == "#":
                        return True
            # check top
            for index_y, val_y in enumerate(self._state[index_w][self._z[1] - 1]):
                for index_x in range(abs(self._x[0] - self._x[1])):
                    if val_y[index_x] == "#":
                        return True
            # check x and y sides (left and right, top and bottom) for middle layers
            for index_z in range(self._z[0] + 1, self._z[1] - 2):
                for index_y, val_y in enumerate(self._state[index_w][index_z]):
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
        if DEBUG:
            print(f"Growing: currently x:{self._x} y:{self._y} z:{self._z}")
            # print(self)
            print("Now growing...")
        self._history.append(deepcopy(self._state))
        # update boundaries
        self._x = [self._x[0] - 1, self._x[1] + 1]
        self._y = [self._y[0] - 1, self._y[1] + 1]
        self._z = [self._z[0] - 1, self._z[1] + 1]
        self._w = [self._w[0] - 1, self._w[1] + 1]

        for index_w in range(self._w[0] + 1, self._w[1] - 1):
            for index_z in range(self._z[0] + 1, self._z[1] - 1):
                for index_y, val_y in enumerate(self._state[index_w][index_z]):
                    # expand existing x (left and right)
                    self._state[index_w][index_z][index_y] = (
                        "." + self._state[index_w][index_z][index_y] + "."
                    )

                # expand existing y (top and bottom)
                self._state[index_w][index_z].insert(
                    0, "." * abs(self._x[0] - self._x[1])
                )
                self._state[index_w][index_z].append("." * abs(self._x[0] - self._x[1]))

    def step(self):
        # iterate through every position
        # check the number of neighbors
        # activate based on the rules
        # build new state, don't modify the current
        if DEBUG:
            print("Stepping")
        new_state = deepcopy(self._state)
        for index_w in range(self._w[0], self._w[1]):
            for index_z in range(self._z[0], self._z[1]):
                if DEBUG:
                    print(f"index_z {index_z}")
                for index_y in range(abs(self._y[0] - self._y[1])):
                    if DEBUG:
                        print(f"index_y {index_y} ", end="")
                    for index_x in range(abs(self._x[0] - self._x[1])):
                        neighbor_count = self._count_neighbors(
                            index_x, index_y, index_z, index_w
                        )

                        if DEBUG:
                            pass
                            # print(
                            #     f"index: x:{index_x} y:{index_y} z:{index_z} state: {new_state[index_w][index_z][index_y][index_x]} neighbors: {neighbor_count}"
                            # )
                        if new_state[index_w][index_z][index_y][index_x] == "#":
                            if neighbor_count in ConwayState.STAY_ACTIVE_RULE:
                                # Stay alive
                                if DEBUG:
                                    print(f"#", end="")
                                continue
                            else:
                                # Kill
                                new_state[index_w][index_z][index_y] = (
                                    new_state[index_w][index_z][index_y][:index_x]
                                    + "."
                                    + new_state[index_w][index_z][index_y][
                                        index_x + 1 :
                                    ]
                                )
                                if DEBUG:
                                    print(f".", end="")
                                # self._state[index_w][index_z][index_y][index_x] = "."
                        elif new_state[index_w][index_z][index_y][index_x] == ".":
                            if neighbor_count in ConwayState.BECOME_ACTIVE_RULE:
                                # Come alive
                                new_state[index_w][index_z][index_y] = (
                                    new_state[index_w][index_z][index_y][:index_x]
                                    + "#"
                                    + new_state[index_w][index_z][index_y][
                                        index_x + 1 :
                                    ]
                                )
                                if DEBUG:
                                    print(f"#", end="")
                                # self._state[index_w][index_z][index_y][index_x] = "#"
                            else:
                                # Stay dead
                                if DEBUG:
                                    print(f".", end="")
                    if DEBUG:
                        print(f"")
        self._history.append(deepcopy(self._state))
        self._state = new_state
        if self._is_needtogrow():
            self._grow()

    def _count_w_neighbors(self, x, y, z, w):
        count = 0
        # z layer below
        if z > self._z[0]:
            for index_y, val_y in enumerate(
                self._state[w][z - 1][
                    max(0, y - 1) : min(abs(self._y[0] - self._y[1]), y + 2)
                ]
            ):
                count += val_y[max(0, x - 1) : min(len(val_y), x + 2)].count("#")
        # z layer above
        if z < self._z[1] - 1:
            for index_y, val_y in enumerate(
                self._state[w][z + 1][
                    max(0, y - 1) : min(abs(self._y[0] - self._y[1]), y + 2)
                ]
            ):
                count += val_y[max(0, x - 1) : min(len(val_y), x + 2)].count("#")
        # y line top, bottom
        if y > 0:
            count += self._state[w][z][y - 1][
                max(0, x - 1) : min(abs(self._x[0] - self._x[1]), x + 2)
            ].count("#")
        if y < abs(self._y[0] - self._y[1]) - 1:
            count += self._state[w][z][y + 1][
                max(0, x - 1) : min(abs(self._x[0] - self._x[1]), x + 2)
            ].count("#")
        # x left, right
        if x > 0 and self._state[w][z][y][x - 1] == "#":
            count += 1
        if x < abs(self._x[0] - self._x[1]) - 1 and self._state[w][z][y][x + 1] == "#":
            count += 1
        return count

    def _count_neighbors(self, x, y, z, w):
        """x, y, z are indexes (meaning x and y are 0 based), not coordinates
            Counting here was really tricky to fix because originally I treated
        x and y as coordinates instead of calculating the absolute difference.
        (First code was using self._x[0] and _x[1] directly)
        """

        count = 0
        for index_w in range(w - 1, w + 2):
            count += self._count_w_neighbors(x, y, z, index_w)
            if index_w != w:
                count += self._state[index_w][z][y][x].count("#")
        return count

    def count(self):
        count = 0
        for key_w, value_w in self._state.items():
            for key_z, value_z in value_w.items():
                for value_y in value_z:
                    count += value_y.count("#")
        return count


if __name__ == "__main__":
    filename = "./AdventOfCode/2020/day17-input.txt"
    # filename = "./AdventOfCode/2020/day17-example1-input.txt"
    # filename = "./AdventOfCode/2020/day17-example2-input.txt"

    state = ConwayState(filename)
    if DEBUG:
        print(state)

    for i in range(6):
        if DEBUG:
            print(f"Start Step {i + 1}:")
        state.step()
        if DEBUG:
            print(state)
            print(f"Count: {state.count()}")
            print(f"End Step {i + 1}")

    part1 = state.count()
    print(f"Part 1: {part1}")  # 401

    state2 = ConwayStatePart2(filename)
    if DEBUG:
        print(state2)

    for i in range(6):
        if DEBUG:
            print(f"Start Step {i + 1}:")
        state2.step()
        if DEBUG:
            print(state2)
            print(f"Count: {state2.count()}")
            print(f"End Step {i + 1}")

    part2 = state2.count()
    print(f"Part 2: {part2}")  # 2224
