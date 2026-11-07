.PHONY: help create-day test test-all bench run get-input

.DEFAULT_GOAL := help

DAY ?= $(shell date +%-d)
PADDED_DAY = $(shell printf "%02d" $(DAY))

help: ## Show this help
	@echo "Usage: make <target> [DAY=n]"
	@echo ""
	@echo "Targets:"
	@grep -E '^[a-zA-Z_-]+:.*?## ' $(MAKEFILE_LIST) | \
		awk 'BEGIN {FS = ":.*?## "}; {printf "  \033[36m%-12s\033[0m %s\n", $$1, $$2}'
	@echo ""
	@echo "Variables:"
	@echo "  DAY          Puzzle day (default: today, currently $(DAY))"

create-day: ## Create puzzles/dayXX from template and download input
	@cp -r ./template puzzles/day$(PADDED_DAY)
	@$(MAKE) get-input
	@echo "Day $(DAY) created"

test: ## Run tests for the given day
	@go test ./puzzles/day$(PADDED_DAY)

test-all: ## Run tests for all days
	@go test ./...

# run=XXX provide a regex that doesn't match any tests, effectively
# skipping the testing phase and just running benchmarks
bench: ## Run benchmarks for the given day (skips tests)
	@go test -run=XXX -bench=. -benchmem ./puzzles/day$(PADDED_DAY)

run: ## Run the solution for the given day
	@go run ./puzzles/day$(PADDED_DAY)/main.go

get-input: ## Download puzzle input (needs cookie.txt)
	@curl --silent --header @cookie.txt --output ./puzzles/day$(PADDED_DAY)/input.txt https://adventofcode.com/2026/day/$(DAY)/input
