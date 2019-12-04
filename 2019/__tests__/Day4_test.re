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
