open Jest;
open Expect;

test("handles positive increment", () =>
  expect(CalibrateOne.deviceFrequency(~input="+1", ()).result) |> toEqual(1)
);

test("handles multiple positive increments", () =>
  expect(CalibrateOne.deviceFrequency(~input="+1, +1", ()).result)
  |> toEqual(2)
);

test("handles mixed values", () =>
  expect(CalibrateOne.deviceFrequency(~input="+1, +1, -2", ()).result)
  |> toEqual(0)
);

test("handles puzzle input #1", () =>
  expect(CalibrateOne.deviceFrequency(~input=DayOneData.data, ()).result)
  |> toEqual(411)
);

test("handles small puzzle input #2", () =>
  expect(CalibrateTwo.findDuplicateFrequency(~input="+1, -1", ()))
  |> toEqual(0)
);

test("handles medium puzzle input #2", () =>
  expect(
    CalibrateTwo.findDuplicateFrequency(~input="+3, +3, +4, -2, -4", ()),
  )
  |> toEqual(10)
);

test("handles medium puzzle input #2", () =>
  expect(
    CalibrateTwo.findDuplicateFrequency(~input="-6, +3, +8, +5, -6", ()),
  )
  |> toEqual(5)
);

test("handles medium puzzle input #2", () =>
  expect(
    CalibrateTwo.findDuplicateFrequency(~input="+7, +7, -2, -7, -4", ()),
  )
  |> toEqual(14)
);

Skip.test("handles full puzzle input #2 - (long running task)", () =>
  expect(CalibrateTwo.findDuplicateFrequency(~input=DayOneData.data, ()))
  |> toEqual(56360)
);
