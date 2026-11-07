package files

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func TestReadLines(t *testing.T) {
	assert.Equal(t, []string{"1", "2", "3"}, ReadLines("testdata/lines.txt"))
}

func TestRead(t *testing.T) {
	assert.Equal(t, "1\n2\n3\n", Read("testdata/lines.txt"))
}

func TestReadLinesMissingFilePanics(t *testing.T) {
	assert.Panics(t, func() { ReadLines("testdata/missing.txt") })
}

func TestReadLinesLongLine(t *testing.T) {
	lines := ReadLines("testdata/long.txt")

	assert.Len(t, lines, 1)
	assert.Len(t, lines[0], 200000)
}

func TestReadParagraphs(t *testing.T) {
	// Multiple blank lines in a row are one separator
	assert.Equal(
		t,
		[][]string{{"a", "b"}, {"c"}, {"d", "e"}},
		ReadParagraphs("testdata/paragraphs.txt"),
	)
}

func TestReadParagraphsIgnoresLeadingAndTrailingBlanks(t *testing.T) {
	assert.Equal(
		t,
		[][]string{{"a"}, {"b"}},
		ReadParagraphs("testdata/paragraphs_edges.txt"),
	)
}

func TestLines(t *testing.T) {
	assert.Equal(t, []string{"1", "2", "3"}, Lines("testdata/lines.txt"))
}

func TestInts(t *testing.T) {
	assert.Equal(t, [][]int{{-1, 2, 30}, {}}, Ints("testdata/ints.txt"))
}
