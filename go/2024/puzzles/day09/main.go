package main

import (
	"fmt"
	"slices"
	"strconv"
	"strings"

	"github.com/believer/aoc-utils/files"
	"github.com/believer/aoc-utils/utils"
)

type File struct {
	id    string
	start int
	size  int
}

// Did string manipulation for part 1 at first, but of course that became
// hard to manage with file IDs bigger than 9 like the example input.
// Changing to a slice didn't change the code much and even simplified some parts.
func main() {
	fmt.Println("Part 1: ", part1("input.txt"))
	fmt.Println("Part 2: ", part2("input.txt"))
}

func part1(name string) int {
	line := strings.TrimSpace(files.Read(name))
	diskMap, fileBlocks := createDiskMap(line)

	// Compact file system
	for i := 1; i < len(diskMap); i++ {
		rune := diskMap[len(diskMap)-i]

		if rune == "." {
			continue
		}

		firstEmpty := slices.Index(diskMap, ".")

		// The blocks have been compacted
		if firstEmpty == fileBlocks {
			break
		}

		diskMap[firstEmpty] = rune
		diskMap[len(diskMap)-i] = "."
	}

	return calculateChecksum(diskMap)
}

func part2(name string) int {
	line := strings.TrimSpace(files.Read(name))
	diskMap, _ := createDiskMap(line)
	fileMap := map[string]File{}

	// Create map of all files
	for i, id := range diskMap {
		if id == "." {
			continue
		}

		if file, ok := fileMap[id]; ok {
			file.size++
			fileMap[id] = file
		} else {
			fileMap[id] = File{id: id, start: i, size: 1}
		}
	}

	// Convert map to slice for sorting
	files := []File{}

	for _, file := range fileMap {
		files = append(files, file)
	}

	// Sort files by file ID descending since we're going backwards
	slices.SortFunc(files, func(a, b File) int {
		return utils.MustIntFromString(b.id) - utils.MustIntFromString(a.id)
	})

	// Move files to suitable positions
	for _, file := range files {
		newPosition := findFreeSpace(diskMap, file)

		if newPosition == -1 {
			continue
		}

		// Move file
		for i := file.start; i < file.start+file.size; i++ {
			diskMap[i] = "."
		}

		for i := 0; i < file.size; i++ {
			diskMap[newPosition+i] = file.id
		}
	}

	return calculateChecksum(diskMap)
}

func createDiskMap(input string) ([]string, int) {
	disk := []string{}
	fileId := 0
	blocks := 0

	for i, digit := range strings.Split(input, "") {
		digitAsInt := utils.MustIntFromString(digit)

		for range digitAsInt {
			if i%2 == 0 {
				disk = append(disk, strconv.Itoa(fileId))
				blocks += 1
			} else {
				disk = append(disk, ".")
			}
		}

		if i%2 == 0 {
			fileId++
		}
	}

	return disk, blocks
}

func findFreeSpace(disk []string, file File) int {
	currentSize := 0
	currentStart := -1

	// Find a space that fits the file
	for i := range file.start {
		if disk[i] == "." {
			if currentStart == -1 {
				currentStart = i
			}

			currentSize++

			if currentSize == file.size {
				return currentStart
			}
		} else {
			currentSize = 0
			currentStart = -1
		}
	}

	return -1
}

func calculateChecksum(disk []string) int {
	checksum := 0

	for i, digit := range disk {
		if digit == "." {
			continue
		}

		checksum += i * utils.MustIntFromString(digit)
	}

	return checksum
}
