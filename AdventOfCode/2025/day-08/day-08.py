import math
from itertools import combinations
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
    return math.sqrt((a[0]-b[0])**2 + (a[1] - b[1])**2 + (a[2] - b[2])**2)

def part1(junctions):
    # print(junctions)
    dists = {}
    junctions = sorted(junctions)
    for a, b in combinations(junctions, 2):
        dists[a + ' ' + b] = dist(a, b)
        # print(a+b, dists[a+b])
    # print(dists.items())
    sorted_d = sorted(dists.items(), key=lambda x: x[1])
    pprint(sorted_d)


p1_test = part1(get("day-08.test.in"))
print(f"Test Part 1: {p1_test}")
assert p1_test == 40
