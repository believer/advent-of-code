package graph

type Edge struct {
	P1, P2   int
	Distance int
}

type Edges []Edge

// Sort interface
func (e Edges) Len() int           { return len(e) }
func (e Edges) Swap(i, j int)      { e[i], e[j] = e[j], e[i] }
func (e Edges) Less(i, j int) bool { return e[i].Distance < e[j].Distance }
