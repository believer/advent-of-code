type performance = {now: unit => float}

@module("perf_hooks") external performance: performance = "performance"

let now = performance.now
