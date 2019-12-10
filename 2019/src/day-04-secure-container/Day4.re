module NoDecrease = {
  let make = values =>
    values
    ->Belt.Array.mapWithIndex((i, curr) =>
        switch (curr, values->Belt.Array.get(i - 1)) {
        | (x, Some(y)) => x >= y
        | (_, None) => true
        }
      )
    ->Belt.Array.every(v => v === true);
};

module HasDigits = {
  type t =
    | AtLeast(int)
    | Exactly(int);

  let make = (password, ~matcher) => {
    let map = Belt.Array.make(10, 0);

    password->Belt.Array.forEach(curr =>
      switch (map->Belt.Array.get(curr)) {
      | Some(v) => map->Belt.Array.set(curr, v + 1)->ignore
      | None => map->Belt.Array.set(curr, 1)->ignore
      }
    );

    switch (matcher) {
    | AtLeast(n) => Js.Math.maxMany_int(map) >= n
    | Exactly(n) => map->Belt.Array.some(v => v === n)
    };
  };
};

module ElfPassword = {
  let make = number =>
    number
    ->Belt.Int.toString
    ->Js.String2.split("")
    ->Belt.Array.map(int_of_string);
};

module FindPassword = {
  let make = (~bounds, ~matcher) => {
    let (lower, upper) = bounds;

    Belt.Array.range(lower, upper)
    ->Belt.Array.reduce(
        0,
        (acc, curr) => {
          let pass = ElfPassword.make(curr);

          switch (NoDecrease.make(pass), HasDigits.make(pass, ~matcher)) {
          | (true, true) => acc + 1
          | _ => acc
          };
        },
      );
  };
};

module PartOne = {
  let make = (lower, upper) =>
    FindPassword.make(~bounds=(lower, upper), ~matcher=AtLeast(2));
};

module PartTwo = {
  let make = (lower, upper) =>
    FindPassword.make(~bounds=(lower, upper), ~matcher=Exactly(2));
};
