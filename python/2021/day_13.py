def run():
	part_1 = False

	with open('input_13.txt') as d:
		# Strip lines and remove empty lines
		lines = [line.strip() for line in d if line.strip()]
		grid = {}

		for line in lines:
			temp_grid = {}
			
			# Parse folding instructions
			if line.startswith('fold'):
				instruction = line.split()[-1]
				direction, value = instruction.split('=')
				value = int(value)

				if direction == 'x':
					for (x, y) in grid:
						if x < value:
							temp_grid[(x, y)] = True
						else:
							temp_grid[(value - (x - value), y)] = True

				if direction == 'y':
					for (x, y) in grid:
						if y < value:
							temp_grid[(x, y)] = True
						else:
							temp_grid[(x, value - (y - value))] = True

				grid = temp_grid

				if not part_1:
					part_1 = True
					print('Part 1:', len(temp_grid))

			else:
				# Parse dots
				x, y = [int(v) for v in line.split(',')]
				grid[(x, y)] = True

		# Create a grid from the dots in the final grid
		cols = max(x for x, y in grid.keys())
		rows = max(y for x, y in grid.keys())

		answer_part_2 = ''

		# Build the answer
		for y in range(rows + 1):
			for x in range(cols + 1):
				answer_part_2 += ('x' if (x, y) in grid else ' ')
			print('Part 2:', answer_part_2)
			answer_part_2 = ''


run()
