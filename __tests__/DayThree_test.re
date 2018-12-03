open Jest;
open Expect;

let exampleDataOne = [|"#1 @ 1,3: 4x4", "#2 @ 3,1: 4x4", "#3 @ 5,5: 2x2"|];

test("handles example input puzzle #1", () =>
  expect(SliceItOne.findOverlap(exampleDataOne)) |> toEqual(4)
);

test("solves puzzle #1", () =>
  expect(SliceItOne.findOverlap(DayThreeData.data)) |> toEqual(100261)
);
