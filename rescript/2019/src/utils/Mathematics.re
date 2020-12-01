open Tablecloth;

module GreatestCommonDivisor = {
  let rec make = (a, b) =>
    switch (b) {
    | 0. => abs_float(a)
    | b => make(b, mod_float(a, b))
    };

  let fromList = input => input |> List.foldl(~init=0., ~f=make);
};

module LeastCommonMultiple = {
  let make = (m, n) =>
    switch (m, n) {
    | (0., _)
    | (_, 0.) => 0.
    | (m, n) => abs_float(m *. n) /. GreatestCommonDivisor.make(m, n)
    };

  let fromList = input => input |> List.foldl(~init=1., ~f=make);
};
