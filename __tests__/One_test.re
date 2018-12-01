open Jest;
open One;
open Expect;

test("handles positive increment", () =>
  expect(deviceFrequency(~input="+1", ()).result) |> toEqual(1)
);

test("handles multiple positive increments", () =>
  expect(deviceFrequency(~input="+1, +1", ()).result) |> toEqual(2)
);

test("handles mixed values", () =>
  expect(deviceFrequency(~input="+1, +1, -2", ()).result) |> toEqual(0)
);

test("handles puzzle input #1", () =>
  expect(deviceFrequency(~input=OneTestData.data, ()).result)
  |> toEqual(411)
);

test("handles small puzzle input #2", () =>
  expect(findDuplicateFrequency(~input="+1, -1", ())) |> toEqual(0)
);

test("handles medium puzzle input #2", () =>
  expect(findDuplicateFrequency(~input="+3, +3, +4, -2, -4", ()))
  |> toEqual(10)
);

test("handles medium puzzle input #2", () =>
  expect(findDuplicateFrequency(~input="-6, +3, +8, +5, -6", ()))
  |> toEqual(5)
);

test("handles medium puzzle input #2", () =>
  expect(findDuplicateFrequency(~input="+7, +7, -2, -7, -4", ()))
  |> toEqual(14)
);

test("handles full puzzle input #2", () =>
  expect(findDuplicateFrequency(~input=OneTestData.data, ()))
  |> toEqual(56360)
);
