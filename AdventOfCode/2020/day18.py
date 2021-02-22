# Advent of Code 2020 Day 18
# https://adventofcode.com/2020/18


from collections import namedtuple

# Expression =  "(" Expression ")" |
#               Number Operator Expression
# Operator = "+" | "*"
# Number = int()

Token = namedtuple("Token", "type, value", defaults=[None])

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



def test_tokenize():
    assert tokenize("") == None
    assert tokenize("1") == [Token("Number", 1)]
    assert tokenize("15") == [Token("Number", 15)]
    assert tokenize("154") == [Token("Number", 154)]
    assert tokenize("1 + 2") == [Token("Number", 1), Token("Operator", "Add"), Token("Number", 2)]
    assert tokenize("(1 + 2)") == [Token("OpenParen"), Token("Number", 1), Token("Operator", "Add"), Token("Number", 2), Token("CloseParen")]


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
    print(tokenize(lines[1]))
