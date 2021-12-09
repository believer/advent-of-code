import numpy as np

directions = [(-1, 0), (1, 0), (0, -1), (0, 1)]


def parse_input(data):
	inputs = []
	output = 0

	for line in data:
		line = [int(char) for char in line.strip()]
		inputs.append(line)

	rows = len(inputs)
	cols = len(inputs[0])

	return (inputs, rows, cols)


with open('input_09.txt') as d:
	(inputs, rows, cols) = parse_input(d)

seen = [[False] * cols for _ in range(cols)]


def part_01():
	answer = 0

	for row in range(rows):
		for col in range(cols):
			v = inputs[row][col]
			adjacent = []

			for dr, dc in directions:
				next_row_value = row + dr
				next_col_value = col + dc

				if 0 <= next_row_value < rows and 0 <= next_col_value < cols:
					adjacent.append(inputs[next_row_value][next_col_value])

			if all(i > v for i in adjacent):
				answer += 1 + v

	return answer


def find_basin(row, col):
	if not (0 <= row < rows and 0 <= col < cols):
		return 0

	if seen[row][col]:
		return 0

	seen[row][col] = True

	if inputs[row][col] == 9:
		return 0

	output = 1

	for dr, dc in directions:
		nr = row + dr
		nc = col + dc

		if 0 <= nr < rows and 0 <= nc < cols:
			output += find_basin(nr, nc)

	return output


def part_02():
	with open('input_09.txt') as d:
		basin_sizes = []

		for row in range(rows):
			for col in range(cols):
				basin_sizes.append(find_basin(row, col))

		basin_sizes.sort()

		# Only the three largest basins
		return np.prod(basin_sizes[-3:])


print('Part 1:', part_01())
print('Part 2:', part_02())
