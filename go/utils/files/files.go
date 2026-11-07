package files

import (
	"os"
	"path/filepath"
	"runtime"
	"strings"

	"github.com/believer/aoc-utils/utils"
)

// Borrowed from https://github.com/kylehoehns/aoc-2023-go/blob/main/pkg/files/filereader.go
// Seems like a great starting point for me to learn and adjust according to my needs.

// ReadLines reads a file and returns a slice of strings, one for each line
func ReadLines(name string) []string {
	return splitLines(readFile(name, callerFile()))
}

// Lines is a shorter name for [ReadLines]
func Lines(name string) []string {
	return splitLines(readFile(name, callerFile()))
}

// Ints reads a file and returns all integers found on each line.
// Lines without integers produce empty slices.
func Ints(name string) [][]int {
	lines := splitLines(readFile(name, callerFile()))
	result := make([][]int, len(lines))

	for i, line := range lines {
		result[i] = utils.Ints(line)
	}

	return result
}

// Read reads a file and returns a string containing the entire file
func Read(name string) string {
	return readFile(name, callerFile())
}

// ReadParagraphs reads a file and returns a slice of slices of strings, one for each line
// A gap of one or more blank lines is used to split the file into groups
func ReadParagraphs(name string) [][]string {
	lines := splitLines(readFile(name, callerFile()))

	var groups [][]string

	curGroup := make([]string, 0)

	for _, line := range lines {
		if line == "" {
			if len(curGroup) > 0 {
				groups = append(groups, curGroup)
				curGroup = make([]string, 0)
			}
		} else {
			curGroup = append(curGroup, line)
		}
	}

	if len(curGroup) > 0 {
		groups = append(groups, curGroup)
	}

	return groups
}

// Find the file that called the public function (two frames up)
func callerFile() string {
	_, callingFile, _, ok := runtime.Caller(2)

	if !ok {
		panic("unable to find caller so cannot build path to read file")
	}

	return callingFile
}

func readFile(name string, callingFile string) string {
	b, err := os.ReadFile(filepath.Join(filepath.Dir(callingFile), name))

	if err != nil {
		panic(err)
	}

	return string(b)
}

// Split on newlines. A single trailing newline doesn't create an extra line.
func splitLines(s string) []string {
	if s == "" {
		return nil
	}

	lines := strings.Split(strings.TrimSuffix(s, "\n"), "\n")

	for i, l := range lines {
		lines[i] = strings.TrimSuffix(l, "\r")
	}

	return lines
}
