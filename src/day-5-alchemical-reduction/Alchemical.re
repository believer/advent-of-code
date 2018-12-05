open Js.Array;

let polymerUnits = input =>
  (input |> Js.String.split(""))
  ->Belt.Array.reduce([||], (remaining, a) =>
      switch (remaining->pop) {
      | Some(b) =>
        if (a != b && Js.String.toUpperCase(a) == Js.String.toUpperCase(b)) {
          remaining;
        } else {
          remaining |> push(b) |> ignore;
          remaining |> push(a) |> ignore;
          remaining;
        }
      | None =>
        remaining |> push(a) |> ignore;
        remaining;
      }
    )
  ->Belt.Array.length;