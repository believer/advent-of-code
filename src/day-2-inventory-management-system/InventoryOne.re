type t = {values: array((string, int))};

let convertToNumbers = row =>
  switch (row) {
  | Some(r) => r->Belt.Array.map(rowValue => Js.String.length(rowValue))
  | None => [|0|]
  };

let sortAndMatch = row =>
  row
  |> Js.Array.sortInPlace
  |> Js.Array.joinWith("")
  |> Js.String.match([%re "/(.)\\1+/g"]);

let removeDuplicates = row =>
  row->Belt.Array.reduce([||], (acc, curr) =>
    Js.Array.includes(curr, acc) ? acc : Belt.Array.concat(acc, [|curr|])
  );

let findDuplicateCharacters = input =>
  input
  ->Belt.Array.map(row => row |> Js.String.split("") |> sortAndMatch)
  ->Belt.Array.map(convertToNumbers)
  ->Belt.Array.map(removeDuplicates)
  ->Belt.Array.reduce([||], (acc, curr) => Belt.Array.concat(acc, curr))
  ->sortAndMatch
  ->convertToNumbers
  ->Belt.Array.reduce(1, (acc, curr) => acc * curr);
