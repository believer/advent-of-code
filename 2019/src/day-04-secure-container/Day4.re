open Tablecloth;

module NoDecrease = {
  let make = values =>
    values
    ->Array.mapi(~f=(i, curr) =>
        switch (curr, values->Array.get(~index=i - 1)) {
        | (x, Some(y)) => x >= y
        | (_, None) => true
        }
      )
    ->Array.all(~f=v => v === true);
};

module HasDigits = {
  type t =
    | AtLeast(int)
    | Exactly(int);

  let make = (password, ~matcher) => {
    let map = Array.initialize(~length=10, ~f=_ => 0);

    password->Array.forEach(~f=curr =>
      switch (map->Array.get(~index=curr)) {
      | Some(v) => map->Array.set(~index=curr, ~value=v + 1)
      | None => map->Array.set(~index=curr, ~value=1)
      }
    );

    switch (matcher) {
    | AtLeast(n) => Js.Math.maxMany_int(map) >= n
    | Exactly(n) => map->Array.any(~f=v => v === n)
    };
  };
};

module ElfPassword = {
  let make = number =>
    number
    ->Belt.Int.toString
    ->String.split(~on="")
    ->List.map(~f=int_of_string)
    ->Array.fromList;
};

module FindPassword = {
  let make = (~bounds, ~matcher) => {
    let (lower, upper) = bounds;

    Array.range(~from=lower, upper + 1)
    ->Array.foldLeft(
        ~initial=0,
        ~f=(curr, acc) => {
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
