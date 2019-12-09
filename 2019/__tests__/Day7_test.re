open Jest;
open Expect;

describe("PartOne", () => {
  test("example data", () => {
    Day7.PartOne.make([|
      3,
      15,
      3,
      16,
      1002,
      16,
      10,
      16,
      1,
      16,
      15,
      15,
      4,
      15,
      99,
      0,
      0,
    |])
    |> expect
    |> toEqual(43210)
  });

  Skip.test("puzzle data", () => {
    Day7.PartOne.make(Day7Data.data) |> expect |> toEqual(9025675)
  });
});
