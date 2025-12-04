def get(file="day-02.in"):
    with open(file) as f:
        return f.readline().rstrip().split(',')

def part1(lines):
    invalid = []
    for id_range in lines:
        # print(id_range)
        id_range = [int(x) for x in id_range.split('-')]
        for i in range(id_range[0], id_range[1] + 1):
            # print(i)
            s = str(i)
            l = len(s)
            if l % 2 != 0:
                continue

            half = l // 2
            if s[0:half] == s[half:]:
                invalid.append(i)
    print(invalid)
    return sum(invalid)




def part2(lines):
    pass

print(f"Test Part 1: {part1(get("day-02.test.in"))}")
# print(f"Test Part 2: {part2(get("day-02.test.in"))}")

print(f"Part 1: {part1(get())}")
# print(f"Part 2: {part2(get())}")

