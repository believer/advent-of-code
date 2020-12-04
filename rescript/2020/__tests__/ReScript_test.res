open Jest
open Expect

test("Day 1 | Part 1", () => {
  expect(Day01.partOne()) |> toEqual(898299)
})

test("Day 1 | Part 2", () => {
  expect(Day01.partTwo()) |> toEqual(143933922)
})

test("Day 2 | Part 1", () => {
  expect(Day02.partOne()) |> toEqual(524)
})

test("Day 2 | Part 2", () => {
  expect(Day02.partTwo()) |> toEqual(485)
})

test("Day 3 | Part 1", () => {
  expect(Day03.partOne()) |> toEqual(259.)
})

test("Day 3 | Part 2", () => {
  expect(Day03.partTwo()) |> toEqual(2224913600.)
})

test("Day 4 | Part 1", () => {
  expect(Day04.partOne()) |> toEqual(200)
})

test("Day 4 | Part 2", () => {
  expect(Day04.partTwo()) |> toEqual(116)
})
