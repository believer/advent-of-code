open Jest;
open Expect;

describe("PartOne", () => {
  test("puzzle data", () => {
    Day5Part1.make(Day5Data.data) |> expect |> toEqual(9025675)
  })
});

describe("PartTwo", () => {
  test("puzzle data", () => {
    Day5Part2.make(Day5Data.data) |> expect |> toEqual(11981754)
  })
});
