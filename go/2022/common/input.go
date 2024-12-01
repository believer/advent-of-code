package common

import (
	"os"
)

func ReadInputFile() (string, error) {
	filePath := "input.txt"
	isTest := os.Getenv("TEST")

	if isTest == "true" {
		filePath = "test.txt"
	}

	data, err := os.ReadFile(filePath)

	if err != nil {
		return "", err
	}

	return string(data), nil
}
