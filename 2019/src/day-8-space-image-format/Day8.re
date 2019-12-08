module IntCmp =
  Belt.Id.MakeComparable({
    type t = int;
    let cmp = (a, b) => Pervasives.compare(a, b);
  });

module PartOne = {
  let make = input => {
    let imageSize = 25 * 6;
    let layers = Js.String2.length(input) / imageSize;

    let out =
      Array.range(0, layers - 1)
      ->Array.map(i => {
          let map = Map.make(~id=(module IntCmp));

          input
          ->Js.String2.slice(~from=i * imageSize, ~to_=imageSize * (i + 1))
          ->Js.String2.split("")
          ->Array.reduce(
              map,
              (acc, curr) => {
                let id = curr->int_of_string;

                switch (acc->Map.get(id)) {
                | Some(v) => acc->Map.set(id, v + 1)
                | None => acc->Map.set(id, 1)
                };
              },
            );
        })
      ->List.fromArray
      ->List.sort((a, b) => {
          let valuesA = Map.valuesToArray(a);
          let valuesB = Map.valuesToArray(b);

          switch (valuesA[0], valuesB[0]) {
          | (Some(a), Some(b)) => a - b
          | _ => (-1)
          };
        })
      ->List.get(0);

    switch (out) {
    | Some(v) =>
      switch (Map.valuesToArray(v)) {
      | [|_, v1, v2|] => v1 * v2
      | _ => 0
      }
    | None => 0
    };
  };
};
