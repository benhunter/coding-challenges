# Problem: https://adventofcode.com/2021/day/4

from pprint import pprint
import itertools

with open('4-input.txt') as f:
    # with open('4-input-test.txt') as f:
    # numbers, boards = [
    #     [line for line in section.split('\n')] for section in f.read().split('\n\n')]
    sections = f.read().split('\n\n')
    # print(sections)

numbers = sections[0].split(',')
boards = [[[cell.strip() for cell in row.split()] for row in board.split('\n')] for board in sections[1:]]
pprint(numbers)
pprint(boards)

def mark(board, number):
    for y, row in enumerate(board):
        for x, cell in enumerate(row):
            if cell == number:
                board[y][x] = 'X'
                return


# Check if a board has a completed row or column
def check(board):
    for row in board:
        if ''.join(row) == 'XXXXX':
            return True
    for col in zip(*board):
        if ''.join(col) == 'XXXXX':
            return True
    return False


# Draw numbers until there is a winner
winner = False
winning_board = None
for n in numbers:
    for board in boards:
        mark(board, n)
        if check(board):  # Should we update all the boards first?
            winner = True
            winning_board = board
            print(f'Winner: {n}')
            print(f'Winning board: {board}')
            break;
    if winner:
        break

part1 = list(itertools.chain.from_iterable(winning_board))
part1 = filter(lambda x: x != 'X', part1)
part1 = list(map(int, part1))
part1 = sum(part1)
part1 = part1 * int(n)
print(f'Part 1: {part1}')

# Part 2

part2 = 0
print(f'Part 2: {part2}')
