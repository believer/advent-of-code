open Belt.Array

type t = {values: array<(string, int)>}

let convertToNumbers = row =>
  switch row {
  | Some(r) => r->map(rowValue => Js.String.length(rowValue))
  | None => [0]
  }

let sortAndMatch = row =>
  row->Js.Array2.sortInPlace->Js.Array2.joinWith("")->Js.String2.match_(%re("/(.)\\1+/g"))

let findDuplicateCharacters = input =>
  input
  ->map(row => row |> Js.String.split("") |> sortAndMatch)
  ->map(convertToNumbers)
  ->map(Utils.removeDuplicates)
  ->reduce([], (acc, curr) => concat(acc, curr))
  ->sortAndMatch
  ->convertToNumbers
  ->reduce(1, (acc, curr) => acc * curr)
