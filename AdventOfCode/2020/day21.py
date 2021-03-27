# Advent of Code 2020 Day 21
# https://adventofcode.com/2020/day/21


from pprint import pprint

DEBUG = True


def load(filename):
    """Load puzzle input.
    Input will be stored as a list of labels.
    Each label is a list of ingredients and a list of allergens.
    List[List[List,List]]
    """
    with open(filename) as f:
        lines = [line.rstrip() for line in f]
    if DEBUG:
        print(lines)

    foods = [
        [
            [ingredient for ingredient in splitline[0].split()],
            [allergen.strip(",") for allergen in splitline[1][:-1].split()],
        ]
        for line in lines
        if (splitline := line.split(" (contains "))
    ]
    if DEBUG:
        pprint(foods)
    return foods


def test_load():
    example1_file = "./AdventOfCode/2020/day21-example1-input.txt"
    assert load(example1_file)


def part1(filename):
    foods = load(filename)

    allergens = [allergen for food in foods for allergen in food[1]]
    allergen_dict = {}

    for allergen in allergens:
        foods_with_allergen = [food for food in foods if allergen in food[1]]
        foods_without_allergen = [food for food in foods if allergen not in food[1]]

        pprint(foods_with_allergen)
        pprint(foods_without_allergen)

        allergen_dict[allergen] = [
            ingredient
            for food in foods_with_allergen
            for ingredient in food[0]
            # ingedient is in every food with allergen one time
            # and not in any other food that doesn't have the allergen
            if (
                len(
                    [
                        food2
                        for food2 in foods_with_allergen
                        if food2[0].count(ingredient) == 1
                    ]
                )
                == len(foods_with_allergen)
                # and len([
                #     food3
                #     for food3 in foods_without_allergen
                #     if (ingredient in food3[0])
                # ]) == 0
            )
        ][0]
    print(allergen_dict)


def test_part1():
    example1_file = "./AdventOfCode/2020/day21-example1-input.txt"
    assert part1(example1_file) == 5

    # TODO
    # input_file = "./AdventOfCode/2020/day21-input.txt"
    # assert part1(load(input_file)) == XXXX


def main():
    # filename = "./AdventOfCode/2020/day21-input.txt"
    filename = "./AdventOfCode/2020/day21-example1-input.txt"
    part1(filename)


def test_main():
    main()


if __name__ == "__main__":
    main()