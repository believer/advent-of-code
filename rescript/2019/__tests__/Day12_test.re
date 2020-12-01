open Jest;
open Expect;

describe("Day 12 - Part 1", () => {
  test("example input", () => {
    let data =
      Day12Data.Moon.(
        [|
          make(-1., 0., 2.),
          make(2., -10., -7.),
          make(4., -8., 8.),
          make(3., 5., -1.),
        |]
      );

    Day12.PartOne.make(data, ~iterations=10.) |> expect |> toEqual(179.);
  });

  test("solves puzzle", () => {
    Day12.PartOne.make(Day12Data.data, ~iterations=1000.)
    |> expect
    |> toEqual(6678.)
  });
});

describe("Day 12 - Part 2", () => {
  test("example input", () => {
    let data =
      Day12Data.Moon.(
        [|
          make(-1., 0., 2.),
          make(2., -10., -7.),
          make(4., -8., 8.),
          make(3., 5., -1.),
        |]
      );

    Day12.PartTwo.make(data) |> expect |> toEqual(2772.);
  });

  test("solves puzzle", () => {
    // Answer too big for int
    Day12.PartTwo.make(Day12Data.data)
    |> expect
    |> toEqual(496734501382552.)
  });
});
