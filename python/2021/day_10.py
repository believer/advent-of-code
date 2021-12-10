points = {ord(')'): 3, ord(']'): 57, ord('}'): 1197, ord('>'): 25137}
points_part_two = {'(': 1, '[': 2, '{': 3, '<': 4}
open_brackets = ('(', '[', '{', '<')


def part_01():
	with open('input_10.txt') as d:
		score = 0

		for line in d:
			openers = []

			for c in line.strip():
				if c in open_brackets:
					openers.append(c)
				else:
					b = openers.pop()

					# End on corrupted lines
					if abs(ord(b) - ord(c)) > 3:
						score += points[ord(c)]
						break

		return score


def part_02():
	with open('input_10.txt') as d:
		scores = []

		for line in d:
			score = 0
			openers = []

			for c in line.strip():
				if c in open_brackets:
					openers.append(c)
				else:
					b = openers.pop()

					# End on corrupted lines
					if abs(ord(b) - ord(c)) > 3:
						break
			else:
				# Remove the open brackets one by one and
				# calculate the score of the closing bracket
				while len(openers):
					score *= 5
					score += points_part_two[openers.pop()]

				scores.append(score)

		scores.sort()

		return scores[len(scores) // 2]


print('Part 1:', part_01())
print('Part 2:', part_02())
