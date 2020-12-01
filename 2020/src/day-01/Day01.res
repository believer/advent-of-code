type performance = {now: unit => int}

@module("perf_hooks") external performance: performance = "performance"

let partOne = () => {
  let s = ref(0)

  for i in 0 to Js.Array2.length(Day01Input.data) {
    switch Day01Input.data->Belt.Array.get(i) {
    | Some(i) => {
        let v = 2020 - i

        if Day01Input.data->Js.Array2.includes(v) {
          s := i * v
        }
      }
    | _ => ()
    }
  }

  s.contents
}

let partTwo = () => {
  let s = ref(0)

  for i in 0 to Js.Array2.length(Day01Input.data) {
    for j in 0 to Js.Array2.length(Day01Input.data) {
      let v1 = Day01Input.data->Belt.Array.get(i)
      let v2 = Day01Input.data->Belt.Array.get(j)

      switch (v1, v2) {
      | (Some(v1), Some(v2)) =>
        if v1 + v2 > 2020 {
          ()
        } else {
          let v = 2020 - v1 - v2

          if Day01Input.data->Js.Array2.includes(v) {
            s := v1 * v2 * v
          }
        }
      | _ => ()
      }
    }
  }

  s.contents
}

let t0 = performance.now()
Js.log("Part 1")
Js.log2("Result: ", partOne())
let t1 = performance.now()
Js.log2("Time: ", (t1 - t0)->Js.Int.toPrecisionWithPrecision(~digits=2) ++ " ms")

let t0 = performance.now()
Js.log("\nPart 2")
Js.log2("Result: ", partTwo())
let t1 = performance.now()
Js.log2("Time: ", (t1 - t0)->Js.Int.toPrecisionWithPrecision(~digits=2) ++ " ms")
