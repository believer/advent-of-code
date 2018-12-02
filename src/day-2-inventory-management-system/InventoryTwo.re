let sortValues = ((_, _, a), (_, _, b)) =>
  int_of_float(b *. 100.) - int_of_float(a *. 100.);

let findBoxes = input => {
  open Belt.Array;
  open Js.Array;

  let defaultValue = ("", "", 0.);

  let similarity =
    (input |> sortInPlace)
    ->mapWithIndex((i, row) => {
        let nextRow = input->get(i + 1);

        switch (nextRow) {
        | Some(next) =>
          let distance = Levenshtein.similarity(row, next);
          (row, next, distance);
        | _ => defaultValue
        };
      });

  let sorted = (similarity |> sortInPlaceWith(sortValues))->get(0);

  let (first, second, _) =
    switch (sorted) {
    | Some(value) => value
    | _ => defaultValue
    };

  let allFirst = first |> Js.String.split("");
  let allSecond = second |> Js.String.split("");

  allFirst->keepWithIndex((firstChar, i) => allSecond[i] == firstChar)
  |> joinWith("");
};
