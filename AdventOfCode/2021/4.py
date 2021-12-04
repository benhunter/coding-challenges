# Problem: https://adventofcode.com/2021/day/4

from pprint import pprint
import itertools

# with open('4-input.txt') as f:
with open('4-input-test.txt') as f:
    # numbers, boards = [
    #     [line for line in section.split('\n')] for section in f.read().split('\n\n')]
    sections = f.read().split('\n\n')
    # print(sections)

numbers = sections[0].split(',')
boards = [[[cell.strip() for cell in row.split()] for row in board.split('\n')] for board in sections[1:]]
boards_p2 = boards.copy()
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


def sum_board(board):
    return sum(int(cell) for row in board for cell in row if cell != 'X')


# part1 = list(itertools.chain.from_iterable(winning_board))
# part1 = filter(lambda x: x != 'X', part1)
# part1 = list(map(int, part1))
# part1 = sum(part1)
part1 = sum_board(board) * int(n)
print(f'Part 1: {part1}')  # 4512

# Part 2
last_board_bingo = False
for n in numbers:
    print(f'Drawing {n}')
    for board in boards_p2:
        mark(board, n)
        print(f'Marked {board} {n} sum: {sum_board(board)}')
        if check(board):
            if len(boards_p2) > 1:
                boards_p2.remove(board)
                print(f'Removed board: {board}')
            else:
                print(f'Not removing last board: {board}')
                last_board_bingo = True
    if last_board_bingo:
        print(f'Bingo!')
        break

print(f'Last board: {boards_p2[0]}')
pprint(boards_p2[0])
print(f'Part 2: {sum_board(boards_p2[0])}')  # 706

part2 = 0
print(f'Part 2: {part2}')
