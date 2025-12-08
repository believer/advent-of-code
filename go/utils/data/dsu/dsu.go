package dsu

// Disjoint Set Union (DSU)
type DSU struct {
	Parent []int
	Size   []int
	Count  int
}

func New(n int) *DSU {
	dsu := &DSU{
		Parent: make([]int, n),
		Size:   make([]int, n),
		Count:  n,
	}

	for i := range n {
		dsu.Parent[i] = i // Each element is its own parent
		dsu.Size[i] = 1   // Each set starts with size 1
	}

	return dsu
}

func (d *DSU) Find(i int) int {
	if d.Parent[i] == i {
		return i
	}

	// Path compression: set parent[i] directly to the root
	d.Parent[i] = d.Find(d.Parent[i])

	return d.Parent[i]
}

// Merge sets that contain i and j. Report whether a successful
// merge occurred (when i and j were in different sets)
func (d *DSU) Union(i, j int) bool {
	a := d.Find(i)
	b := d.Find(j)

	if a == b {
		return false
	}

	// Swap to make sure that a is the larger set's root
	if d.Size[a] < d.Size[b] {
		a, b = b, a
	}

	d.Parent[b] = a
	d.Size[a] += d.Size[b]
	d.Count -= 1

	return true
}
