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

def valid_part2(i):
    s = str(i)
    l = len(s)
    for pattern_length in range(1, (l//2)+1):
        # print(f"{pattern_length=}")
        pattern = True
        for chunk in range(0, l-pattern_length, pattern_length):
            # print(f"{chunk=}")
            if s[chunk:chunk+pattern_length] != s[chunk+pattern_length:chunk+pattern_length+pattern_length]:
                pattern = False
                break
        if pattern:
            return False
    return True

def part2(lines):
    invalid = []
    for id_range in lines:
        # print(id_range)
        id_range = [int(x) for x in id_range.split('-')]
        for i in range(id_range[0], id_range[1] + 1):
            # print(i)
            if not valid_part2(i):
                # print(f"{i} is invalid")
                invalid.append(i)
            # else:
            #     print(f"{i} is valid")
    print(invalid)
    return sum(invalid)

print(f"Test Part 1: {part1(get("day-02.test.in"))}")
test_result_part_2 = part2(get("day-02.test.in"))
print(f"Test Part 2: {test_result_part_2}")
assert test_result_part_2 == 4174379265

print(f"Part 1: {part1(get())}") # 12850231731
print(f"Part 2: {part2(get())}") # 24774350322

# print(f"{valid_part2(2121212120)}")
