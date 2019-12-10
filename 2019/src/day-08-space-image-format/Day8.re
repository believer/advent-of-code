open Tablecloth;

module Image = {
  let height = 6;
  let pixels = 25;

  let size = pixels * height;

  let make = (input, i) => {
    input
    ->String.slice(~from=i * size, ~to_=size * (i + 1))
    ->String.split(~on="");
  };
};

module Layers = {
  let make = input =>
    Array.range(String.length(input) / Image.size)->Array.toList;
};

module PartOne = {
  let make = input => {
    let out =
      Layers.make(input)
      ->List.map(~f=i => {
          let map = Belt.Map.make(~id=(module Cmp.Int));

          Image.make(input, i)
          ->List.foldl(
              ~init=map,
              ~f=(curr, acc) => {
                let id = curr->int_of_string;

                switch (acc->Belt.Map.get(id)) {
                | Some(v) => acc->Belt.Map.set(id, v + 1)
                | None => acc->Belt.Map.set(id, 1)
                };
              },
            );
        })
      ->Belt.List.sort((a, b) => {
          let valuesA = Belt.Map.valuesToArray(a);
          let valuesB = Belt.Map.valuesToArray(b);

          switch (
            valuesA->Array.get(~index=0),
            valuesB->Array.get(~index=0),
          ) {
          | (Some(a), Some(b)) => a - b
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
      ->List.foldl(
          ~init=Array.empty->Array.toList,
          ~f=(i, acc) => {
            let values = Image.make(input, i);

            switch (i) {
            | 0 => values
            | _ =>
              values->List.mapi(~f=(index, curr) => {
                switch (acc->List.getAt(~index), curr) {
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
      ->List.map(~f=i => {
          switch (i) {
          | "0" => "."
          | x => x
          }
        });

    Array.range(Image.height + 1)
    ->Array.map(~f=i => {
        Array.slice(
          ~from=i * Image.pixels,
          ~to_=(i + 1) * Image.pixels,
          out->Array.fromList,
        )
        ->Array.toList
        ->String.join(~sep="")
      });
  };
};
