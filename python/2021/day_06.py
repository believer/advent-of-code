from collections import defaultdict

# Solved part 1 using a list, but that solution was too slow in part 2.
# Reworked the entire solution using a dict and it became so much faster.


def run(days):
	with open('input_06.txt') as d:
		fishes = [int(i) for i in d.read().strip().split(',')]
		sea = defaultdict(int)

		for fish in fishes:
			sea[fish] += 1

		for i in range(0, days):
			next_sea = defaultdict(int)

			for fish, count in sea.items():
				next_fish = fish - 1

				if next_fish < 0:
					next_fish = 6
					next_sea[8] += count

				next_sea[next_fish] += count

			sea = next_sea

		return sum(sea.values())


print('Part 1:', run(80))
print('Part 2:', run(256))
