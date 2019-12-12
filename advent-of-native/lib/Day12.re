open Tablecloth;
open Printf;

module Coordinates = {
  let make = ((_, coords, _)) => coords;
};

module Velocity = {
  let make = (x, y) => {
    switch (x, y) {
    | (x, y) when x === y => 0
    | (x, y) when y > x => 1
    | _ => (-1)
    };
  };
};

module Energy = {
  let make = ((x, y, z)) => abs(x) + abs(y) + abs(z);
};

module PartOne = {
  let rec make = (moons, ~iterations) => {
    switch (iterations) {
    | 0 =>
      moons
      |> List.map(~f=((_, potential, kinetic)) => {
           Energy.make(potential) * Energy.make(kinetic)
         })
      |> List.foldl(~init=0, ~f=(+))
    | _ =>
      let step =
        moons
        |> List.map(~f=((name, (x, y, z), v)) => {
             let (vx, vy, vz) =
               moons
               |> List.filter(~f=((moon, _, _)) => {moon !== name})
               |> List.foldl(
                    ~init=v,
                    ~f=(moon, (ax, ay, az)) => {
                      let (xx, yy, zz) = Coordinates.make(moon);

                      (
                        ax + Velocity.make(x, xx),
                        ay + Velocity.make(y, yy),
                        az + Velocity.make(z, zz),
                      );
                    },
                  );

             (name, (x + vx, y + vy, z + vz), (vx, vy, vz));
           });

      make(step, ~iterations=iterations - 1);
    };
  };
};
