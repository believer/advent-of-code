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

  test("example data", () => {
    Day7.PartOne.make([|
      3,
      23,
      3,
      24,
      1002,
      24,
      10,
      24,
      1002,
      23,
      (-1),
      23,
      101,
      5,
      23,
      23,
      1,
      24,
      23,
      23,
      4,
      23,
      99,
      0,
      0,
    |])
    |> expect
    |> toEqual(54321)
  });

  test("example data", () => {
    Day7.PartOne.make([|
      3,
      31,
      3,
      32,
      1002,
      32,
      10,
      32,
      1001,
      31,
      (-2),
      31,
      1007,
      31,
      0,
      33,
      1002,
      33,
      7,
      33,
      1,
      33,
      31,
      31,
      1,
      32,
      31,
      31,
      4,
      31,
      99,
      0,
      0,
      0,
    |])
    |> expect
    |> toEqual(65210)
  });

  test("puzzle data", () => {
    Day7.PartOne.make(Day7Data.data) |> expect |> toEqual(255590)
  });
});

describe("PartTwo", () => {
  test("example data", () => {
    Day7.PartTwo.make([|
      3,
      26,
      1001,
      26,
      (-4),
      26,
      3,
      27,
      1002,
      27,
      2,
      27,
      1,
      27,
      26,
      27,
      4,
      27,
      1001,
      28,
      (-1),
      28,
      1005,
      28,
      6,
      99,
      0,
      0,
      5,
    |])
    |> expect
    |> toEqual(139629729)
  })
});
