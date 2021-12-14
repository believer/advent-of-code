from collections import Counter

# My first attempt used a list. This worked fine in part 1, but was
# too slow to complete part 2.

with open('input_14.txt') as d:
	lines = d.readlines()
	polymer = lines[0].strip()
	rules = {}

	for l in lines[2:]:
		pair, insertion = l.strip().split(' -> ')
		rules[pair] = insertion
	
	# Find pairs and the count of that pair
	pairs = Counter()
	
	for i in range(len(polymer) - 1):
		pairs[polymer[i] + polymer[i+1]] += 1
	
for step in range(41):
	if step in [10, 40]:
		# Find how many times each elements occurs
		output = Counter()
		
		# If we take the first letter, v[0], from each pair we
		# get every character except the last one. This character is
		# the same as in the original string. We just need to add that
		# as well.
		for v in pairs:
			output[v[0]] += pairs[v]
			
		output[polymer[-1]] += 1
		answer = max(output.values()) - min(output.values())
		
		print('Part 1:' if step == 10 else 'Part 2:', answer)
		
	temp = Counter()
	
	# For instance AB -> C => (AC, CB)
	for p in pairs:
		rule = rules[p]
		value = pairs[p]
		
		temp[p[0] + rule] += value
		temp[rule + p[1]] += value
	
	pairs = temp
