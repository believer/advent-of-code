open Tablecloth;

module Rows = {
  let make = input => input->Array.length->Array.range;
};

module Columns = {
  let make = input =>
    switch (input->Array.get(~index=0)) {
    | Some(len) => len->String.length->Array.range
    | None => Array.empty
    };
};

module Grid = {
  let make = input =>
    Rows.make(input)
    ->Array.map(~f=r => {Columns.make(input)->Array.map(~f=c => {(r, c)})})
    ->Array.concatenate;
};

module PartOne = {
  module GreatestCommonDivisor = {
    let rec make = ((x, y)) =>
      switch (x) {
      | 0 => y
      | x => make((y mod x, x))
      };
  };

  module CanSee = {
    let calculate = (d, g) =>
      switch (g) {
      | 0.0 => 0
      | g => Js.Math.floor(d->Belt.Float.fromInt /. g)
      };

    let make = (input, (r, rr), (c, cc), seen) => {
      switch (input->Array.get(~index=rr), (rr !== r, cc !== c)) {
      | (Some(row), (true, true))
      | (Some(row), (false, true))
      | (Some(row), (true, false)) =>
        switch (row->Js.String2.get(cc)) {
        | "#" =>
          let dr = rr - r;
          let dc = cc - c;
          let g =
            GreatestCommonDivisor.make((dr, dc))
            ->Js.Math.abs_int
            ->Belt.Float.fromInt;

          seen :=
            IntPairSet.add(
              seen^,
              ~value=(calculate(dr, g), calculate(dc, g)),
            );
        | _ => ()
        }
      | _ => ()
      };

      seen^;
    };
  };

  let make = input => {
    let ans = ref(0);

    Grid.make(input)
    ->Array.forEach(~f=((r, c)) => {
        let seen = ref(IntPairSet.make());

        switch (input->Array.get(~index=r)) {
        | Some(row) =>
          switch (row->Js.String2.get(c)) {
          | "#" =>
            Grid.make(input)
            ->Array.forEach(~f=((rr, cc)) => {
                seen := CanSee.make(input, (r, rr), (c, cc), seen)
              });

            if (IntPairSet.length(seen^) > ans^) {
              ans := IntPairSet.length(seen^);
            };
          | _ => ()
          }
        | None => ()
        };
      });

    ans^;
  };
};
