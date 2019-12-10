module Image = {
  let height = 6;
  let pixels = 25;

  let size = pixels * height;

  let make = (input, i) => {
    input
    ->Js.String2.slice(~from=i * size, ~to_=size * (i + 1))
    ->Js.String2.split("");
  };
};

module Layers = {
  let make = input => {
    let layers = Js.String2.length(input) / Image.size;

    Belt.Array.range(0, layers - 1);
  };
};

module PartOne = {
  let make = input => {
    let out =
      Layers.make(input)
      ->Belt.Array.map(i => {
          let map = Belt.Map.make(~id=(module Cmp.Int));

          Image.make(input, i)
          ->Belt.Array.reduce(
              map,
              (acc, curr) => {
                let id = curr->int_of_string;

                switch (acc->Belt.Map.get(id)) {
                | Some(v) => acc->Belt.Map.set(id, v + 1)
                | None => acc->Belt.Map.set(id, 1)
                };
              },
            );
        })
      ->Belt.List.fromArray
      ->Belt.List.sort((a, b) => {
          let valuesA = Belt.Map.valuesToArray(a);
          let valuesB = Belt.Map.valuesToArray(b);

          switch (valuesA[0], valuesB[0]) {
          | (a, b) => a - b
          | _ => (-1)
          };
        })
      ->Belt.List.get(0);

    switch (out) {
    | Some(v) =>
      switch (Belt.Map.valuesToArray(v)) {
      | [|_, v1, v2|] => v1 * v2
      | _ => 0
      }
    | None => 0
    };
  };
};

module PartTwo = {
  let make = input => {
    let out =
      Layers.make(input)
      ->Belt.Array.reduce(
          [||],
          (acc, i) => {
            let values = Image.make(input, i);

            switch (i) {
            | 0 => values
            | _ =>
              values->Belt.Array.mapWithIndex((j, curr) => {
                switch (acc->Belt.Array.get(j), curr) {
                | (Some("2"), "0") => "0"
                | (Some("2"), "1") => "1"
                | (Some("2"), "2") => "2"
                | (Some("0"), _) => "0"
                | (Some("1"), _) => "1"
                | (_, curr) => curr
                }
              })
            };
          },
        )
      ->Belt.Array.map(i => {
          switch (i) {
          | "0" => "."
          | x => x
          }
        });

    Belt.Array.range(0, Image.height)
    ->Belt.Array.map(i => {
        Belt.Array.slice(out, ~offset=i * Image.pixels, ~len=Image.pixels)
        |> Js.Array.joinWith("")
      });
  };
};
