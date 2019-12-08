open Jest;
open Expect; /*describe("PartOne", () => {*/

describe("PartOne", () => {
  test("solves puzzle", () => {
    Day8.PartOne.make(Day8Data.data) |> expect |> toEqual(1935)
  })
});
