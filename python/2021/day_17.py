# Slow part 2, at least when running it on an iPad, but it works.

with open('input.txt') as d:
	lines = d.read().strip().split(': ')
	target = lines[1].split(',')
	target_x = list(map(int, target[0][2:].split('..')))
	target_y = list(map(int, target[1][3:].split('..')))

part_one = 0
part_two = 0

for dxx in range(max(target_x) + 1):
	for dyy in range(min(target_y) - 1, 1000):
		hit_target = False
		highest_y = 0
		x = 0
		y = 0
		dx = dxx
		dy = dyy

		for t in range(1000):
			x += dx
			y += dy
			highest_y = max(highest_y, y)

			# Drag
			if dx > 0:
				dx -= 1
			elif dx < 0:
				dx += 1

			# Gravity
			dy -= 1

			if target_x[0] <= x <= target_x[1] and target_y[0] <= y <= target_y[1]:
				hit_target = True

		if hit_target:
			part_two += 1
			if highest_y > part_one:
				part_one = highest_y
				print('x:', dxx, 'y:', dyy, 'max height:', part_one)

print('Part 1:', part_one)
print('Part 2:', part_two)
