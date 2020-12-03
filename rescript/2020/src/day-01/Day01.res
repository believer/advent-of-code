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

/* Result.make(partOne, partTwo) */
