# Advent of Code 2020 Day 16
# https://adventofcode.com/2020/16


import re


def valid_field(field, field_values):
    for min_max in field_values:
        # print(min_max)
        if field >= min_max[0] and field <= min_max[1]:
            return True
    return False


def valid_ticket(ticket, field_values):
    for field in ticket:
        if not valid_field(field, field_values):
            return False
    return True


def remove_option(possibility, solutions):
    reduced_solution = []
    for s in solutions:
        if type(s) is list:
            reduced_list = [x for x in s if x != possibility]
            if len(reduced_list) == 1:
                reduced_list = reduced_list[0]
            reduced_solution.append(reduced_list)
        else:
            reduced_solution.append(s)
    return reduced_solution


def product(values):
    # see also: math.prod
    ret = 1
    ret = [ret := ret * v for v in values][-1]
    return ret


def test_product():
    assert product([2, 2]) == 4
    assert product([2, 3]) == 6


if __name__ == "__main__":
    filename = "./AdventOfCode/2020/day16-input.txt"
    # filename = "./AdventOfCode/2020/day16-example1-input.txt"

    with open(filename) as f:
        # lines = [line.rstrip() for line in f]
        # fields, my_ticket, tickets = f.read().split("\n\n")
        fields, my_ticket, tickets = [
            [line for line in section.split("\n")] for section in f.read().split("\n\n")
        ]
        # print(lines)
    # print(f"{fields}\n{my_ticket}\n{tickets}")

    field_values = [
        [int(r) for r in ranges.split("-")]
        for field in fields
        for index, ranges in enumerate(field.split())
        if re.match(r"\d+-\d+", ranges)
    ]
    # print(field_values)

    my_ticket_vals = [int(x) for x in my_ticket[1].split(",")]

    tickets = [[int(val) for val in ticket.split(",")] for ticket in tickets[1:]]
    # print(tickets)

    invalid = []
    for ticket in tickets:
        for field in ticket:
            if not valid_field(field, field_values):
                invalid.append(field)
    part1 = sum(invalid)
    print(f"Part 1: {part1}")  # 23115

    # Part 2
    valid_tickets = []
    for ticket in tickets:
        if valid_ticket(ticket, field_values):
            valid_tickets.append(ticket)
    # print(valid_tickets)

    # build all possible fields
    # eliminate by testing every ticket
    # until there's only one possible field

    solutions = [None for i in range(len(fields))]
    for index_fields in range(len(fields)):
        valid_fields = [True for i in range(len(fields))]
        for ticket in valid_tickets:
            for index_rules in range(len(fields)):
                # print(f"index_fields: {index_fields} ticket: {ticket}")
                if not valid_field(
                    ticket[index_fields],
                    field_values[index_rules * 2 : index_rules * 2 + 2],
                ):
                    # print(f"Invalid")
                    valid_fields[index_rules] = False

        # print(f"valid_fields: {valid_fields}")
        if valid_fields.count(True) == 1:
            solutions[index_fields] = valid_fields.index(True)
        elif valid_fields.count(True) > 1:
            # list of possible locations
            possibles = [i for i, vf in enumerate(valid_fields) if vf]
            solutions[index_fields] = possibles

    # reduce solutions
    solutions = [
        solutions := remove_option(s, solutions)
        for count in range(len(solutions))
        for i, s in enumerate(solutions)
        if type(s) is int
    ][-1]
    # Same as list comprehension above
    # for count in range(len(solutions)):
    #     for i, s in enumerate(solutions):
    #         if type(s) is int:
    #             # print(f"index: {i} solution: {s}")
    #             # remove s from all solutions that are a list
    #             solutions = remove_option(s, solutions)

    part2 = [my_ticket_vals[i] for i, s in enumerate(solutions) if s < 6]
    # Same as list comprehension above
    # part2 = []
    # for i, s in enumerate(solutions):
    #     if s < 6:
    #         part2.append(my_ticket_vals[i])

    part2 = product(part2)
    print(f"Part 2: {part2}")  # 239727793813
