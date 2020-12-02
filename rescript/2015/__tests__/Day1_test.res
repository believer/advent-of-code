open Jest
open Expect

describe("PartOne", () => {
  test("test first example input", () => {
    Day1.PartOne.make("(())") |> expect |> toEqual(0)
  })

  test("test second example input", () => {
    Day1.PartOne.make("()()") |> expect |> toEqual(0)
  })

  test("test second example input", () => {
    Day1.PartOne.make("))(((((") |> expect |> toEqual(3)
  })

  test("puzzle data", () => {
    Day1.PartOne.make(Day1Data.data) |> expect |> toEqual(232)
  })
})

describe("PartTwo", () => {
  test("test first example input", () => {
    Day1.PartTwo.make(")") |> expect |> toEqual(Some(1))
  })

  test("test second example input", () => {
    Day1.PartTwo.make("()())") |> expect |> toEqual(Some(5))
  })

  test("puzzle data", () => {
    Day1.PartTwo.make(Day1Data.data) |> expect |> toEqual(Some(1783))
  })
})
