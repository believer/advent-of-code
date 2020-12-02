let make = (partOne, partTwo) => {
  let t0 = Perf.now()
  Js.log("Part 1")
  Js.log2("Result: ", partOne())
  let t1 = Perf.now()
  Js.log2("Time: ", (t1 - t0)->Js.Int.toPrecisionWithPrecision(~digits=2) ++ " ms")

  let t0 = Perf.now()
  Js.log("\nPart 2")
  Js.log2("Result: ", partTwo())
  let t1 = Perf.now()
  Js.log2("Time: ", (t1 - t0)->Js.Int.toPrecisionWithPrecision(~digits=2) ++ " ms")
}
