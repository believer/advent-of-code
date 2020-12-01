open Jest;
open Expect;

let exampleDataOne = [|
  "abcdef",
  "bababc",
  "abbcde",
  "abcccd",
  "aabcdd",
  "abcdee",
  "ababab",
|];

test("handles example input puzzle #1", () =>
  expect(InventoryOne.findDuplicateCharacters(exampleDataOne))
  |> toEqual(12)
);

test("solves puzzle #1", () =>
  expect(InventoryOne.findDuplicateCharacters(DayTwoData.data))
  |> toEqual(7163)
);

let exampleDataTwo = [|
  "abcde",
  "fghij",
  "klmno",
  "pqrst",
  "fguij",
  "axcye",
  "wvxyz",
|];

test("levenshtein similarity", () =>
  expect(
    Levenshtein.similarity(
      "bghfbsyxznoumkjleevacpwqtr",
      "bghfbtypznoumkjlxevacpwqtr",
    ),
  )
  |> toBeGreaterThan(0.8)
);

test("handles example input puzzle #2", () =>
  expect(InventoryTwo.findBoxes(exampleDataTwo)) |> toEqual("fgij")
);

test("solves puzzle #2", () =>
  expect(InventoryTwo.findBoxes(DayTwoData.data))
  |> toEqual("ighfbyijnoumxjlxevacpwqtr")
);
