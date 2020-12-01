open TestFramework;

describe("Day 2 - Part 1", ({test}) => {
  test("test example input", ({expect}) => {
    expect.array(
      DayTwo.Computer.make([|1, 0, 0, 0, 99|], ~noun=0, ~verb=0, ()),
    ).
      toEqual([|
      2,
      0,
      0,
      0,
      99,
    |])
  });

  test("test example input", ({expect}) => {
    expect.array(
      DayTwo.Computer.make([|2, 3, 0, 3, 99|], ~noun=3, ~verb=0, ()),
    ).
      toEqual([|
      2,
      3,
      0,
      6,
      99,
    |])
  });

  test("test example input", ({expect}) => {
    expect.array(
      DayTwo.Computer.make([|2, 4, 4, 5, 99, 0|], ~noun=4, ~verb=4, ()),
    ).
      toEqual([|
      2,
      4,
      4,
      5,
      99,
      9801,
    |])
  });

  test("test example input", ({expect}) => {
    expect.array(
      DayTwo.Computer.make(
        [|1, 1, 1, 4, 99, 5, 6, 0, 99|],
        ~noun=1,
        ~verb=1,
        (),
      ),
    ).
      toEqual([|
      30,
      1,
      1,
      4,
      2,
      5,
      6,
      0,
      99,
    |])
  });

  test("puzzle data", ({expect}) => {
    expect.option(DayTwo.PartOne.make(Data.Day2.data)).toBe(Some(3790689))
  });
});

describe("Day 2 - Part 2", ({test}) => {
  test("should solve part two", ({expect}) => {
    expect.option(DayTwo.PartTwo.make(Data.Day2.data)).toBe(Some("6533"))
  })
});
