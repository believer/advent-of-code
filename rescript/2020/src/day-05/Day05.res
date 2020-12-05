module Seat = {
  let rows = (0, 127)
  let columns = (0, 7)

  let halfRange = ((start, end)) => {
    ((float_of_int(end) -. float_of_int(start)) /. 2.)->Js.Math.round->int_of_float
  }

  let lowerHalf = ((start, end) as range) => {
    (start, end - halfRange(range))
  }

  let upperHalf = ((start, end) as range) => {
    (start + halfRange(range), end)
  }

  let findMySeat = () => {
    Day05Data.data->Js.Array2.map(seat => {
      let ((_, rowEnd), (_, colEnd)) = seat->Js.String2.split("")->Js.Array2.reduce(((
        row,
        col,
      ), c) => {
        switch c {
        | "F" => (lowerHalf(row), col)
        | "B" => (upperHalf(row), col)
        | "L" => (row, lowerHalf(col))
        | "R" => (row, upperHalf(col))
        | _ => (row, col)
        }
      }, (rows, columns))

      rowEnd * 8 + colEnd
    })
  }
}

let partOne = () => {
  let seats = Seat.findMySeat()

  Js.Array2.sortInPlaceWith(seats, (a, b) => a - b)->ignore
  Js.Array2.pop(seats)->Belt.Option.getWithDefault(0)
}

let partTwo = () => {
  let mySeat = ref(0)
  let seats = Seat.findMySeat()

  Js.Array2.sortInPlaceWith(seats, (a, b) => a - b)->ignore

  for i in 0 to Js.Array2.length(seats) {
    switch (seats->Belt.Array.get(i), seats->Belt.Array.get(i + 1)) {
    | (Some(curr), Some(next)) when curr + 1 != next => mySeat := curr + 1
    | _ => ()
    }
  }

  mySeat.contents
}

/* Result.make(partOne, partTwo) */
