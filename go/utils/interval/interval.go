package interval

type Range struct {
	Start int
	End   int
}

func (r *Range) Contains(v int) bool {
	return v >= r.Start && v <= r.End
}
