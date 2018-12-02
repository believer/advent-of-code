open Belt.Array;

let defaultValue = ("", "", 0.);

let findSimilarity = (input, i, row) =>
  switch (input->get(i + 1)) {
  | Some(next) => (row, next, Levenshtein.similarity(row, next))
  | _ => defaultValue
  };

let findBoxes = input => {
  let (first, second, _) =
    input
    ->Js.Array.sortInPlace
    ->mapWithIndex(findSimilarity(input))
    ->Utils.sortInPlaceWith(Utils.sortFloats)
    ->get(0)
    ->Belt.Option.getWithDefault(defaultValue);

  let allSecond = second |> Js.String.split("");

  Js.String.split("", first)
  ->keepWithIndex((firstChar, i) => allSecond[i] == firstChar)
  ->Utils.join("");
};
