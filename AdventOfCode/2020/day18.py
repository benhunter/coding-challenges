# Advent of Code 2020 Day 18
# https://adventofcode.com/2020/18


from collections import namedtuple
from typing import List


# Part 1
# Grammar:
# Expression =  "(" Expression ")" |
#               Expression Operator Number |
#               Number
# Operator = "+" | "*"
# Number = int()

Token = namedtuple("Token", "type, value", defaults=[None])
Token.__repr__ = lambda self: f"{self.type} {self.value}"


def tokenize(raw_expr):

    if raw_expr == "":
        return None

    expr = raw_expr.split()
    if len(expr) > 1:
        tokens = []
        for e in expr:
            tokens.extend(tokenize(e))
        return tokens

    if raw_expr[0] == "(":
        return [Token("OpenParen")] + tokenize(raw_expr[1:])
    elif raw_expr[-1] == ")":
        return tokenize(raw_expr[:-1]) + [Token("CloseParen")]
    elif raw_expr[0] == "+":
        return [Token("Operator", "Add")]
    elif raw_expr[0] == "*":
        return [Token("Operator", "Multiply")]
    elif raw_expr[0].isdigit():
        return [Token("Number", int(raw_expr))]


def parse(tokens: List[Token]):
    # Recursive
    # Base case
    if len(tokens) == 0:
        return None
    if len(tokens) == 1:
        if type(tokens[0]) == int:
            return tokens[0]
        elif tokens[0].type == "Number":
            return tokens[0].value

    if tokens[-2].type == "Operator":
        operand2 = tokens[-1] if type(tokens[-1]) is int else tokens[-1].value
        if tokens[-2].value == "Add":
            return parse(tokens[:-2]) + operand2
        elif tokens[-2].value == "Multiply":
            return parse(tokens[:-2]) * operand2

    if tokens[-1].type == "CloseParen":
        paren_count = 1
        index = len(tokens) - 1
        while paren_count != 0:
            index -= 1
            if tokens[index].type == "OpenParen":
                paren_count -= 1
            elif tokens[index].type == "CloseParen":
                paren_count += 1
        return parse(tokens[:index] + [parse(tokens[index + 1 : -1])])


# Part 1
# Grammar:
# Expression =  "(" Expression ")" |
#               Expression Multiply Number |
#               Number
# Multiply =    Expression "*" Expression |
#               Add "*" Expression
# Add =         Expression "+" Expression
# Number =      int()

# if parens, parse those
# if adds, parse those
# if muls, parse those


def parse_part2(tokens: List[Token]):
    # Recursive
    # Base case
    if type(tokens) is int:
        return tokens
    if len(tokens) == 0:
        raise RuntimeError()
    if len(tokens) == 1:
        if type(tokens[0]) is int:
            return tokens[0]
        if tokens[0].type == "Number":
            return tokens[0].value

    if Token("OpenParen") in tokens:
        open_paren_index = tokens.index(Token("OpenParen"))
        paren_count = 1
        close_paren_index = open_paren_index
        while paren_count != 0:
            close_paren_index += 1
            if tokens[close_paren_index].type == "OpenParen":
                paren_count += 1
            elif tokens[close_paren_index].type == "CloseParen":
                paren_count -= 1
        return parse_part2(
            tokens[:open_paren_index]
            + [parse_part2(tokens[open_paren_index + 1 : close_paren_index])]
            + tokens[close_paren_index + 1 :]
        )

    elif Token("Operator", "Multiply") in tokens:
        mul_index = tokens.index(Token("Operator", "Multiply"))
        return parse_part2(
            parse_part2(tokens[:mul_index]) * parse_part2(tokens[mul_index + 1 :])
        )
    elif Token("Operator", "Add") in tokens:
        add_index = tokens.index(Token("Operator", "Add"))
        return parse_part2(
            parse_part2(tokens[:add_index]) + parse_part2(tokens[add_index + 1 :])
        )


def test_tokenize():
    assert tokenize("") == None
    assert tokenize("1") == [Token("Number", 1)]
    assert tokenize("15") == [Token("Number", 15)]
    assert tokenize("154") == [Token("Number", 154)]
    assert tokenize("1 + 2") == [
        Token("Number", 1),
        Token("Operator", "Add"),
        Token("Number", 2),
    ]
    assert tokenize("(1 + 2)") == [
        Token("OpenParen"),
        Token("Number", 1),
        Token("Operator", "Add"),
        Token("Number", 2),
        Token("CloseParen"),
    ]


if __name__ == "__main__":
    filename = "./AdventOfCode/2020/day18-input.txt"
    # filename = "./AdventOfCode/2020/day18-example1-input.txt"

    with open(filename) as f:
        lines = [line.rstrip() for line in f]
    # print(lines)
    # for line in lines:
    # print(line)
    # print(lines[0])
    # print(tokenize(lines[0]))
    # print(parse(tokenize(lines[0])))
    # print(tokenize(lines[1]))
    # print(parse(tokenize(lines[1])))

    part1 = sum([parse(tokenize(line)) for line in lines])
    print(f"Part 1: {part1}")  # 209335026987

    part2 = sum([parse_part2(tokenize(line)) for line in lines])
    print(f"Part 2: {part2}")  # 33331817392479
