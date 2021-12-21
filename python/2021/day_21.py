class Player:
	def __init__(self, start_position):
		self.position = start_position
		self.score = 0

	def play(self, t):
		rolls = [100 if l % 100 == 0 else l % 100 for l in range(t, t + 3)]
		dice_roll = sum(rolls)
		next_position = self.position + (dice_roll % 10)
		next_position = next_position % 10 if next_position > 10 else next_position

		self.position = next_position
		self.score += next_position

	def has_won(self):
		return self.score >= 1000


def part_01():
	player_one = Player(2)
	player_two = Player(8)
	rolls = 0
	turn = 0
	die = 1

	while True:
		rolls += 3

		# Player 1
		if turn % 2 == 0:
			player_one.play(die)
		else:
			player_two.play(die)

		if player_one.has_won() or player_two.has_won():
			break

		die += 3
		turn += 1

	losing_score = player_two.score if player_one.has_won() else player_one.score

	return losing_score * rolls


print('Part 1:', part_01())
