open Jest;
open Expect;

describe("PartOne", () => {
  test("test example input", () => {
    Day4.PartOne.make(111110, 111112) |> expect |> toEqual(2)
  });

  test("puzzle data", () => {
    Day4.PartOne.make(206938, 679128) |> expect |> toEqual(1653)
  });
});

describe("PartTwo", () => {
  test("test example input", () => {
    Day4.PartTwo.make(111121, 111122) |> expect |> toEqual(1)
  });

  test("test example input", () => {
    Day4.PartTwo.make(112230, 112233) |> expect |> toEqual(1)
  });

  test("test example input", () => {
    Day4.PartTwo.make(123443, 123445) |> expect |> toEqual(1)
  });

  test("puzzle data", () => {
    Day4.PartTwo.make(206938, 679128) |> expect |> toEqual(1133)
  });
});
