package main

import (
	"fmt"
	"sync"

	"github.com/believer/aoc-2024/utils/files"
	"github.com/believer/aoc-2024/utils/grid"
)

// Find all distances using BFS. Then use the manhattan distance
// as the maximum allowed cheat time. The difference in distance
// between the points needs to be at least 100 (otherwise we couldn't
// cheat for 100). We also need to take into account the time to cheat
// so the minimum distance needs to be 102.
func main() {
	fmt.Println("Part 1: ", part1("input.txt"))
	fmt.Println("Part 2: ", part2("input.txt"))
}

func part1(name string) int {
	return race(name, 2)
}

func part2(name string) int {
	return race(name, 20)
}

func race(name string, maxCheatTime int) int {
	lines := files.ReadLines(name)
	cheats := 0

	raceTrack := grid.New(lines)
	start := raceTrack.Find('S')
	end := raceTrack.Find('E')

	queue := []grid.Point{start}
	distances := map[grid.Point]int{}

	for len(queue) > 0 {
		current := queue[0]
		queue = queue[1:]

		if current == end {
			continue
		}

		for _, direction := range grid.CARDINALS {
			next := current.Add(direction)

			if _, ok := distances[next]; ok {
				continue
			}

			if value, ok := raceTrack.Contains(next); ok && value != '#' {
				queue = append(queue, next)
				distances[next] = distances[current] + 1
			}
		}
	}

	var mu sync.Mutex
	var wg sync.WaitGroup

	for p := range distances {
		wg.Add(1)

		go func(p grid.Point) {
			defer wg.Done()

			for p2 := range distances {
				d := p.ManhattanDistance(p2)

				if d > maxCheatTime {
					continue
				}

				if distances[p2]-distances[p]-d >= 100 {
					mu.Lock()
					cheats++
					mu.Unlock()
				}
			}
		}(p)
	}

	wg.Wait()

	return cheats
}
