def find_commons(lines, look_at):
	ones = 0
	zeroes = 0

	for i in range(len(lines)):
		v = lines[i][look_at].strip()

		if v == '0':
			zeroes += 1
		if v == '1':
			ones += 1

	return (zeroes, ones)


def binary_to_int(value):
	return int(value, 2)


def part_01():
	with open('input_03.txt') as d:
		lines = d.readlines()
		gamma = ''
		epsilon = ''
		look_at = 0

		while len(gamma) < len(lines[0].strip()):
			zeroes, ones = find_commons(lines, look_at)

			if ones > zeroes:
				gamma += '1'
				epsilon += '0'
			else:
				gamma += '0'
				epsilon += '1'

			look_at += 1

		return binary_to_int(gamma) * binary_to_int(epsilon)


def life_support_rating(input, type):
	output = input
	look_at = 0

	while len(output) > 1:
		zeroes, ones = find_commons(output, look_at)
		temp = []

		for i in range(len(output)):
			current = output[i][look_at]
			value = output[i]

			if type == 'oxygen':
				if ones > zeroes and current == '1':
					temp.append(value)
				if zeroes > ones and current == '0':
					temp.append(value)
				if ones == zeroes and current == '1':
					temp.append(value)

			if type == 'co2':
				if zeroes < ones and current == '0':
					temp.append(value)
				if ones < zeroes and current == '1':
					temp.append(value)
				if ones == zeroes and current == '0':
					temp.append(value)

		output = temp
		look_at += 1

	return binary_to_int(output[0])


def part_02():
	with open('input_03.txt') as d:
		lines = d.readlines()

		oxygen = life_support_rating(lines, 'oxygen')
		co2 = life_support_rating(lines, 'co2')

		return oxygen * co2


print('Part 1:', part_01())
print('Part 2:', part_02())
