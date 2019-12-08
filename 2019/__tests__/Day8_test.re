open Jest;
open Expect; /*describe("PartOne", () => {*/

describe("PartOne", () => {
  test("solves puzzle", () => {
    Day8.PartOne.make(Day8Data.data) |> expect |> toEqual(1935)
  })
});

describe("PartTwo", () => {
  test("solves puzzle", () => {
    Day8.PartTwo.make(Day8Data.data)
    |> expect
    |> toEqual([|
         ".11..1111.1....1..1.1....",
         "1..1.1....1....1..1.1....",
         "1....111..1....1..1.1....",
         "1....1....1....1..1.1....",
         "1..1.1....1....1..1.1....",
         ".11..1....1111..11..1111.",
         "",
       |])
  })
});
