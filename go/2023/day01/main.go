package main

import (
	"beliver/advent-of-code/common"
	"log"
	"strconv"
	"strings"
	"time"
)

func Part1(input []string) (int, error) {
	start := time.Now()
	sum := 0

	for _, line := range input {
		chars := strings.Split(line, "")
		ints := []int{}

		for _, char := range chars {
			charInt, err := strconv.Atoi(char)

			if err != nil {
				continue
			}

			ints = append(ints, charInt)
		}

		firstDigit := ints[0]
		lastDigit := ints[len(ints)-1]

		sum += firstDigit*10 + lastDigit
	}

	log.Println("Part 1 took:", time.Since(start))

	return sum, nil
}

func main() {
	input, err := common.ReadInputFile()

	if err != nil {
		log.Fatalln(err)
	}

	inputData := strings.Split(strings.Trim(input, " "), "\n")
	values := common.RemoveEmptyStrings(inputData)

	part1, err := Part1(values)

	if err != nil {
		log.Fatalln(err)
	}

	log.Println("Part 1:", part1)
}
