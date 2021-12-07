def part_01():
	with open('input_07.txt') as d:
		lines = list(map(int, d.read().strip().split(',')))
		fuels = []

		for i in range(min(lines), max(lines)):
			crab = []

			for l in lines:
				crab.append(abs(l - i))

			fuels.append(sum(crab))

		return min(fuels)

# This is hella slow, but the best I could figure out while unpacking
# in a new house and looking after a baby

def part_02():
	with open('input_07.txt') as d:
		lines = list(map(int, d.read().strip().split(',')))
		least_fuel = 0

		for i in range(min(lines), max(lines)):
			crab = []

			for l in lines:
				crab_fuel = []
				for j in range(0, abs(l - i + 1)):
					crab_fuel.append(j + 1)

				crab.append(sum(crab_fuel))

			fuel = sum(crab)

			if fuel < least_fuel or least_fuel == 0:
				least_fuel = fuel

		return least_fuel


print('Part 1:', part_01())
print('Part 2:', part_02())
