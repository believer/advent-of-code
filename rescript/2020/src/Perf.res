type performance = {now: unit => int}

@module("perf_hooks") external performance: performance = "performance"

let now = performance.now
