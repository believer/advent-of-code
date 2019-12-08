open Jest;
open Expect; /*describe("PartOne", () => {*/

describe("PartOne", () => {
  test("test example input", () => {
    Day6.PartOne.make([|
      "COM)B",
      "B)C",
      "C)D",
      "D)E",
      "E)F",
      "B)G",
      "G)H",
      "D)I",
      "E)J",
      "J)K",
      "K)L",
    |])
    |> expect
    |> toEqual(42)
  });

  test("solves puzzle", () => {
    Day6.PartOne.make(Day6Data.data) |> expect |> toEqual(333679)
  });
});
