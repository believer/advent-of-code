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
    masses->Belt.List.reduce(0, (acc, mass) => acc + Fuel.make(mass));
};

module PartTwo = {
  let make = masses =>
    masses->Belt.List.reduce(
      0,
      (acc, mass) => {
        let fuel = Fuel.make(mass);

        acc + fuel + AdditionalFuel.make(fuel, ());
      },
    );
};
