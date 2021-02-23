# Advent of Code 2020 Day 18
# https://adventofcode.com/2020/18


from collections import namedtuple
from typing import List


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
        if tokens[0].type == "Number":
            return tokens[0].value

    if tokens[0].type == "OpenParen":
        # TODO wrong close parens ex: ()()
        # index_close_paren = reversed().index(Token("CloseParen"))
        return parse(tokens[1:index_close_paren])

    if tokens[-2].type == "Operator":
        if tokens[-2].value == "Add":
            return parse(tokens[:-2]) + tokens[-1].value
        elif tokens[-2].value == "Multiply":
            return parse(tokens[:-2]) * tokens[-1].value


    # old
    # if tokens[1].type == "Operator":
    #     if tokens[1].value == "Add":
    #         # TODO fix L->R parsing, order of operations
    #         return tokens[0].value + parse(tokens[2:])
    #     elif tokens[1].value == "Multiply":
    #         # TODO same as above
    #         return parse(tokens[2:] * tokens[0].value)


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
    print(lines[0])
    print(tokenize(lines[0]))
    print(parse(tokenize(lines[0])))
    print(tokenize(lines[1]))
