# Advent of Code Day 14
# https://adventofcode.com/2020/day/14


from collections import defaultdict


class State:
    def __init__(self):
        self.memory = defaultdict(lambda: "0" * 36)
        self.mask = 0

    def sum_mem(self):
        return sum([int(x, 2) for x in self.memory.values()])

    def __repr__(self):
        str_mem = " ".join(
            [
                str(int(addr, 2)) + ":" + str(int(val, 2))
                for (addr, val) in self.memory.items()
            ]
        )
        return "State: memory: " + str_mem + " mask: " + self.mask


if __name__ == "__main__":
    filename = "./AdventOfCode/2020/day14-input.txt"
    # filename = "./AdventOfCode/2020/day14-example1-input.txt"
    # filename = "./AdventOfCode/2020/day14-example2-input.txt"

    with open(filename) as f:
        operations = [line.rstrip() for line in f]
    # print(operations)
    state = State()

    for op in operations:
        # print(op)
        op_split = op.split()
        operation = op_split[0]
        operand = op_split[-1]

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

    # Part 2
    state = State()

    for op in operations:
        op_split = op.split()
        operation = op_split[0]
        operand = op_split[-1]

        if operation == "mask":
            state.mask = operand
        elif operation[:3] == "mem":
            operand = bin(int(operand))[2:].rjust(36, "0")
            # print(f"operand: {operand}")

            # need address from operation
            address = int(operation.split("[")[-1].split("]")[0])
            bin_addr = bin(address)[2:].rjust(36, "0")
            # print(f"address: {address}")

            # overlay mask
            masked = "".join(
                [
                    "1"
                    if state.mask[i] == "1"
                    else (bin_addr[i] if state.mask[i] == "0" else state.mask[i])
                    for i, x in enumerate(operand)
                ]
            )
            # print(f"masked: {masked}")

            # generate floating addresses (anything with X)
            addresses = [""]
            for i, x in enumerate(masked):
                if x != "X":
                    for index_addr in range(len(addresses)):
                        addresses[index_addr] += x
                elif x == "X":
                    addresses.extend(addresses.copy())
                    for index_addr in range(len(addresses) // 2):
                        # first half
                        addresses[index_addr] += "0"
                    for index_addr in range(len(addresses) // 2, len(addresses)):
                        addresses[index_addr] += "1"

                # print(i, x)

            for addr in addresses:
                state.memory[addr] = operand
            # print(state)

    print(f"Part 2: {state.sum_mem()}")
