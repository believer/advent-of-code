with open('input.txt') as d:
	data = d.read().strip()
	binary = bin(int(data, 16))[2:]
	
	# Pad binary
	while len(binary) < 4 * len(data):
		binary = '0' + binary 

part_one = 0

def parse(bits, i):
	global part_one
	# First three bits is the version
	version = int(bits[i+0:i+3], 2)
	part_one += version
	# Next three bits is the ID
	id = int(bits[i+3:i+6], 2)
	
	# The packet is a literal value
	if id == 4:
		# Skip version and ID
		i += 6
		v = 0
		
		# Check every five bits
		while True:
			v = v * 16 + int(bits[i+1:i+5], 2)
			i += 5 
			
			# If a bit starts with 0, we've reached
			# the end
			if bits[i-5] == '0':
				return v,i
	# The packet is an operator
	else:
		length_id = int(bits[i+6], 2)
		vs = []
		
		# Length ID 0 indicates a 15-bit numbert
		if length_id == 0:
			bits_length = int(bits[i+7:i+7+15], 2)
			
			start_iter = i + 7 + 15
			i = start_iter
			
			while True:
				v, next_iter = parse(bits, i)
				i = next_iter
				if next_iter - start_iter == bits_length:
					break
		# Length ID 1 indicates a 11-bit number
		else:
			number_of_packets = int(bits[i+7:i+7+11], 2)
			
			i += 7 + 11
			
			for n in range(number_of_packets):
				v, next_iter = parse(bits, i)
				i = next_iter
		
		return v,i
	
parse(binary, 0)

print('Part 1:', part_one)
