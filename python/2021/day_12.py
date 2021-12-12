input = []
paths = {}

with open('input_sample_12.txt') as d:
	lines = d.readlines()

	for line in lines:
		input.append(line.strip().split('-'))

	# Find all path pairs
	for x, y in input:
		for a, b in ((x, y), (y, x)):
			paths[a] = paths[a] + [b] if paths.get(a) else [b]


def small_cave_visited(visited):
	for v in visited:
		if v != 'start' and v.islower() and visited.count(v) > 1:
			return v

	return None


def visit_cave(path, visited, roads, allow_small_caves_twice):
	if path == 'end':
		roads.append([*visited, 'end'])
		return len(roads)

	# Small caves are only allowed once (or twice in part 2)
	if path.islower() and path in visited:
		if not allow_small_caves_twice or path == 'start' or small_cave_visited(visited):
			return len(roads)

	visited = [*visited, path]

	for road in paths[path]:
		visit_cave(road, visited, roads, allow_small_caves_twice)

	return len(roads)


def part_01():
	return visit_cave('start', [], [], False)


def part_02():
	return visit_cave('start', [], [], True)


print('Part 1:', part_01())
print('Part 2:', part_02())
