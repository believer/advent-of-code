let polymerUnits = input =>
  (input |> Js.String.split(""))
  ->Belt.Array.reduce([||], (remaining, a) =>
      switch (remaining->Js.Array.pop) {
      | Some(b) =>
        if (a != b && Js.String.toUpperCase(a) == Js.String.toUpperCase(b)) {
          remaining;
        } else {
          remaining |> Js.Array.push(b) |> ignore;
          remaining |> Js.Array.push(a) |> ignore;
          remaining;
        }
      | None =>
        remaining |> Js.Array.push(a) |> ignore;
        remaining;
      }
    )
  ->Belt.Array.length;