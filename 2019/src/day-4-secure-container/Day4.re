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

module PartTwo = {
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

    let test =
      values->Array.reduceWithIndex(false, (acc, curr, i) => {
        switch (
          acc,
          curr,
          values->Array.get(i - 1),
          values->Array.get(i - 2),
          values->Array.get(i + 1),
        ) {
        | (false, a, Some(b), None, None) => a === b
        | (false, a, Some(b), None, Some(d)) => a === b && a !== d
        | (false, a, Some(b), Some(c), None) => a === b && a !== c
        | (false, x, Some(y), Some(z), Some(zz)) =>
          x === y && z !== x && zz !== x
        | (true, _, _, _, _) => true
        | _ => false
        }
      });
    test;
    /*[>Js.log(test);<]*/
    /*let out =*/
    /*values->Belt.Array.reduceWithIndex(false, (acc, curr, i) => {*/
    /*switch (acc, curr, values->Belt.Array.get(i - 1)) {*/
    /*| (false, x, Some(y)) => x === y*/
    /*| (false, _, None) => false*/
    /*| (true, _, _) => true*/
    /*}*/
    /*});*/
    /*out;*/
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
