module Int =
  Belt.Id.MakeComparable({
    type t = int;
    let cmp = (a, b) => Pervasives.compare(a, b);
  });

module Str =
  Belt.Id.MakeComparable({
    type t = string;
    let cmp = (a, b) => Pervasives.compare(a, b);
  });

module IntPair =
  Belt.Id.MakeComparable({
    type t = (int, int);

    let cmp = ((a0, a1), (b0, b1)) =>
      switch (Pervasives.compare(a0, b0)) {
      | 0 => Pervasives.compare(a1, b1)
      | c => c
      };
  });
