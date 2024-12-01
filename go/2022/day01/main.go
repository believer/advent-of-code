package main

import (
	"log"
	"sort"
	"strings"

	"github.com/believer/aoc-2022/common"
)

func Part1(elfs []string) int {
	maxCalories := 0

	for _, elf := range elfs {
		calories := strings.Split(elf, "\n")
		totalCalories := common.SumStrings(calories)

		if totalCalories > maxCalories {
			maxCalories = totalCalories
		}
	}

	return maxCalories
}

func Part2(elfs []string) int {
	elfCalories := []int{}

	for _, elf := range elfs {
		calories := strings.Split(elf, "\n")
		totalCalories := common.SumStrings(calories)

		elfCalories = append(elfCalories, totalCalories)
	}

	sort.Ints(elfCalories)

	threeHighestCalories := elfCalories[len(elfCalories)-3:]
	totalCalories := common.SumInts(threeHighestCalories)

	return totalCalories
}

func main() {
	input, err := common.ReadInputFile()

	if err != nil {
		log.Fatalln(err)
	}

	elfs := strings.Split(input, "\n\n")

	log.Println("Part 1:", Part1(elfs))
	log.Println("Part 2:", Part2(elfs))
}
