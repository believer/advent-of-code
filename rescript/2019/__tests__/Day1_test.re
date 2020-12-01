open Jest;
open Expect;

describe("PartOne", () => {
  test("test first example input", () => {
    Day1.PartOne.make([12]) |> expect |> toEqual(2)
  });

  test("test second example input", () => {
    Day1.PartOne.make([14]) |> expect |> toEqual(2)
  });

  test("test third example input", () => {
    Day1.PartOne.make([1969]) |> expect |> toEqual(654)
  });

  test("test third example input", () => {
    Day1.PartOne.make([100756]) |> expect |> toEqual(33583)
  });

  test("puzzle data", () => {
    Day1.PartOne.make(Day1Data.data) |> expect |> toEqual(3553700)
  });
});

describe("PartTwo", () => {
  test("test first example input", () => {
    Day1.PartTwo.make([14]) |> expect |> toEqual(2)
  });

  test("test second example input", () => {
    Day1.PartTwo.make([1969]) |> expect |> toEqual(966)
  });

  test("test third example input", () => {
    Day1.PartTwo.make([100756]) |> expect |> toEqual(50346)
  });

  test("puzzle data", () => {
    Day1.PartTwo.make(Day1Data.data) |> expect |> toEqual(5327664)
  });
});
