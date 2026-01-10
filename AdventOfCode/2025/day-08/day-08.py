import math
from itertools import accumulate, combinations
from pprint import pprint


def get(file="day-08.in"):
    with open(file) as f:
        return [line.rstrip() for line in f]

def coord(a):
    return list(map(int, a.split(',')))

def dist(a, b):
    # print(a, b)
    a = coord(a)
    b = coord(b)
    return (a[0]-b[0])**2 + (a[1] - b[1])**2 + (a[2] - b[2])**2

def part1(junctions, num_connections=1000):
    # print(len(junctions))
    dists = {}
    junctions = sorted(junctions)
    for a, b in combinations(junctions, 2):
        dists[a + ' ' + b] = dist(a, b)
        # print(a+b, dists[a+b])
    # print(dists.items())
    sorted_d = sorted(dists.items(), key=lambda x: x[1])
    pprint(sorted_d[:10])

    connections = []
    for pair in sorted_d[:num_connections]:
        connections.append([pair])
    connections = [c[0][0].split() for c in connections]
    pprint(connections)
    # print()

    circuit_count = 0
    circuits = {}
    for c in connections:
        # print(c, circuits)
        print(f"Connecting {c=}")

        if c[0] in circuits and c[1] in circuits:
            if circuits[c[0]] != circuits[c[1]]:
                print(f"found loop {c[0]=} {circuits[c[0]]=}, {c[1]=}, {circuits[c[1]]=}")
                # found a loop. Need to make all of the second loop on the first.
                for x in circuits.keys():
                    if circuits[x] == circuits[c[1]]:
                        circuits[x] = circuits[c[0]]
                print(f"fixed loop {c=} {circuits[c[0]]}, {circuits[c[1]]}")
            else:
                print(f"connection already in same circuit {circuits[c[0]]=} {circuits[c[1]]=}")
            break

        elif c[0] in circuits or c[1] in circuits:
            if circuits.get(c[0]) is not None:
                print(f"One is in! {c[0]=} {circuits[c[0]]=}")
                circuits[c[1]] = circuits[c[0]]
            elif circuits.get(c[1]) is not None:
                print(f"One is in! {c[1]=} {circuits[c[1]]=}")
                circuits[c[0]] = circuits[c[1]]
        else:
            circuits[c[0]] = circuit_count
            circuits[c[1]] = circuit_count
            circuit_count += 1
        print(f"connected {c=} {circuits[c[0]]}, {circuits[c[1]]}")

        if c[0] == '6621,37573,99692' or c[1] == '6621,37573,99692':
            pprint(circuits)

    # print()
    pprint(circuits)
    # pprint(circuits.values())

    # test for any junction in two circuits:
    for j in junctions:
        found_in = []
        fail_assert = True
        # for c in circuits:
        #     print(c, circuits[c])
        for conn in connections:
            if j in conn:
                # print(f"{j=} {conn=}")
                found_in.append(conn)
        if len(found_in) > 1:
            print(f"{j=} {found_in=}")
            if found_in[0][0] in circuits:
                p00 = circuits[found_in[0][0]]
                for f in found_in:
                    for p in f:
                        if p in circuits:
                            print(f"{p=} {circuits[p]=}")

                            if circuits[p] != p00:
                                fail_assert = False
                # assert fail_assert
                if not fail_assert:
                    return None

    counts = []
    for x in range(circuit_count):
        count_x = len([y for y in circuits.values() if y == x])
        # print(x, count_x)
        counts.append(count_x)
    print(counts)

    top3 = sorted(counts, reverse=True)[:3]
    print(top3)
    return list(accumulate(top3, lambda x, y: x * y))[-1]


# p1_test = part1(get("day-08.test.in"), num_connections=10)
# print(f"Test Part 1: {p1_test}")
# assert p1_test == 40

p1 = part1((get()))
print(f"Part 1: {p1}")
assert p1 > 8262
