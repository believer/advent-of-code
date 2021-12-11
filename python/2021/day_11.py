octopuses = []
flashes = 0

with open('input_11.txt') as d:
	for c in d:
		octopuses.append([int(c) for c in c.strip()])

rows = len(octopuses)
cols = len(octopuses[0])


def flash(row, col):
	global flashes
	flashes += 1
	octopuses[row][col] = -1

	for dr in [-1, 0, 1]:
		for dc in [-1, 0, 1]:
			rr = row + dr
			rc = col + dc

			if 0 <= rr < rows and 0 <= rc < cols and octopuses[rr][rc] != -1:
				octopuses[rr][rc] += 1

				if octopuses[rr][rc] >= 10:
					flash(rr, rc)


def run():
	steps = 0

	while True:
		steps += 1

		for i in range(rows):
			for j in range(cols):
				octopuses[i][j] += 1

		for i in range(rows):
			for j in range(cols):
				if octopuses[i][j] == 10:
					flash(i, j)

		done = True

		for i in range(rows):
			for j in range(cols):
				if octopuses[i][j] == -1:
					octopuses[i][j] = 0
				else:
					done = False

		if steps == 100:
			print('Part 1:', flashes)

		if done:
			print('Part 2:', steps)
			break


run()
