.PHONE: create-day test bench run get-input

DAY ?= $(shell date +%-d)
PADDED_DAY = $(shell printf "%02d" $(DAY))

create-day:
	@cp -r ./template puzzles/day$(PADDED_DAY)
	@$(MAKE) get-input
	@echo "Day $(DAY) created"

test:
	@go test ./puzzles/day$(PADDED_DAY)

# run=XXX provide a regex that doesn't match any tests, effectively
# skipping the testing phase and just running benchmarks
bench:
	@go test -run=XXX -bench=. ./puzzles/day$(PADDED_DAY)

run:
	@go run ./puzzles/day$(PADDED_DAY)/main.go

get-input:
	@curl --silent --header @cookie.txt --output ./puzzles/day$(PADDED_DAY)/input.txt https://adventofcode.com/2024/day/$(DAY)/input 
