PADDED_DAY = $(shell printf "%02d" $(DAY))

create-day:
	@cp -r ./template puzzles/day$(PADDED_DAY)

test:
	@go test ./puzzles/day$(PADDED_DAY)

run:
	@go run ./puzzles/day$(PADDED_DAY)/main.go
