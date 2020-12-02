open Jest
open Expect

test("handles positive increment", () => DayOne.One.make([1]) |> expect |> toEqual(1))

test("handles multiple positive increment", () => DayOne.One.make([1, 1]) |> expect |> toEqual(2))

test("handles mixed", () => DayOne.One.make([1, 1, -2]) |> expect |> toEqual(0))

test("handles puzzle data", () => DayOne.One.make(DayOneData.data) |> expect |> toEqual(411))

test("handles medium puzzle input #2", () =>
  DayOne.Two.make([3, 3, 4, -2, -4], ()) |> expect |> toEqual(10)
)

test("handles medium puzzle input #3", () =>
  DayOne.Two.make([-6, 3, 8, 5, -6], ()) |> expect |> toEqual(5)
)

test("handles medium puzzle input #4", () =>
  DayOne.Two.make([7, 7, -2, -7, -4], ()) |> expect |> toEqual(14)
)

test("handles full puzzle input #2", () =>
  DayOne.Two.make(DayOneData.data, ()) |> expect |> toEqual(56360)
)
