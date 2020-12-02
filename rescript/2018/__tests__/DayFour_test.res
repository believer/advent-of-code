open Jest
open Expect

let exampleDataOne = [
  "[1518-11-01 00:00] Guard #10 begins shift",
  "[1518-11-01 00:05] falls asleep",
  "[1518-11-01 00:25] wakes up",
  "[1518-11-02 00:50] wakes up",
  "[1518-11-01 00:30] falls asleep",
  "[1518-11-01 00:55] wakes up",
  "[1518-11-01 23:58] Guard #99 begins shift",
  "[1518-11-02 00:40] falls asleep",
  "[1518-11-03 00:29] wakes up",
  "[1518-11-03 00:05] Guard #10 begins shift",
  "[1518-11-03 00:24] falls asleep",
  "[1518-11-05 00:03] Guard #99 begins shift",
  "[1518-11-05 00:45] falls asleep",
  "[1518-11-04 00:36] falls asleep",
  "[1518-11-04 00:46] wakes up",
  "[1518-11-04 00:02] Guard #99 begins shift",
  "[1518-11-05 00:55] wakes up",
]

test("handles example input puzzle #1", () =>
  expect(GuardOne.findSleepy(exampleDataOne)) |> toEqual(240)
)

test("solves puzzle #1", () => expect(GuardOne.findSleepy(DayFourData.data)) |> toEqual(19025))

test("handles example input puzzle #2", () =>
  expect(GuardTwo.findFrequentSleeper(exampleDataOne)) |> toEqual(4455)
)

test("solves puzzle #2", () =>
  expect(GuardTwo.findFrequentSleeper(DayFourData.data)) |> toEqual(23776)
)
