open Jest;
open Expect;

describe("PartOne", () => {
  test("test example input", () => {
    Day5.Computer.make([|1, 0, 0, 0, 99|], ())
    |> expect
    |> toEqual([|2, 0, 0, 0, 99|])
  });

  test("puzzle data", () => {
    Day2.PartOne.make(Day2Data.data) |> expect |> toEqual(Some(3790689))
  });
});
