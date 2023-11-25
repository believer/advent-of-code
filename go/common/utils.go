package common

import (
	"log"
	"strconv"
)

func SumStrings(s []string) int {
	sum := 0

	for _, str := range s {
		if str == "" {
			continue
		}

		num, err := strconv.Atoi(str)

		if err != nil {
			log.Fatalln("SumStrings Failed:", err)
		}

		sum += num
	}

	return sum
}

func SumInts(i []int) int {
	sum := 0

	for _, num := range i {
		sum += num
	}

	return sum
}
