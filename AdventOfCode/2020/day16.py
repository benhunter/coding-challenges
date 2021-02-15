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


if __name__ == "__main__":
    # filename = "./AdventOfCode/2020/day16-input.txt"
    filename = "./AdventOfCode/2020/day16-example1-input.txt"

    with open(filename) as f:
        # lines = [line.rstrip() for line in f]
        # fields, my_ticket, tickets = f.read().split("\n\n")
        fields, my_ticket, tickets = [
            [line for line in section.split("\n")] for section in f.read().split("\n\n")
        ]
        # print(lines)
    print(f"{fields}\n{my_ticket}\n{tickets}")

    field_values = [
        [int(r) for r in ranges.split("-")]
        for field in fields
        for index, ranges in enumerate(field.split())
        if re.match(r"\d+-\d+", ranges)
    ]
    print(field_values)

    tickets = [[int(val) for val in ticket.split(",")] for ticket in tickets[1:]]
    print(tickets)

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
    print(valid_tickets)

    # build all possible fields
    # eliminate by testing every ticket until there's only one possible field

    # print(len(field_values))
    for index_fields in range(len(tickets[0])):
        valid_fields = [True for i in range(len(fields))]
        for ticket in valid_tickets:
            print(ticket)
            # if valid_field(ticket[index_fields], )
    
    part2 = False
    print(f"Part 2: {part2}")




