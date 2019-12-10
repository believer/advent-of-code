open Jest;
open Expect;

describe("PartOne", () => {
  test("example data", () => {
    Day10.PartOne.make([|
      "......#.#.",
      "#..#.#....",
      "..#######.",
      ".#.#.###..",
      ".#..#.....",
      "..#....#.#",
      "#..#....#.",
      ".##.#..###",
      "##...#..#.",
      ".#....####",
    |])
    |> expect
    |> toEqual(33)
  })
});
