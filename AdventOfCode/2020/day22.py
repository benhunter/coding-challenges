# Advent of Code 2020 Day 22
# https://adventofcode.com/2020/


def combat(player1, player2):
    winner = 0  # return int 1 or 2 for the winning player
    winning_deck = []  # return the winning deck

    player1_temp = player1.copy()
    player2_temp = player2.copy()

    round_history = []

    while player1_temp and player2_temp:
        # ensure this round was never played before
        if (player1_temp, player2_temp) in round_history:
            print("Round was played")
            raise RuntimeError("Round was played")
        round_history.append((player1_temp.copy(), player2_temp.copy()))

        if player1_temp[0] > player2_temp[0]:
            player1_temp.append(player1_temp.pop(0))
            player1_temp.append(player2_temp.pop(0))
        else:
            player2_temp.append(player2_temp.pop(0))
            player2_temp.append(player1_temp.pop(0))

    winner = 1 if player1_temp else 2
    winning_deck = player1_temp if player1_temp else player2_temp

    return winner, winning_deck


def recursive_combat(player1, player2):
    winner = 0  # return int 1 or 2 for the winning player
    winning_deck = []  # return the winning deck

    player1_temp = player1.copy()
    player2_temp = player2.copy()

    round_history = []

    while player1_temp and player2_temp:
        # ensure this round was never played before
        if (tuple(player1_temp), tuple(player2_temp)) in round_history:
            # print("Round was already played, Player 1 wins")
            winner = 1
            winning_deck = None
            return winner, winning_deck

        round_history.append((tuple(player1_temp), tuple(player2_temp)))

        if player1_temp[0] <= (len(player1_temp) - 1) and player2_temp[0] <= (
            len(player2_temp) - 1
        ):
            # play a new game of Recursive Combat if both players have at least
            # as many cards remaining as the value of the card they drew

            # the quantity of cards copied is equal to the number on the card they drew
            # to trigger the sub-game
            winner, winning_deck = recursive_combat(
                player1_temp[1 : player1_temp[0] + 1],
                player2_temp[1 : player2_temp[0] + 1],
            )
            if winner == 1:
                player1_temp.append(player1_temp.pop(0))
                player1_temp.append(player2_temp.pop(0))
            elif winner == 2:
                player2_temp.append(player2_temp.pop(0))
                player2_temp.append(player1_temp.pop(0))
        else:
            # play normal round
            # winner, winning_deck = combat(player1_temp, player2_temp)
            if player1_temp[0] > player2_temp[0]:
                player1_temp.append(player1_temp.pop(0))
                player1_temp.append(player2_temp.pop(0))
            else:
                player2_temp.append(player2_temp.pop(0))
                player2_temp.append(player1_temp.pop(0))

    winner = 1 if player1_temp else 2
    winning_deck = player1_temp if player1_temp else player2_temp

    return winner, winning_deck


if __name__ == "__main__":
    filename = "./AdventOfCode/2020/day22-input.txt"
    # filename = "./AdventOfCode/2020/day22-example1-input.txt"
    # filename = "./AdventOfCode/2020/day22-example2-recursive.txt"

    with open(filename) as f:
        # lines = [line.rstrip() for line in f]
        decks = [deck.split("\n") for deck in f.read().split("\n\n")]
    player1_orig = [int(card) for card in decks[0][1:]]
    player2_orig = [int(card) for card in decks[1][1:]]

    player1 = player1_orig.copy()
    player2 = player2_orig.copy()

    winner, winning_deck = combat(player1, player2)

    part1 = 0
    for posn, card in enumerate(winning_deck):
        part1 += card * (len(winning_deck) - posn)
        # print(posn, card, len(winner) - posn, card * (len(winner) - posn))
    part1 = sum(
        card * (len(winning_deck) - posn) for posn, card in enumerate(winning_deck)
    )
    print(f"Part 1: {part1}")  # 36257

    # Part 2
    player1 = player1_orig.copy()
    player2 = player2_orig.copy()
    winner, winning_deck = recursive_combat(player1, player2)
    part2 = sum(
        card * (len(winning_deck) - posn) for posn, card in enumerate(winning_deck)
    )
    print(f"Part 2: {part2}")  # 33304
