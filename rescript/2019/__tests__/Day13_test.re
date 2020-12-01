open Jest;
open Expect;

describe("PartOne", () => {
  test("solves puzzle", () => {
    Day13.PartOne.make(Day13Data.data) |> expect |> toEqual(286)
  })
});
