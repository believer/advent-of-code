open Jest;
open Expect;

test("is setup", () => {
  expect(Day1.test) |> toEqual("test")
});

test("has global belt", () => {
  expect(Day1.belt) |> toEqual(3)
});
