package main

import (
	"fmt"
	"slices"
	"strconv"
	"strings"

	"github.com/believer/aoc-2024/utils"
	"github.com/believer/aoc-2024/utils/files"
)

// I think an optimization could be that once an expression becomes
// true, we can skip the rest of the permutations.
func main() {
	fmt.Println("Part 1: ", part1("input.txt"))
	fmt.Println("Part 2: ", part2("input.txt"))
}

func part1(name string) int {
	lines := files.ReadLines(name)
	calibrationResult := 0
	operators := []string{"+", "*"}

	for _, l := range lines {
		result, valuesAsString, _ := strings.Cut(l, ":")
		stringValues := strings.Split(strings.TrimSpace(valuesAsString), " ")
		values := []int{}
		testValue := utils.MustIntFromString(result)

		for _, v := range stringValues {
			values = append(values, utils.MustIntFromString(v))
		}

		permutations := allPermutations(values, operators)
		totals := []int{}

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

			if total == testValue && !slices.Contains(totals, total) {
				totals = append(totals, total)
			}
		}

		for _, t := range totals {
			calibrationResult += t
		}
	}

	return calibrationResult
}

func part2(name string) int {
	lines := files.ReadLines(name)
	calibrationResult := 0
	operators := []string{"+", "*", "||"}

	for _, l := range lines {
		result, valuesAsString, _ := strings.Cut(l, ":")
		stringValues := strings.Split(strings.TrimSpace(valuesAsString), " ")
		values := []int{}
		testValue := utils.MustIntFromString(result)

		for _, v := range stringValues {
			values = append(values, utils.MustIntFromString(v))
		}

		permutations := allPermutations(values, operators)
		totals := []int{}

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

			if total == testValue && !slices.Contains(totals, total) {
				totals = append(totals, total)
			}
		}

		for _, t := range totals {
			calibrationResult += t
		}
	}

	return calibrationResult
}

func allPermutations(nums []int, ops []string) []string {
	if len(nums) == 0 {
		return nil
	}

	result := []string{}
	initial := strconv.Itoa(nums[0])
	generateExpressions(nums, ops, initial, 1, &result)
	return result
}

func generateExpressions(nums []int, ops []string, current string, index int, result *[]string) {
	if index == len(nums) {
		*result = append(*result, current)
		return
	}

	for _, op := range ops {
		next := current + " " + op + " " + strconv.Itoa(nums[index])
		generateExpressions(nums, ops, next, index+1, result)
	}
}
