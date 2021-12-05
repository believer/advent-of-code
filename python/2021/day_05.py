class Grid:
	def __init__(self, x, y):
		self.grid = []
		self.create_grid(x, y)

	def create_grid(self, x, y):
		gridline = []
		for i in range(x + 1):
			gridline.append('.')
		grid = []
		for i in range(y + 1):
			grid.append(list(gridline))

		self.grid = grid

	def update_grid(self, x, y):
		if self.grid[x][y] == '.':
			self.grid[x][y] = 1
		else:
			self.grid[x][y] += 1

	def update_line(self, x, x2, y, y2):
		for i in range(x, x2 + 1):
			for j in range(y, y2 + 1):
				self.update_grid(i, j)

	def update_diagonal(self, x1, x2, y1, y2):
		xs = []
		ys = []

		for x in range(x1, x2 + 1 if x2 > x1 else x2 - 1, 1 if x2 > x1 else -1):
			xs.append(x)
		for y in range(y1, y2 + 1 if y2 > y1 else y2 - 1, 1 if y2 > y1 else -1):
			ys.append(y)

		d = zip(xs, ys)

		for (x, y) in d:
			self.update_grid(x, y)

	def intersections(self):
		intersections = 0

		for row in self.grid:
			for y in row:
				if y == '.':
					continue
				if y > 1:
					intersections += 1

		return intersections


def parse_input(input):
	data = []
	max_x = 0
	max_y = 0

	for l in input:
		l = l.strip().split(' -> ')
		d = []

		for v in l:
			x, y = list(map(int, v.split(',')))

			if y > max_y:
				max_y = y

			if x > max_x:
				max_x = x

			d.append((x, y))

		data.append(d)
	return (data, max_x, max_y)


def part_01():
	with open('input_05.txt') as d:
		lines = d.readlines()
		(data, max_x, max_y) = parse_input(lines)

		# Create grid
		grid = Grid(max_x, max_y)

		for d in data:
			(x1, y1), (x2, y2) = d

			if x1 == x2 or y1 == y2:
				if x2 > x1 or y2 > y1:
					grid.update_line(x1, x2, y1, y2)

				if x1 > x2 or y1 > y2:
					grid.update_line(x2, x1, y2, y1)

		return grid.intersections()


def part_02():
	with open('input_05.txt') as d:
		lines = d.readlines()
		(data, max_x, max_y) = parse_input(lines)

		# Create grid
		grid = Grid(max_x, max_y)

		for d in data:
			(x1, y1), (x2, y2) = d

			if x1 == x2 or y1 == y2:
				if x2 > x1 or y2 > y1:
					grid.update_line(x1, x2, y1, y2)

				if x1 > x2 or y1 > y2:
					grid.update_line(x2, x1, y2, y1)
			else:
				grid.update_diagonal(x1, x2, y1, y2)

		return grid.intersections()


print('Part 1:', part_01())
print('Part 2:', part_02())
