open Jest;
open Expect;

describe("PartOne", () => {
  test("test example input", () => {
    Day2.Computer.make([|1, 0, 0, 0, 99|], ~noun=0, ~verb=0, ())
    |> expect
    |> toEqual([|2, 0, 0, 0, 99|])
  });

  test("test example input", () => {
    Day2.Computer.make([|2, 3, 0, 3, 99|], ~noun=3, ~verb=0, ())
    |> expect
    |> toEqual([|2, 3, 0, 6, 99|])
  });

  test("test example input", () => {
    Day2.Computer.make([|2, 4, 4, 5, 99, 0|], ~noun=4, ~verb=4, ())
    |> expect
    |> toEqual([|2, 4, 4, 5, 99, 9801|])
  });

  test("test example input", () => {
    Day2.Computer.make([|1, 1, 1, 4, 99, 5, 6, 0, 99|], ~noun=1, ~verb=1, ())
    |> expect
    |> toEqual([|30, 1, 1, 4, 2, 5, 6, 0, 99|])
  });

  test("puzzle data", () => {
    Day2.PartOne.make(Day2Data.data) |> expect |> toEqual(Some(3790689))
  });
});

describe("PartTwo", () => {
  test("should solve part two", () => {
    Day2.PartTwo.make(Day2Data.data) |> expect |> toEqual(Some("6533"))
  })
});
