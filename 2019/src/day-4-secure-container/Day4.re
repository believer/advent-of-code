module NoDecrease = {
  let make = number => {
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
};

module HasDigits = {
  type t =
    | AtLeast(int)
    | Exactly(int);

  let make = (password, ~matcher) => {
    let map = Array.make(10, 0);

    password
    ->Int.toString
    ->Js.String2.split("")
    ->Array.map(int_of_string)
    ->Array.forEach(curr => {
        switch (map->Array.get(curr)) {
        | Some(v) => map->Array.set(curr, v + 1)->ignore
        | None => map->Array.set(curr, 1)->ignore
        }
      });

    switch (matcher) {
    | AtLeast(n) => Js.Math.maxMany_int(map) >= n
    | Exactly(n) => map->Array.keep(v => v === n)->Array.length > 0
    };
  };
};

module Password = {
  let make = (~bounds, ~matcher) => {
    let (lower, upper) = bounds;

    Array.range(lower, upper)
    ->Array.reduce(0, (acc, curr) => {
        switch (NoDecrease.make(curr), HasDigits.make(curr, ~matcher)) {
        | (true, true) => acc + 1
        | _ => acc
        }
      });
  };
};

module PartOne = {
  let make = (lower, upper) =>
    Password.make(~bounds=(lower, upper), ~matcher=AtLeast(2));
};

module PartTwo = {
  let make = (lower, upper) =>
    Password.make(~bounds=(lower, upper), ~matcher=Exactly(2));
};
