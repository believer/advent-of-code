from collections import defaultdict


def part_01():
	with open('input_08.txt') as d:
		numbers = 0

		for line in d:
			words = line.strip().split()

			for word in words[11:15]:
				if len(word) in (2, 3, 4, 7):
					numbers += 1

		return numbers


# Find if every segment matches the other
def every_segment(first, second):
	return all([c in first for c in second])


# Find number of matching segments
def matching_segments(first, second):
	return sum([c in first for c in second])


def part_02():
	with open('input_08.txt') as d:
		outputs = []
		signals = []
		answer = 0

		# Find sections
		for line in d:
			words = line.strip().split()
			signals.append(words[:10])
			outputs.append(words[11:15])

		# Find each number
		for signal, output in zip(signals, outputs):
			decrypt = ['' for i in range(10)]
			signal = sorted(signal, key=len)

			for i in signal:
				if len(i) == 2: decrypt[1] = i
				elif len(i) == 3: decrypt[7] = i
				elif len(i) == 4: decrypt[4] = i
				elif len(i) == 5:
					# If the word contains all segments of 1, it's a 3
					if every_segment(i, decrypt[1]):
						decrypt[3] = i
						# If the word contains 3 parts of a 4, it's a 5
					elif matching_segments(i, decrypt[4]) == 3:
						decrypt[5] = i
						# Otherwise it's a 2
					else:
						decrypt[2] = i
				elif len(i) == 6:
					# If the word contains all segments of 4, it's a 9
					if every_segment(i, decrypt[4]):
						decrypt[9] = i
						# If the word contains all segments of 7, it's a 0
					elif every_segment(i, decrypt[7]):
						decrypt[0] = i
						# Otherwise it's a 6
					else:
						decrypt[6] = i
				else:
					decrypt[8] = i

			# Calculate the output number
			output_number = 0

			for j, n in enumerate(output[::-1]):
				for i in range(10):
					if every_segment(n, decrypt[i]) and len(decrypt[i]) == len(n):
						output_number += i * 10**j
						break

			answer += output_number

		return answer


print('Part 1:', part_01())
print('Part 2:', part_02())
