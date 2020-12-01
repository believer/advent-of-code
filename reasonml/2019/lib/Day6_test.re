open Core;
open TestFramework;

let file = In_channel.read_lines("./lib/inputs/Day6.txt");

describe("Day 6 - Part 1", ({test}) => {
  test("test example input", ({expect}) => {
    let data = [
      "COM)B",
      "B)C",
      "C)D",
      "D)E",
      "E)F",
      "B)G",
      "G)H",
      "D)I",
      "E)J",
      "J)K",
      "K)L",
    ];

    expect.int(Day6.PartOne.make(data)).toBe(42);
  });

  test("solves puzzle", ({expect}) => {
    expect.int(Day6.PartOne.make(file)).toBe(333679)
  });
});

describe("Day 6 - Part 2", ({test}) => {
  test("test example input", ({expect}) => {
    let data = [
      "COM)B",
      "B)C",
      "C)D",
      "D)E",
      "E)F",
      "B)G",
      "G)H",
      "D)I",
      "E)J",
      "J)K",
      "K)L",
      "K)YOU",
      "I)SAN",
    ];

    expect.int(Day6.PartTwo.make(data)).toBe(4);
  });

  test("solves puzzle", ({expect}) => {
    expect.int(Day6.PartTwo.make(file)).toBe(370)
  });
});
