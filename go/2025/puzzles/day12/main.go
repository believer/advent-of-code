package main

import (
	"fmt"
	"strings"

	"github.com/believer/aoc-utils/files"
	"github.com/believer/aoc-utils/utils"
)

// A bit troll-y :D From reading I thought it was going to be
// impossible. But, I started parsing the list and tried if the
// area would actually fit the size of the shapes and it worked!

func main() {
	fmt.Println("Part 1: ", part1("input.txt"))
}

type Region struct {
	width, height int
	counts        []int
}

func (r *Region) Area() int {
	return r.width * r.height
}

func part1(name string) (fits int) {
	paragraphs := files.ReadParagraphs(name)

	// Get size of packages
	shapes := []int{}

	for _, p := range paragraphs[:6] {
		shape := 0

		for _, s := range p[1:] {
			for v := range strings.SplitSeq(s, "") {
				if v == "#" {
					shape++
				}
			}
		}

		shapes = append(shapes, shape)
	}

	// Get region sizes and number of packages
	packages := [][]int{}
	var regions []Region

	for _, p := range paragraphs[6:] {
		for _, s := range p {
			counts := []int{}
			region := strings.Fields(s)
			area := strings.Split(region[0][:len(region[0])-1], "x")

			for _, r := range region[1:] {
				counts = append(counts, utils.MustIntFromString(r))
			}

			packages = append(packages, counts)

			regions = append(regions, Region{
				height: utils.MustIntFromString(area[0]),
				width:  utils.MustIntFromString(area[1]),
				counts: counts,
			})
		}
	}

	// Check if package sizes will fit inside the area
	for i, r := range regions {
		p := packages[i]
		size := 0

		for i, pp := range p {
			size += shapes[i] * pp
		}

		if r.Area() >= size {
			fits++
		}
	}

	return
}
