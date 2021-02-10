# Advent of Code Day 13
# https://adventofcode.com/2020/day/13


if __name__ == "__main__":
    # filename = ".\\AdventOfCode\\2020\\day13-input.txt"
    filename = ".\\AdventOfCode\\2020\\day13-example1-input.txt"

    with open(filename) as f:
        earliest_time = int(f.readline())
        bus_schedule = f.readline().split(",")
    buses = [int(i) for i in bus_schedule if i != "x"]
    bus_schedule = [int(i) if i != "x" else "x" for i in bus_schedule]

    # print(earliest_time, buses)

    # found_bus = False
    # depart_time = earliest_time
    # earliest_bus = 0
    # while not found_bus:
    #     for bus in buses:
    #         if depart_time % bus == 0:
    #             print(f"Found bus: {bus} Depart Time: {depart_time}")
    #             found_bus = True
    #             earliest_bus = bus
    #     if not found_bus:
    #         depart_time += 1

    # # print(bus)
    # print(f"Part 1: {earliest_bus * (depart_time - earliest_time)}")

    # Part 2
    print("Calculating Part 2")
    # Brute Force Solution
    # sequence_found = False
    # time = buses[0]
    # # time = 1068781  # example 1 solution
    # while not sequence_found:
    #     partial_sequence = False
    #     sequence_time = time
    #     for bus in bus_schedule:
    #         if bus == "x":
    #             sequence_time += 1
    #             continue
    #         if sequence_time % bus == 0:
    #             partial_sequence = True
    #             sequence_time += 1
    #         else:
    #             partial_sequence = False
    #             break

    #     sequence_found = partial_sequence
    #     if not sequence_found:
    #         time += buses[0]

    #     if time % 1000000 == 0:
    #         print(time)

    # Chinese Remainder Theorem Sieve
    time = buses[0]
    period = buses[0]

    for offset, bus in enumerate(bus_schedule):
        if bus == "x":
            continue
        print(f"offset, bus: {offset, bus}")

        while (time + offset) % bus != 0:
            time += period
        print(f"Found solution at time: {time} bus:{bus} (time + offset) % bus: {(time + offset) % bus}")
        # after leaving while loop
        period = period * bus
        print(f"New period: {period}")

    print(f"Part 2: ")
    print(f"Time {time}")  # 6055379026300640 too large
