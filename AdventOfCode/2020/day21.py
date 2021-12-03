# Advent of Code 2020 Day 21
# https://adventofcode.com/2020/day/21


import itertools
from pprint import pprint

DEBUG = False


class Day21:
    """Class for maintaining state between Part 1 and 2."""

    def __init__(self, filename):
        """Load puzzle input.
        Input will be stored as a list of labels.
        Each label is a list of ingredients and a list of allergens.
        List[List[List,List]]
        """
        with open(filename) as f:
            lines = [line.rstrip() for line in f]
        if DEBUG:
            print(lines)

        self.foods = [
            [
                [ingredient for ingredient in splitline[0].split()],
                [allergen.strip(",") for allergen in splitline[1][:-1].split()],
            ]
            for line in lines
            if (splitline := line.split(" (contains "))
        ]
        if DEBUG:
            pprint(self.foods)

    def part1(self):
        self.allergen_dict = {}
        allergens = [allergen for food in self.foods for allergen in food[1]]

        for allergen in allergens:
            foods_with_allergen = [food for food in self.foods if allergen in food[1]]
            foods_without_allergen = [
                food for food in self.foods if allergen not in food[1]
            ]

            if DEBUG:
                pprint(foods_with_allergen)
                pprint(foods_without_allergen)

            self.allergen_dict[allergen] = [
                ingredient
                for food in foods_with_allergen
                for ingredient in food[0]
                # ingedient is in every food with allergen one time
                # ingredient may be in other foods that don't have the allergen
                if (
                    len(
                        [
                            food2
                            for food2 in foods_with_allergen
                            if food2[0].zeros(ingredient) == 1
                        ]
                    )
                    == len(foods_with_allergen)
                )
            ]
            self.allergen_dict[allergen] = list(set(self.allergen_dict[allergen]))
        if DEBUG:
            print(self.allergen_dict)
            print(list(self.allergen_dict.values()))

        ingredients_with_allergens = set(
            list(itertools.chain.from_iterable(self.allergen_dict.values()))
        )
        ingredients_without_allergens = [
            ingredient
            for label in self.foods
            for ingredient in label[0]
            if ingredient not in ingredients_with_allergens
        ]
        if DEBUG:
            print(ingredients_without_allergens)
            print(len(ingredients_without_allergens))
        return len(ingredients_without_allergens)

    def part2(self):
        # Find the actual dangerous ingredients by removing redundant values so that
        # each allergen has one ingredient associated.

        # part1() must run first to generate self.allergen_dict
        if not hasattr(self, "allergen_dict"):
            self.part1()

        # create a copy of the allergen dictionary so we can remove items from it
        # as we eleminate redundant valuesmm
        temp_allergen_dict = self.allergen_dict.copy()
        dangerous_ingredients = {}
        while len(temp_allergen_dict) > 0:
            for allergen_key, allergen_values in temp_allergen_dict.items():
                if len(allergen_values) == 1:
                    dangerous_ingredients[allergen_key] = allergen_values[0]
                    temp_allergen_dict.pop(allergen_key)  # remove from temp dict
                    # now remove that ingredient from all other allergen values
                    for temp_key, temp_values in temp_allergen_dict.items():
                        if allergen_values[0] in temp_values:
                            temp_values.remove(allergen_values[0])
                            temp_allergen_dict[temp_key] = temp_values
                    break
        dangerous = ",".join(
            [dangerous_ingredients[key] for key in sorted(dangerous_ingredients.keys())]
        )
        return dangerous


def test_load():
    example1_file = "./AdventOfCode/2020/day21-example1-input.txt"
    day21_example = Day21(example1_file)
    assert day21_example


def test_part1():
    example1_file = "./AdventOfCode/2020/day21-example1-input.txt"
    day21_example = Day21(example1_file)
    assert day21_example.part1() == 5

    input_file = "./AdventOfCode/2020/day21-input.txt"
    day21 = Day21(input_file)
    assert day21.part1() == 2542


def test_part2():
    example1_file = "./AdventOfCode/2020/day21-example1-input.txt"
    day21_example = Day21(example1_file)
    assert day21_example.part2() == "mxmxvkd,sqjhc,fvjkl"

    input_file = "./AdventOfCode/2020/day21-input.txt"
    day21 = Day21(input_file)
    assert day21.part2() == "hkflr,ctmcqjf,bfrq,srxphcm,snmxl,zvx,bd,mqvk"


def main():
    filename = "./AdventOfCode/2020/day21-input.txt"
    # filename = "./AdventOfCode/2020/day21-example1-input.txt"

    day21 = Day21(filename)
    print(day21.part1())
    print(day21.part2())


def test_main():
    main()


if __name__ == "__main__":
    main()