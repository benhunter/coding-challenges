# Problem:

with open('5-input.txt') as f:
# with open('5-input-test.txt') as f:
    data = [line.rstrip() for line in f]
    # print(data)

lines = []
for row in data:
    # print(f"number: {number}")
    start, arrow, end = row.split()
    start = list(map(int, start.split(',')))
    end = list(map(int, end.split(',')))
    lines.append((start, end))

# max int in lines
max_position = max([dim for line in lines for point in line for dim in point])
# print(f'max_position: {max_position}')

# build a grid
grid = [[0 for x in range(max_position + 1)] for y in range(max_position + 1)]


# Update the grid with the lines
def update_grid(grid, start, end, calc_diagonal=False):
    if start[0] != end[0] and start[1] != end[1]:
        # Diagonal
        if calc_diagonal:
            direction = (int((end[0] - start[0])/abs(end[0] - start[0])), int((end[1] - start[1])/abs(end[1] - start[1])))
            x = start[0]
            y = start[1]
            while x != end[0] or y != end[1]:
                grid[y][x] += 1
                x += direction[0]
                y += direction[1]
            grid[y][x] += 1
        return

    # Horizontal or vertical
    for y in range(min(start[1], end[1]), max(start[1], end[1]) + 1):
        for x in range(min(start[0], end[0]), max(start[0], end[0]) + 1):
            grid[y][x] += 1
            # print(f'x: {x}, y: {y}, grid[y][x]: {grid[y][x]}')


# Print the grid
def print_grid(grid):
    for row in grid:
        print(row)


for line in lines:
    start = line[0]
    end = line[1]
    # print(f"start: {start} end: {end}")
    update_grid(grid, start, end)
    # print_grid(grid)

# max_cell = max([max(row) for row in grid])
# print(f'max_cell: {max_cell}')

part1 = len(list(filter(lambda x: x > 1, [cell for row in grid for cell in row])))
print(f'Part 1: {part1}')  # 6666, test: 5

# Part 2
grid = [[0 for x in range(max_position + 1)] for y in range(max_position + 1)]
for line in lines:
    start = line[0]
    end = line[1]
    # print(f"start: {start} end: {end}")
    update_grid(grid, start, end, calc_diagonal=True)
    # print_grid(grid)
print_grid(grid)
part2 = len(list(filter(lambda x: x > 1, [cell for row in grid for cell in row])))
print(f'Part 2: {part2}')  # 19081, test: 12
