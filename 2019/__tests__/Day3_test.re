open Jest;
open Expect;

describe("PartOne", () => {
  test("test example input", () =>
    Day3.PartOne.make((
      ["R75", "D30", "R83", "U83", "L12", "D49", "R71", "U7", "L72"],
      ["U62", "R66", "U55", "R34", "D71", "R55", "D58", "R83"],
    ))
    |> expect
    |> toEqual(159)
  );

  test("puzzle data", () =>
    Day3.PartOne.make(Day3Data.data) |> expect |> toEqual(651)
  );
});

describe("PartTwo", () => {
  test("test example input", () =>
    Day3.PartTwo.make((
      ["R75", "D30", "R83", "U83", "L12", "D49", "R71", "U7", "L72"],
      ["U62", "R66", "U55", "R34", "D71", "R55", "D58", "R83"],
    ))
    |> expect
    |> toEqual(610)
  );

  test("should solve part two", () =>
    Day2.PartTwo.make(Day2Data.data) |> expect |> toEqual(Some("6533"))
  );
});
