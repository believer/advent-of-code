open Tablecloth;

module Fuel = {
  let make = mass => Js.Math.floor(Belt.Int.toFloat(mass) /. 3.) - 2;
};

module AdditionalFuel = {
  let rec make = (fuel, ~extraFuel=0, ()) => {
    switch (Fuel.make(fuel)) {
    | fuel when fuel <= 0 => extraFuel
    | fuel => make(fuel, ~extraFuel=extraFuel + fuel, ())
    };
  };
};

module PartOne = {
  let make = masses =>
    masses->List.foldr(~init=0, ~f=(mass, acc) => acc + Fuel.make(mass));
};

module PartTwo = {
  let make = masses =>
    masses->List.foldr(
      ~init=0,
      ~f=(mass, acc) => {
        let fuel = Fuel.make(mass);

        acc + fuel + AdditionalFuel.make(fuel, ());
      },
    );
};
