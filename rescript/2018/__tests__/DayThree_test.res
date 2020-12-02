open Jest
open Expect

let exampleDataOne = ["#1 @ 1,3: 4x4", "#2 @ 3,1: 4x4", "#3 @ 5,5: 2x2"]

test("handles example input puzzle #1", () =>
  expect(SliceIt.findOverlap(exampleDataOne)) |> toEqual(4)
)

test("solves puzzle #1", () => expect(SliceIt.findOverlap(DayThreeData.data)) |> toEqual(100261))

test("handles example input puzzle #2", () =>
  expect(SliceIt.findFabric(exampleDataOne)) |> toEqual("3")
)

test("solves puzzle #2", () => expect(SliceIt.findFabric(DayThreeData.data)) |> toEqual("251"))
