open TestFramework;

describe("Day 12 - Part 1", ({test}) => {
  test("example input", ({expect}) => {
    let data = [
      (Data.Moon.Io, ((-1), 0, 2), (0, 0, 0)),
      (Europa, (2, (-10), (-7)), (0, 0, 0)),
      (Ganymede, (4, (-8), 8), (0, 0, 0)),
      (Callisto, (3, 5, (-1)), (0, 0, 0)),
    ];

    expect.int(Day12.PartOne.make(data, ~iterations=10)).toBe(179);
  });

  test("solves puzzle", ({expect}) => {
    expect.int(Day12.PartOne.make(Data.Day12.data, ~iterations=1000)).toBe(
      6678,
    )
  });
});
