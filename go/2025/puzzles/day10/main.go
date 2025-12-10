package main

import (
	"fmt"
	"math"
	"slices"
	"strings"

	"github.com/believer/aoc-utils/files"
	"github.com/believer/aoc-utils/utils"
)

func main() {
	fmt.Println("Part 1: ", part1("input.txt"))
	fmt.Println("Part 2: ", part2("input.txt"))
}

type Machine struct {
	lights  []int
	buttons [][]int
	joltage []int
}

func part1(name string) (totalIterations int) {
	lines := files.ReadLines(name)
	machines := createMachines(lines)

	for _, m := range machines {
		start := []int{}

		for i, l := range m.lights {
			if l == 1 {
				start = append(start, i)
			}
		}

		maskedStart := toBitmask(start)
		maskedButtons := []int{}
		end := 0

		for _, b := range m.buttons {
			maskedButtons = append(maskedButtons, toBitmask(b))
		}

		current := []int{maskedStart}
		iterations := 0

		for !slices.Contains(current, end) {
			next := []int{}

			for _, c := range current {
				for _, b := range maskedButtons {
					next = append(next, c^b)
				}
			}

			current = next
			iterations += 1
		}

		totalIterations += iterations
	}

	return
}

func part2(name string) int {
	return 0
}

func toBitmask(values []int) (mask int) {
	for _, i := range values {
		mask += int(math.Pow(2, float64(i)))
	}

	return
}

func createMachines(lines []string) (machines []Machine) {
	for _, l := range lines {
		lights := strings.Split(l[1:], "] ")
		buttonSchema := strings.Split(lights[1], " {")
		buttonSchema = strings.Fields(buttonSchema[0])

		var lightSchema []int
		var buttons [][]int

		for l := range strings.SplitSeq(lights[0], "") {
			switch l {
			case ".":
				lightSchema = append(lightSchema, 0)
			case "#":
				lightSchema = append(lightSchema, 1)
			}
		}

		for _, b := range buttonSchema {
			// Remove parenthesis
			b = b[1:]
			b = b[:len(b)-1]

			var buttonsSchema []int

			for b := range strings.SplitSeq(b, ",") {
				button := utils.MustIntFromString(b)
				buttonsSchema = append(buttonsSchema, button)
			}

			buttons = append(buttons, buttonsSchema)
		}

		machines = append(machines, Machine{
			lights:  lightSchema,
			buttons: buttons,
		})
	}

	return
}
