open Belt;

module Rows = {
  let make = input => {
    let rows = Array.length(input);
    Array.range(0, rows);
  };
};

module Columns = {
  let make = input => {
    let cols = input->Array.getUnsafe(0)->Js.String2.length;
    Array.range(0, cols);
  };
};

module PartOne = {
  module GreatestCommonDivisor = {
    let rec make = ((x, y)) => {
      switch (x) {
      | 0 => y
      | x => make((y mod x, x))
      };
    };
  };

  module CanSee = {
    let calculate = (d, g) => {
      switch (g) {
      | 0.0 => 0
      | g => Js.Math.floor(d->Float.fromInt /. g)
      };
    };

    let make = (input, (r, rr), (c, cc), seen) => {
      switch (input->Array.get(rr), (rr !== r, cc !== c)) {
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
            ->Float.fromInt;

          seen := Set.add(seen^, (calculate(dr, g), calculate(dc, g)));
        | _ => ()
        }
      | _ => ()
      };

      seen^;
    };
  };

  let make = input => {
    let ans = ref(0);
    let rows = Rows.make(input);
    let columns = Columns.make(input);

    rows->Array.forEach(r => {
      columns->Array.forEach(c => {
        let seen = ref(Set.make(~id=(module Cmp.IntPair)));

        switch (input->Array.get(r)) {
        | Some(row) =>
          switch (row->Js.String2.get(c)) {
          | "#" =>
            rows->Array.forEach(rr => {
              columns->Array.forEach(cc => {
                seen := CanSee.make(input, (r, rr), (c, cc), seen)
              })
            });

            if (Set.toArray(seen^)->Array.length > ans^) {
              ans := Set.toArray(seen^)->Array.length;
            };
          | _ => ()
          }
        | None => ()
        };
      })
    });

    ans^;
  };
};
