class Board:
	def __init__(self, data):
		self.position = {}
		self.playBoard = [
			[0, 0, 0, 0, 0],
			[0, 0, 0, 0, 0],
			[0, 0, 0, 0, 0],
			[0, 0, 0, 0, 0],
			[0, 0, 0, 0, 0],
		]
		self.bingo = {"row": [0, 0, 0, 0, 0], "col": [0, 0, 0, 0, 0]}
		self.createBoard(data)

	def createBoard(self, data):
		self.playBoard = data

		for i in range(5):
			for j in range(5):
				choice = data[i][j]
				self.position[choice] = (i, j)

	def updateBoard(self, val):
		if val in self.position:
			x, y = self.position[val]
			self.playBoard[x][y] = 'X'
			self.updateBingo(x, y)

	def updateBingo(self, x, y):
		self.bingo["row"][x] += 1
		self.bingo["col"][y] += 1

	def checkBingo(self):
		return 5 in self.bingo["row"] or 5 in self.bingo["col"]

	def unmarked_numbers(self):
		unmarked = []

		for i in range(5):
			for j in range(5):
				val = self.playBoard[i][j]
				if val != 'X':
					unmarked.append(val)

		return unmarked

	def displayBoard(self):
		board = self.playBoard

		for i in range(5):
			for j in board[i]:
				if j == 'X':
					print(f" {j}", end=" ")
				elif j > 9:
					print(j, end=" ")
				else:
					print(f" {j}", end=" ")
			print()
		print()


def part_01():
	with open('input_04.txt') as d:
		lines = d.readlines()

		bingo_numbers = list(map(int, lines[0].strip().split(',')))
		all_boards = lines[2:]
		boards = []
		i = 0
		board = []

		for data in all_boards:
			line = list(map(int, data.strip().split()))

			if len(line) != 0:
				board.append(line)

			if len(board) == 5:
				boards.append(Board(board))
				i = 0
				board = []
				continue

		winning_number = 0
		winning_board = 0

		for n in bingo_numbers:
			for b in boards:
				b.updateBoard(n)

				if b.checkBingo():
					winning_number = n
					winning_board = b
					break
			else:
				continue
			break

		return winning_number * sum(winning_board.unmarked_numbers())


def part_02():
	with open('input_04.txt') as d:
		lines = d.readlines()

		bingo_numbers = list(map(int, lines[0].strip().split(',')))
		all_boards = lines[2:]
		boards = []
		i = 0
		board = []

		for data in all_boards:
			line = list(map(int, data.strip().split()))

			if len(line) != 0:
				board.append(line)

			if len(board) == 5:
				boards.append(Board(board))
				i = 0
				board = []
				continue

		last_winning_number = 0
		last_states = {}
		last_winner = 0

		for n in bingo_numbers:
			for i, b in enumerate(boards):
				b.updateBoard(n)

				if i in last_states:
					continue

				if b.checkBingo():
					last_states[i] = (n, sum(b.unmarked_numbers()))
					last_winner = i
			else:
				continue
			break

		winning_number, winning_sum = last_states[last_winner]

		return winning_number * winning_sum


print('Part 1:', part_01())
print('Part 2:', part_02())
