open TestFramework;

describe("Day 1", ({test}) => {
  describe("Part 1", _ => {
    test("handles one param", ({expect}) => {
      expect.int(DayOne.PartOne.make([12])).toBe(2)
    });

    test("handles two params", ({expect}) => {
      expect.int(DayOne.PartOne.make([14])).toBe(2)
    });

    test("handles larger input", ({expect}) => {
      expect.int(DayOne.PartOne.make([1969])).toBe(654)
    });

    test("handles even larger input", ({expect}) => {
      expect.int(DayOne.PartOne.make([100756])).toBe(33583)
    });

    test("solves puzzle", ({expect}) => {
      expect.int(DayOne.PartOne.make(Data.Day1.data)).toBe(3553700)
    });
  });

  describe("Part 2", _ => {
    test("handles two params", ({expect}) => {
      expect.int(DayOne.PartTwo.make([14])).toBe(2)
    });

    test("handles larger input", ({expect}) => {
      expect.int(DayOne.PartTwo.make([1969])).toBe(966)
    });

    test("handles even larger input", ({expect}) => {
      expect.int(DayOne.PartTwo.make([100756])).toBe(50346)
    });

    test("solves puzzle", ({expect}) => {
      expect.int(DayOne.PartTwo.make(Data.Day1.data)).toBe(5327664)
    });
  });
});
