@val external parseInt: (string, int) => int = "parseInt"

module Seat = {
  let findAirplaneSeats = () => {
    Day05Data.data->Js.Array2.map(seat => {
      let seatId = seat->Js.String2.split("")->Js.Array2.reduce((acc, c) => {
        switch c {
        | "F" | "L" => acc ++ "0"
        | "B" | "R" => acc ++ "1"
        | _ => acc
        }
      }, "")

      parseInt(seatId, 2)
    })
  }
}

let partOne = () => {
  let seats = Seat.findAirplaneSeats()

  Js.Array2.sortInPlaceWith(seats, (a, b) => a - b)->ignore
  Js.Array2.pop(seats)->Belt.Option.getWithDefault(0)
}

let partTwo = () => {
  let mySeat = ref(0)
  let seats = Seat.findAirplaneSeats()

  Js.Array2.sortInPlaceWith(seats, (a, b) => a - b)->ignore

  for i in 0 to Js.Array2.length(seats) {
    switch (seats->Belt.Array.get(i), seats->Belt.Array.get(i + 1)) {
    | (Some(curr), Some(next)) when curr + 1 != next => mySeat := curr + 1
    | _ => ()
    }
  }

  mySeat.contents
}

Result.make(partOne, partTwo)
