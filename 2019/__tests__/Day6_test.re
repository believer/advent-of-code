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

describe("Day 6 - Part 2", () => {
  test("test example input", () => {
    let data = [
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
      "K)YOU",
      "I)SAN",
    ];

    Day6.PartTwo.make(data) |> expect |> toEqual(4);
  });

  test("solves puzzle", () => {
    let s = Performance.now();
    let data = Day6.PartTwo.make(Day6Data.data |> Array.to_list);
    let e = Performance.now();

    Performance.Time.make(s, e);
    data |> expect |> toEqual(370);
  });
});
