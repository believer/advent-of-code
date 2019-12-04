module PartOne = {
  let isBigger = number => {
    let values = number->Belt.Int.toString->Js.String2.split("");

    let output =
      values->Belt.Array.mapWithIndex((i, curr) => {
        switch (curr, values->Belt.Array.get(i - 1)) {
        | (x, Some(y)) => x >= y
        | (_, None) => true
        }
      });

    switch (output) {
    | [|true, true, true, true, true, true|] => true
    | _ => false
    };
  };

  let nextIsTheSame = number => {
    let values = number->Belt.Int.toString->Js.String2.split("");

    let out =
      values->Belt.Array.reduceWithIndex(false, (acc, curr, i) => {
        switch (acc, curr, values->Belt.Array.get(i - 1)) {
        | (false, x, Some(y)) => x === y
        | (false, _, None) => false
        | (true, _, _) => true
        }
      });

    out;
  };

  let make = (lower, upper) => {
    Array.range(lower, upper)
    ->Array.reduce(
        0,
        (acc, curr) => {
          let isBigger = isBigger(curr);
          let nextIsTheSame = nextIsTheSame(curr);

          switch (isBigger, nextIsTheSame) {
          | (true, true) => acc + 1
          | _ => acc
          };
        },
      );
  };
};
