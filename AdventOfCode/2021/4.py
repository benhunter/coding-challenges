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
# boards_orig = [[[cell.strip() for cell in row.split()] for row in board.split('\n')] for board in sections[1:]]
boards = [[[cell.strip() for cell in row.split()] for row in board.split('\n')] for board in sections[1:]]
boards_p2 = [[[cell.strip() for cell in row.split()] for row in board.split('\n')] for board in sections[1:]]

# pprint(numbers)
# pprint(boards)


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
def winner(numbers, boards):
    for n in numbers:
        for board in boards:
            mark(board, n)
        for board in boards:
            if check(board):
                # print(f'Winner: {n}')
                # print(f'Winning board: {board}')
                return board, n
    return None


winning_board, n = winner(numbers, boards)


def sum_board(board):
    return sum(int(cell) for row in board for cell in row if cell != 'X')


# part1 = list(itertools.chain.from_iterable(winning_board))
# part1 = filter(lambda x: x != 'X', part1)
# part1 = list(map(int, part1))
# part1 = sum(part1)
part1 = sum_board(winning_board) * int(n)
print(f'Part 1: {part1}')  # 55770, test: 4512

# Part 2
last_board_bingo = False
for n in numbers:
    # print(f'Drawing {n}')
    for board_num, board in enumerate(boards_p2):
        if board is None:
            continue
        mark(board, n)
        # print(f'Marked {n} on {board_num} sum: {sum_board(board)}')
        # pprint(board)
    for board_num, board in enumerate(boards_p2):
        if check(board):
            # print(f'Bingo! {n} on {board_num}')
            if len(boards_p2) > 1:
                boards_p2[board_num] = None
                # print(f'Removed board: {board_num} {board}, boards remaining: {len(boards_p2)}')
            else:
                # print(f'Not removing last board: {board}')
                last_board_bingo = True
    # Remove any boards set to None
    # boards_p2 = [board for board in boards_p2 if board is not None]
    boards_p2 = list(filter(lambda b: b is not None, boards_p2))  # same as above
    if last_board_bingo:
        # print(f'Bingo!')
        # print(f'Last board: {boards_p2[0]}')
        # pprint(boards_p2[0])
        break


part2 = sum_board(boards_p2[0]) * int(n)
print(f'Part 2: {part2}')  # 2980, test: 1924

boards_p2 = [[[cell.strip() for cell in row.split()] for row in board.split('\n')] for board in sections[1:]]
win_order = []
for n in numbers:
    # print(f'Drawing {n} board: {boards_p2}')
    for board_num, board in enumerate(boards_p2):
        if board_num in win_order:
            continue
        mark(board, n)
    for board in boards_p2:
        if board_num in win_order:
            continue
        if check(board):
            print(f'Bingo! {n}')
            win_order.append(board_num)
    if len(win_order) == len(boards_p2):
        break
print(f'Winning order: {win_order}')
print(f'Winning board: {boards_p2[win_order[-1]]}')
print(f'n: {n}')
print(f'Part 2: {sum_board(boards_p2[win_order[-1]]) * int(n)}')
