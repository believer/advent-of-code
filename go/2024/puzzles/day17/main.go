package main

import (
	"fmt"
	"regexp"
	"slices"
	"strconv"
	"strings"

	"github.com/believer/aoc-2024/utils"
	"github.com/believer/aoc-2024/utils/files"
)

type Computer struct {
	A, B, C      int
	Iteration    int
	Instructions []int
	Output       []string
}

func (c *Computer) Operand() int {
	index := c.Iteration + 1

	switch c.Instructions[index] {
	case 0, 1, 2, 3:
		return c.Instructions[index]
	case 4:
		return c.A
	case 5:
		return c.B
	case 6:
		return c.C
	default:
		panic("This should be unreachable")
	}
}

func (c *Computer) Run() {
	for c.Iteration < len(c.Instructions) {
		opcode := c.Instructions[c.Iteration]
		operand := c.Operand()
		literalOperand := c.Instructions[c.Iteration+1]

		switch opcode {
		// adv :: A / 2^operand -> A
		case 0:
			// Found this fancy and performant way of doing the calculation
			// using right shift assignment. It's the same as dividing by a
			// power of 2 for integers and truncates the value.
			// So exactly what the puzzle description hints to.
			// For part 1 it doesn't make a difference, but in part 2 it actually
			// makes quite a larger difference in performance.
			c.A >>= operand
			// bxl :: bitwise XOR of B and the _literal_ operand -> B
		case 1:
			c.B ^= literalOperand
			// bst :: operand % 8 -> B
		case 2:
			// When we calculate modulo of a % b, where b is a power of 2 (like we have),
			// we can also do a & (b-1) (Bitwise AND). This should be faster in theory, but
			// I tested it and found that it wasn't in this case. Still a nice learning
			// to keep around.
			c.B = operand % 8
			// jnz :: Nothing if A is zero. Otherwise, jump to _literal_ operand.
		case 3:
			if c.A != 0 {
				c.Iteration = literalOperand
				continue
			}
			// bxc :: bitwise XOR of B and C -> B
		case 4:
			c.B ^= c.C
			// out :: Output value as operand % 8
		case 5:
			c.Output = append(c.Output, strconv.Itoa(operand%8))
			// bdv :: A / 2^operand -> B
		case 6:
			c.B = c.A >> operand
			// cdv :: A / 2^operand -> C
		case 7:
			c.C = c.A >> operand
		}

		c.Iteration += 2
	}
}

func main() {
	fmt.Println("Part 1: ", part1("input.txt"))
	fmt.Println("Part 2: ", part2("input.txt"))
}

func part1(name string) string {
	A, B, C, instructions := parseInput(name)

	computer := Computer{A: A, B: B, C: C, Instructions: instructions}
	computer.Run()

	return strings.Join(computer.Output, ",")
}

func part2(name string) int {
	_, B, C, instructions := parseInput(name)
	validPrograms := []int{0}

	// Run through the instructions of the program in reverse since that's
	// how they are added to the list
	instructionsReversed := append([]int{}, instructions...)
	slices.Reverse(instructionsReversed)

	for _, instruction := range instructionsReversed {
		next := []int{}

		for _, v := range validPrograms {
			for n := range 8 {
				// Increase the value three bits since we're in a 3-bit computer.
				// This is effectively v * 2^3. Then insert the value of n in the lower bits.
				a := (v << 3) | n

				computer := Computer{A: a, B: B, C: C, Instructions: instructions}
				computer.Run()

				// If the instruction matches, add it to the list of valid inputs
				if instruction == utils.MustIntFromString(computer.Output[0]) {
					next = append(next, a)
				}
			}
		}

		validPrograms = next
	}

	return slices.Min(validPrograms)
}

func parseInput(name string) (int, int, int, []int) {
	lines := files.ReadParagraphs(name)
	registerRe := regexp.MustCompile(`\d+`)

	A := utils.MustIntFromString(registerRe.FindString(lines[0][0]))
	B := utils.MustIntFromString(registerRe.FindString(lines[0][1]))
	C := utils.MustIntFromString(registerRe.FindString(lines[0][2]))
	instructionList := registerRe.FindAllString(lines[1][0], -1)
	instructions := []int{}

	for _, instruction := range instructionList {
		instructions = append(instructions, utils.MustIntFromString(instruction))
	}

	return A, B, C, instructions
}
