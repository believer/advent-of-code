let flatten = input =>
  input->Belt.Array.reduce([||], (acc, curr) =>
    Belt.Array.concat(acc, curr)
  );

let parseNumber = v =>
  Js.String.replaceByRe([%re "/\\+/g"], "", v)->int_of_string;

let removeDuplicates = row =>
  row->Belt.Array.reduce([||], (acc, curr) =>
    Js.Array.includes(curr, acc) ? acc : Belt.Array.concat(acc, [|curr|])
  );

let sortFloats = ((_, _, a), (_, _, b)) =>
  int_of_float(b *. 100.) - int_of_float(a *. 100.);

let join = (arr, value) => Js.Array.joinWith(value, arr);

let sortInPlaceWith = (arr, value) => Js.Array.sortInPlaceWith(value, arr);
