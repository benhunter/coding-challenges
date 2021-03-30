# Advent of Code 2020 Day 22
# https://adventofcode.com/2020/


if __name__ == "__main__":
    # filename = "./AdventOfCode/2020/day22-input.txt"
    filename = "./AdventOfCode/2020/day22-example1-input.txt"

    with open(filename) as f:
        # lines = [line.rstrip() for line in f]
        decks = [deck.split("\n") for deck in f.read().split("\n\n")]
    player1_orig = [int(card) for card in decks[0][1:]]
    player2_orig = [int(card) for card in decks[1][1:]]

    player1 = player1_orig.copy()
    player2 = player2_orig.copy()

    while player1 and player2:
        if player1[0] > player2[0]:
            player1.append(player1.pop(0))
            player1.append(player2.pop(0))
        else:
            player2.append(player2.pop(0))
            player2.append(player1.pop(0))
    winner = player1 if player1 else player2
    part1 = 0
    for posn, card in enumerate(winner):
        part1 += card * (len(winner) - posn)
        # print(posn, card, len(winner) - posn, card * (len(winner) - posn))
    print(part1)  # 36257

    # Part 2
    

