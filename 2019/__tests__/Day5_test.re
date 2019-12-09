open Jest;
open Expect;

describe("PartOne", () => {
  test("test example input", () => {
    Day5.PartOne.make(
      [|3, 225, 1, 225, 6, 6, 1100, 1, 238, 225, 104, 0, 99|],
      (),
    )
    |> expect
    |> toEqual(0)
  });

  test("puzzle data", () => {
    Day5.PartOne.make(Day5Data.data, ()) |> expect |> toEqual(9025675)
  });
});
