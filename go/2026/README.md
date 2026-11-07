# Advent of Code 2026

Go for main solutions.

## Days

| Day | #1  | #1 Answer | #2  | #2 Answer |
| --- | --- | --------: | --- | --------: |

## Benchmarks

Using Go's built-in benchmarking with the [testing](https://pkg.go.dev/testing#hdr-Benchmarks) package. Computer is a 2021 MacBook Pro M1 Pro, 32 GB RAM.

| Day | #1 | #1 Memory | #2 | #2 Memory | Time improvement\* | Memory improvement\* |
| --- | -: | --------: | -: | --------: | ------------------ | -------------------- |

Times are ns/op and memory is B/op allocated (from `-benchmem`). \* compared to first solution. A negative memory improvement means the current solution allocates more than the first one did.

### Previous solutions

| Day | #1 | #1 Memory | #2 | #2 Memory | Improvement | Link |
| --: | -: | --------: | -: | --------: | ----------: | ---- |

## Running

Run a day

```
make run DAY=<number>
```

Run test for a day

```
make test DAY=<number>
```

Create a new day (needs `cookie.txt` with the session cookie)

```
make create-day DAY=<number>
```
