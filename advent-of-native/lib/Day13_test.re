open TestFramework;

describe("Day 13 - Part 1", ({test}) => {
  test("solves puzzle", ({expect}) => {
    expect.int(Day13.PartOne.make(Day13_data.data)).toBe(286)
  })
});
