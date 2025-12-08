# https://adventofcode.com/2025/day/3

def get(file="day-03.in"):
    with open(file) as f:
        return [line.rstrip() for line in f]

def max_i(ints):
    # print(ints)
    max = ints[0] 
    m_i = 0
    for i, x in enumerate(ints):
        # print(i, x)
        if x > max:
            max = x
            m_i = i
    return m_i

def part1(input):
    ans = 0
    for l in input:
        # print(f"{l=}")
        ints = [int(c) for c in l]
        # print(f"{ints=}")
        i = max_i(ints[:-1]) # exclude last num
        # print(f"{i=}")
        ni = max_i(ints[i + 1:]) + i + 1
        # print(f"{ni=}")
        # print(i, ni)
        # print(i, ni, ints[i], ints[ni])
        ans += ints[i] * 10 + ints[ni]
        # print(f"{l=}, {ints[i]} {ints[ni]}, indexes: {i=}, {ni=}")

    return ans

assert part1(get('day-03.test.in')) == 357
assert part1(['2232133233333122223222321121432322323324333234233221423334362333113343833132233313523312322224432234']) == 85

p1 = part1(get())
assert p1 == 17311 
print(f"Part 1: {p1}")

def largest_x(ints, x):
    # print(f"largest_x {ints=} {x=}")
    if x == 0:
        raise RuntimeError
    if x == 1:
        mi = 0
        max = ints[mi]
        for i, c in enumerate(ints):
            if c > max:
                max = c
                mi = i
        return [max], mi

    ai = max_i(ints[:-(x - 1)])
    start_bi = ai + 1
    end_bi = len(ints) - x - 1
    slice = ints[start_bi:]
    # print(f"largest_x looking for b: {start_bi=} {end_bi=} {slice}")
    b, bi = largest_x(slice, x - 1)
    bi += ai + 1
    # print(f"{b=}")

    return [ints[ai]] + b, bi 

def part2(input):
    ans = 0
    for l in input:
        ints = [int(c) for c in l]
        lx = largest_x(ints, 12)
        ans += int(''.join([str(x) for x in lx[0]]))
    return ans
 
p2_test = part2(get('day-03.test.in'))
assert p2_test == 3121910778619

p2 = part2(get())
print(f"Part 2: {p2}")
assert p2 == 171419245422055
