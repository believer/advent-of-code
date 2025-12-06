package main

import (
	"cmp"
	"fmt"
	"slices"
	"strconv"
	"strings"

	"github.com/believer/aoc-utils/files"
	"github.com/believer/aoc-utils/utils"
)

func main() {
	fmt.Println("Part 1: ", part1("input.txt"))
	fmt.Println("Part 2: ", part2("input.txt"))
}

type Gate struct {
	a, b, operation, target string
}

func part1(name string) int64 {
	data := files.ReadParagraphs(name)

	gates := []Gate{}
	wires := map[string]int{}

	// Get initial wire values
	for _, input := range data[0] {
		gate, value, _ := strings.Cut(input, ": ")
		wires[gate] = utils.MustIntFromString(value)
	}

	// Create a list of all our gate operations
	for _, g := range data[1] {
		instance, target, _ := strings.Cut(g, " -> ")
		gateOperation := strings.Split(instance, " ")
		a, operation, b := gateOperation[0], gateOperation[1], gateOperation[2]

		gates = append(gates, Gate{a, b, operation, target})

		// Set initial wire values to "empty"
		if _, ok := wires[a]; !ok {
			wires[a] = -1
		}

		if _, ok := wires[b]; !ok {
			wires[b] = -1
		}

		if _, ok := wires[target]; !ok {
			wires[target] = -1
		}
	}

	for {
		// Do the calculations for each gate
		for _, g := range gates {
			a := wires[g.a]
			b := wires[g.b]

			if a == -1 || b == -1 {
				continue
			}

			switch g.operation {
			case "AND":
				wires[g.target] = a & b
			case "OR":
				wires[g.target] = a | b
			case "XOR":
				wires[g.target] = a ^ b
			}
		}

		// Check if all z-values have been set
		zValid := false

		for k, v := range wires {
			if strings.HasPrefix(k, "z") && v == -1 {
				zValid = true
				break
			}
		}

		if !zValid {
			break
		}
	}

	// Sort output keys descending
	outputKeys := []string{}

	for gate := range wires {
		if strings.HasPrefix(gate, "z") {
			outputKeys = append(outputKeys, gate)
		}
	}

	slices.SortFunc(outputKeys, func(a, b string) int {
		return cmp.Compare(b, a)
	})

	// Create a binary string from the sorted outputs
	binary := ""

	for _, k := range outputKeys {
		binary += strconv.Itoa(wires[k])
	}

	// Convert binary string to hex
	hex, _ := strconv.ParseInt(binary, 2, 64)

	return hex
}

func part2(name string) int {
	return 0
}
