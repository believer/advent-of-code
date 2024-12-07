package main

import (
	"fmt"
	"strconv"
	"strings"

	"github.com/believer/aoc-2024/utils"
	"github.com/believer/aoc-2024/utils/files"
)

func main() {
	fmt.Println("Part 1: ", part1("input.txt"))
	fmt.Println("Part 2: ", part2("input.txt"))
}

func part1(name string) int {
	lines := files.ReadLines(name)
	calibrationResult := 0
	operators := []string{"+", "*"}

expression:
	for _, l := range lines {
		result, valuesAsString, _ := strings.Cut(l, ":")
		values := strings.Split(strings.TrimSpace(valuesAsString), " ")
		testValue := utils.MustIntFromString(result)

		permutations := allPermutations(values, operators)

		for _, expression := range permutations {
			total := 0
			operation := "+"

			for _, v := range strings.Split(expression, " ") {
				if v == "+" {
					operation = "+"
					continue
				}

				if v == "*" {
					operation = "*"
					continue
				}

				if operation == "+" {
					total += utils.MustIntFromString(v)
				}

				if operation == "*" {
					total *= utils.MustIntFromString(v)
				}
			}

			if total == testValue {
				calibrationResult += total
				continue expression
			}
		}
	}

	return calibrationResult
}

func part2(name string) int {
	lines := files.ReadLines(name)
	calibrationResult := 0
	operators := []string{"+", "*", "||"}

expression:
	for _, l := range lines {
		result, valuesAsString, _ := strings.Cut(l, ":")
		values := strings.Split(strings.TrimSpace(valuesAsString), " ")
		testValue := utils.MustIntFromString(result)

		permutations := allPermutations(values, operators)

		for _, expression := range permutations {
			total := 0
			operation := "+"

			for _, v := range strings.Split(expression, " ") {
				if v == "+" {
					operation = "+"
					continue
				}

				if v == "*" {
					operation = "*"
					continue
				}

				if v == "||" {
					operation = "||"
					continue
				}

				if operation == "+" {
					total += utils.MustIntFromString(v)
				}

				if operation == "*" {
					total *= utils.MustIntFromString(v)
				}

				if operation == "||" {
					total = utils.MustIntFromString(strconv.Itoa(total) + v)
				}
			}

			if total == testValue {
				calibrationResult += total
				continue expression
			}
		}
	}

	return calibrationResult
}

func allPermutations(nums []string, ops []string) []string {
	if len(nums) == 0 {
		return nil
	}

	result := []string{}
	generateExpressions(nums, ops, nums[0], 1, &result)
	return result
}

func generateExpressions(nums []string, ops []string, current string, index int, result *[]string) {
	if index == len(nums) {
		*result = append(*result, current)
		return
	}

	for _, op := range ops {
		next := current + " " + op + " " + nums[index]
		generateExpressions(nums, ops, next, index+1, result)
	}
}
