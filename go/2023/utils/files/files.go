package files

import (
	"bufio"
	"os"
	"path"
	"runtime"
)

// Borrowed from https://github.com/kylehoehns/aoc-2023-go/blob/main/pkg/files/filereader.go
// Seems like a great starting point for me to learn and adjust according to my needs.

// ReadLines reads a file and returns a slice of strings, one for each line
func ReadLines(name string) []string {
	_, callingFile, _, ok := runtime.Caller(1)

	if !ok {
		panic("unable to find caller so cannot build path to read file")
	}

	return readLines(name, callingFile)
}

// Read reads a file and returns a string containing the entire file
func Read(name string) string {
	_, callingFile, _, ok := runtime.Caller(1)

	if !ok {
		panic("unable to find caller so cannot build path to read file")
	}

	b, err := os.ReadFile(path.Join(path.Dir(callingFile), name))

	if err != nil {
		panic(err)
	}

	return string(b)
}

// ReadParagraphs reads a file and returns a slice of slices of strings, one for each line
// A gap of one or more blank lines is used to split the file into groups
func ReadParagraphs(name string) [][]string {
	_, callingFile, _, ok := runtime.Caller(1)

	if !ok {
		panic("unable to find caller so cannot build path to read file")
	}

	lines := readLines(name, callingFile)

	var groups [][]string

	curGroup := make([]string, 0)

	for _, line := range lines {
		if line == "" {
			groups = append(groups, curGroup)
			curGroup = make([]string, 0)
		} else {
			curGroup = append(curGroup, line)
		}
	}

	if len(curGroup) > 0 {
		groups = append(groups, curGroup)
	}

	return groups
}

func readLines(name string, callingFile string) []string {
	inputFile, err := os.Open(path.Join(path.Dir(callingFile), name))

	if err != nil {
		panic(err)
	}

	defer func(inputFile *os.File) {
		err := inputFile.Close()

		if err != nil {
			panic(err)
		}
	}(inputFile)

	scanner := bufio.NewScanner(inputFile)
	scanner.Split(bufio.ScanLines)

	var lines []string

	for scanner.Scan() {
		lines = append(lines, scanner.Text())
	}

	return lines
}
