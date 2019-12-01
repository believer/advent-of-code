module Fuel = {
  let make = mass => Js.Math.floor(Int.toFloat(mass) /. 3.) - 2;
};

module PartOne = {
  let make = masses =>
    masses->List.reduce(0, (acc, mass) => acc + Fuel.make(mass));
};

module PartTwo = {
  let rec additionalFuel = (fuel, ~extraFuel=0, ()) => {
    switch (Fuel.make(fuel)) {
    | fuel when fuel <= 0 => extraFuel
    | fuel => additionalFuel(fuel, ~extraFuel=extraFuel + fuel, ())
    };
  };

  let make = masses =>
    masses->List.reduce(
      0,
      (acc, mass) => {
        let fuel = Fuel.make(mass);

        acc + fuel + additionalFuel(fuel, ());
      },
    );
};
