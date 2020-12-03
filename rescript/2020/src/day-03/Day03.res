// Had to convert to float in order to be able to multiply the values
// in part two. Multiplying int would overflow and the result would be incorrect.
let slopeFinder = (input, rs, cs) => {
  let rows = Js.Array2.length(input)
  let trees = ref(0)
  let row = ref(0)
  let col = ref(0)

  while row.contents < rows {
    let cols = Js.String2.length(input[row.contents])

    if input[row.contents]->Js.String2.get(mod(col.contents, cols)) == "#" {
      trees.contents = trees.contents + 1
    }

    row := row.contents + rs
    col := col.contents + cs
  }

  trees.contents->float_of_int
}

let partOne = () => slopeFinder(Day03Data.data, 1, 3)

let partTwo = () =>
  [(1, 1), (1, 3), (1, 5), (1, 7), (2, 1)]
  ->Js.Array2.map(((rs, cs)) => slopeFinder(Day03Data.data, rs, cs))
  ->Js.Array2.reduce((acc, v) => acc *. v, 1.0)

Result.make(partOne, partTwo)
