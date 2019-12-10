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
  });

  test("example data", () => {
    Day10.PartOne.make([|
      "#.#...#.#.",
      ".###....#.",
      ".#....#...",
      "##.#.#.#.#",
      "....#.#.#.",
      ".##..###.#",
      "..#...##..",
      "..##....##",
      "......#...",
      ".####.###.",
    |])
    |> expect
    |> toEqual(35)
  });

  test("solves puzzle", () => {
    Day10.PartOne.make(Day10Data.data) |> expect |> toEqual(303)
  });
});
