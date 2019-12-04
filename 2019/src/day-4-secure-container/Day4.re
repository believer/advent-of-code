module NoDecrease = {
  let make = values => {
    values
    ->Array.mapWithIndex((i, curr) => {
        switch (curr, values->Array.get(i - 1)) {
        | (x, Some(y)) => x >= y
        | (_, None) => true
        }
      })
    ->Array.every(v => v === true);
  };
};

module HasDigits = {
  type t =
    | AtLeast(int)
    | Exactly(int);

  let make = (password, ~matcher) => {
    let map = Array.make(10, 0);

    password->Array.forEach(curr => {
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

module ElfPassword = {
  let make = number =>
    number->Int.toString->Js.String2.split("")->Array.map(int_of_string);
};

module FindPassword = {
  let make = (~bounds, ~matcher) => {
    let (lower, upper) = bounds;

    Array.range(lower, upper)
    ->Array.reduce(
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
