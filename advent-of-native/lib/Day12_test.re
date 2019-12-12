open TestFramework;

describe("Day 12 - Part 1", ({test}) => {
  test("example input", ({expect}) => {
    let data =
      Data.Moon.[
        make(-1, 0, 2),
        make(2, -10, -7),
        make(4, -8, 8),
        make(3, 5, -1),
      ];

    expect.int(Day12.PartOne.make(data, ~iterations=10)).toBe(179);
  });

  test("solves puzzle", ({expect}) => {
    expect.int(Day12.PartOne.make(Data.Day12.data, ~iterations=1000)).toBe(
      6678,
    )
  });
});

describe("Day 12 - Part 2", ({test}) => {
  test("example input", ({expect}) => {
    let data =
      Data.Moon.[
        make(-1, 0, 2),
        make(2, -10, -7),
        make(4, -8, 8),
        make(3, 5, -1),
      ];

    expect.int(Day12.PartTwo.make(data)).toBe(2772);
  });

  test("solves puzzle", ({expect}) => {
    expect.int(Day12.PartTwo.make(Data.Day12.data)).toBe(496734501382552)
  });
});
