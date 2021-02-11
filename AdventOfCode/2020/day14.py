# Advent of Code Day 14
# https://adventofcode.com/2020/day/14


class State:
    # Singleton with class variables
    memory = dict()  # use defaultdict instead?
    mask = 0

    def sum_mem(self):
        return sum([int(x, 2) for x in State.memory.values()])


if __name__ == "__main__":
    filename = ".\\AdventOfCode\\2020\\day14-input.txt"
    # filename = ".\\AdventOfCode\\2020\\day14-example1-input.txt"

    with open(filename) as f:
        operations = [line.rstrip() for line in f]
    # print(operations)
    state = State()

    for op in operations:
        # print(op)
        op = op.split()
        operation = op[0]
        operand = op[-1]

        if operation == "mask":
            state.mask = operand
        elif operation[:3] == "mem":
            operand = bin(int(operand))[2:].rjust(36, "0")
            # print(f"operand: {operand}")

            # need address from operation
            address = int(operation.split("[")[-1].split("]")[0])
            # print(f"address: {address}")

            # overlay mask
            masked = "".join(
                [
                    x if state.mask[i] == "X" else state.mask[i]
                    for i, x in enumerate(operand)
                ]
            )
            # print(f"masked: {masked}")
            state.memory[address] = masked

    print(f"Part 1: {state.sum_mem()}")
