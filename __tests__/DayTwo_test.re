open Jest;
open Expect;

let exampleData = [|
  "abcdef",
  "bababc",
  "abbcde",
  "abcccd",
  "aabcdd",
  "abcdee",
  "ababab",
|];

test("handles example input puzzle #1", () =>
  expect(InventoryOne.findDuplicateCharacters(exampleData)) |> toEqual(12)
);

test("solves puzzle #1", () =>
  expect(InventoryOne.findDuplicateCharacters(DayTwoData.data))
  |> toEqual(7163)
);
