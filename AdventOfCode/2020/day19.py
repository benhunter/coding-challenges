# Advent of Code 2020 Day 19
# https://adventofcode.com/2020/day/19

import time
from functools import lru_cache


DEBUG = False
rules_tuple = {}


@lru_cache(maxsize=None)
def match(message, rule):
    """Match the message against the rules, starting with rule_num.
    Check left to right.

    There are 3 types of rule:
    1. literal: "a"
    2. sequence: 1 2
    3. option: sequence | sequence.
    Note sequence must be a tuple, even if only one element long.

    Return the substring that matches successfully. "" means no match.
    If the full message is returned, it matched entirely.

    LRU Cache increases performance dramatically. Note that the cache must be cleared
    when the global rules_tuple changes. Call match.cache_clear().
    """

    # print(f"Rule: {rule_num} : {rules_tuple[rule_num]}")
    if message == "":
        return ""
    if rule is None:
        raise RuntimeError("rule is None")
    if rule == ():
        raise RuntimeError("rule is empty list")
    if rules_tuple is None:
        raise RuntimeError("rules_tuple is None")

    # determine the type of rule that is rules_tuple[rule_num]
    if isinstance(rule, str):
        # literal rule
        if message == rule:
            
            return message
    elif isinstance(rule, int):
        return match(message, rules_tuple[rule])
    elif isinstance(rule, tuple):
        if isinstance(rule[0], int):
            if len(rule) == 1:
                # single rule
                return match(message, rules_tuple[rule[0]])
            # sequence rule
            # try every combo of consume + remaining message
            for message_index in range(len(message) - 1):
                if match(message[: message_index + 1], rule[0]) and match(
                    message[message_index + 1 :], rule[1:]
                ):
                    return message
        elif isinstance(rule[0], tuple):
            # option rule
            for option in rule:
                if match(message, option):
                    return message
            return ""  # none of the options matched
    return ""


def is_valid(message, rule):
    return match(message, rule) == message


def load_input(str_file_name):
    with open(str_file_name) as f:
        rules, messages = [x.splitlines() for x in f.read().split("\n\n")]

    rules_tuple = {}

    for r in rules:
        rule = r.split(":")
        rule[0] = int(rule[0])
        rules_tuple[rule[0]] = rule[1]
        rule = [rule[0], rule[1].strip().strip('"')]

        if rule[1].isalpha():
            pass

        elif "|" in rule[1]:
            rule = [rule[0], [x.strip() for x in rule[1].split("|")]]

            options = []
            for option in rule[1]:
                option = tuple(map(int, option.split()))
                # print(f"option_index: {option_index}, option: {option}")
                options.append(option)
            rule[1] = tuple(options)

        else:
            rule[1] = tuple(map(int, rule[1].split()))

        rules_tuple[rule[0]] = rule[1]

    return rules_tuple, messages


def count_matches(local_rules_tuple, messages):
    matches = 0
    match.cache_clear()
    global rules_tuple
    rules_tuple = local_rules_tuple

    # parse messages
    for message in messages:
        if DEBUG:
            print(f"Checking: {message}")
            start = time.time()
        FIRST_RULE = rules_tuple[0]
        if is_valid(message, FIRST_RULE):
            if DEBUG:
                print(f"Matched: {message}")
            matches += 1
        if DEBUG:
            print(f"Took {time.time() - start} seconds")
    return matches


def test_is_valid1():
    # literal rule
    match.cache_clear()
    global rules_tuple
    rules_tuple = {0: "a"}

    assert is_valid("a", rules_tuple[0])  # good
    assert not is_valid("b", rules_tuple[0])  # wrong char


def test_is_valid2():
    # sequence and literal rules
    match.cache_clear()
    global rules_tuple
    rules_tuple = {0: (1, 2), 1: "a", 2: "b"}
    assert is_valid("ab", rules_tuple[0])  # good
    assert not is_valid("a", rules_tuple[0])  # message too short
    assert not is_valid("b", rules_tuple[0])  # message too short
    assert not is_valid("abc", rules_tuple[0])  # message too long


def test_is_valid3():
    # sequence of repeating literals
    match.cache_clear()
    global rules_tuple
    rules_tuple = {0: (1, 2), 1: "a", 2: "a"}
    assert is_valid("aa", rules_tuple[0])  # good
    assert not is_valid("aaa", rules_tuple[0])  # too long


def test_is_valid4():
    # sequence of repeating literals, same rule used again
    match.cache_clear()
    global rules_tuple
    rules_tuple = {0: (1, 1), 1: "a", 2: "a"}
    assert is_valid("aa", rules_tuple[0])  # good
    assert not is_valid("aaa", rules_tuple[0])  # too long

    match.cache_clear()
    rules_tuple = {0: (1, 1, 1), 1: "a", 2: "a"}
    assert not is_valid("aa", rules_tuple[0])  # too short
    assert is_valid("aaa", rules_tuple[0])  # good
    assert not is_valid("aaaa", rules_tuple[0])  # too long

    match.cache_clear()
    rules_tuple = {0: (1, 1, 1, 1), 1: "a", 2: "a"}
    assert not is_valid("aaa", rules_tuple[0])  # too short
    assert is_valid("aaaa", rules_tuple[0])  # good
    assert not is_valid("aaaaa", rules_tuple[0])  # too long


def test_is_valid5():
    # sequence of sequences, nested rules
    match.cache_clear()
    global rules_tuple
    rules_tuple = {0: (1, 2), 1: (3, 4), 2: (4, 3), 3: "a", 4: "b"}
    assert is_valid("abba", rules_tuple[0])  # good
    assert not is_valid("baab", rules_tuple[0])  # bad
    assert not is_valid("a", rules_tuple[0])  # bad
    assert not is_valid("ab", rules_tuple[0])  # bad
    assert not is_valid("abb", rules_tuple[0])  # bad
    assert not is_valid("abbab", rules_tuple[0])  # bad


def test_is_valid6():
    # rule with simple option: a | b
    match.cache_clear()
    global rules_tuple
    rules_tuple = {0: ((1,), (2,)), 1: "a", 2: "b"}
    assert is_valid("a", rules_tuple[0])
    assert is_valid("b", rules_tuple[0])
    assert not is_valid("ab", rules_tuple[0])
    assert not is_valid("aa", rules_tuple[0])


def test_is_valid7():
    # option of sequences
    match.cache_clear()
    global rules_tuple
    rules_tuple = {0: (1, 2), 1: "a", 2: ((1, 3), (3, 1)), 3: "b"}
    assert is_valid("aab", rules_tuple[0])
    assert is_valid("aba", rules_tuple[0])
    assert not is_valid("ab", rules_tuple[0])
    assert not is_valid("abab", rules_tuple[0])
    assert not is_valid("abba", rules_tuple[0])


def test_is_valid_example2():
    match.cache_clear()
    global rules_tuple
    rules_tuple = {
        0: (4, 1, 5),
        1: ((2, 3), (3, 2)),
        2: ((4, 4), (5, 5)),
        3: ((4, 5), (5, 4)),
        4: "a",
        5: "b",
    }
    assert is_valid("ababbb", rules_tuple[0])
    assert is_valid("abbbab", rules_tuple[0])
    assert not is_valid("bababa", rules_tuple[0])
    assert not is_valid("aaabbb", rules_tuple[0])
    assert not is_valid("aaaabbb", rules_tuple[0])


def test_example1():
    count = count_matches(
        *load_input(".\\AdventOfCode\\2020\\day19-example1-input.txt")
    )
    assert count == 2


def test_example2():
    count = count_matches(
        *load_input(".\\AdventOfCode\\2020\\day19-example2-input.txt")
    )

    assert count == 2


def test_example3_before():
    match.cache_clear()
    count = count_matches(
        *load_input(".\\AdventOfCode\\2020\\day19-example3-input.txt")
    )
    assert count == 3


def test_example3_after():
    match.cache_clear()
    rules_tuple, messages = load_input(
        ".\\AdventOfCode\\2020\\day19-example3-input.txt"
    )

    rules_tuple[8] = ((42,), (42, 8))
    rules_tuple[11] = ((42, 31), (42, 11, 31))

    count = count_matches(rules_tuple, messages)
    assert count == 12


def test_example3_after_fail1():
    match.cache_clear()
    global rules_tuple
    rules_tuple = {
        0: (8, 11),
        10: ((23, 14), (28, 1)),
        11: ((42, 31), (42, 11, 31)),
        12: ((24, 14), (19, 1)),
        13: ((14, 3), (1, 12)),
        14: "b",
        15: ((1,), (14,)),
        16: ((15, 1), (14, 14)),
        17: ((14, 2), (1, 7)),
        18: (15, 15),
        19: ((14, 1), (14, 14)),
        1: "a",
        20: ((14, 14), (1, 15)),
        21: ((14, 1), (1, 14)),
        22: (14, 14),
        23: ((25, 1), (22, 14)),
        24: (14, 1),
        25: ((1, 1), (1, 14)),
        26: ((14, 22), (1, 20)),
        27: ((1, 6), (14, 18)),
        28: (16, 1),
        2: ((1, 24), (14, 4)),
        31: ((14, 17), (1, 13)),
        3: ((5, 14), (16, 1)),
        42: ((9, 14), (10, 1)),
        4: (1, 1),
        5: ((1, 14), (15, 1)),
        6: ((14, 14), (1, 14)),
        7: ((14, 5), (1, 21)),
        8: ((42,), (42, 8)),
        9: ((14, 27), (1, 26)),
    }

    assert is_valid("bbabbbbaabaabba", rules_tuple[0])
    assert is_valid("babbbbaabbbbbabbbbbbaabaaabaaa", rules_tuple[0])
    assert is_valid("aaabbbbbbaaaabaababaabababbabaaabbababababaaa", rules_tuple[0])
    assert is_valid("bbbbbbbaaaabbbbaaabbabaaa", rules_tuple[0])
    assert is_valid("bbbababbbbaaaaaaaabbababaaababaabab", rules_tuple[0])
    assert is_valid("ababaaaaaabaaab", rules_tuple[0])
    assert is_valid("ababaaaaabbbaba", rules_tuple[0])
    assert is_valid("baabbaaaabbaaaababbaababb", rules_tuple[0])
    assert is_valid("abbbbabbbbaaaababbbbbbaaaababb", rules_tuple[0])
    assert is_valid("aaaaabbaabaaaaababaa", rules_tuple[0])
    assert is_valid("aaaabbaabbaaaaaaabbbabbbaaabbaabaaa", rules_tuple[0])
    assert is_valid("aabbbbbaabbbaaaaaabbbbbababaaaaabbaaabba", rules_tuple[0])


if __name__ == "__main__":
    rules_tuple, messages = load_input(".\\AdventOfCode\\2020\\day19-input.txt")
    part1 = count_matches(rules_tuple, messages)
    print(f"Part 1: {part1}")

    # Part 2
    rules_tuple[8] = ((42,), (42, 8))  # tuples of one item must have the trailing comma
    rules_tuple[11] = ((42, 31), (42, 11, 31))
    part2 = count_matches(rules_tuple, messages)
    print(f"Part 2: {part2}")
