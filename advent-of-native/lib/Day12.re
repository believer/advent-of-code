open Tablecloth;

module Coordinates = {
  let make = ((coords, _)) => coords;
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

module Step = {
  let make = moons =>
    moons
    |> List.map(~f=(((x, y, z), v)) => {
         let (vx, vy, vz) =
           moons
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

         ((x + vx, y + vy, z + vz), (vx, vy, vz));
       });
};

module PartOne = {
  let rec make = (moons, ~iterations) => {
    switch (iterations) {
    | 0 =>
      moons
      |> List.map(~f=((potential, kinetic)) => {
           Energy.make(potential) * Energy.make(kinetic)
         })
      |> List.foldl(~init=0, ~f=(+))
    | _ => make(Step.make(moons), ~iterations=iterations - 1)
    };
  };
};

module PartTwo = {
  module Hash = {
    let make = values => {
      values
      |> List.foldl(~init="", ~f=((x, v), acc) => {
           acc ++ (x |> string_of_int) ++ (v |> string_of_int)
         });
    };
  };

  module Steps = {
    module Count = {
      let rec make = (values, ~seen, ~steps=1, ()) => {
        let nextValues =
          values
          |> List.map(~f=((x, v)) => {
               let vx =
                 values
                 |> List.foldl(~init=v, ~f=((y, _), acc) =>
                      acc + Velocity.make(x, y)
                    );

               (x + vx, vx);
             });

        let nextHash = Hash.make(nextValues);

        switch (seen |> StrSet.has(~value=nextHash)) {
        | false =>
          let seen = seen |> StrSet.add(~value=nextHash);
          make(nextValues, ~seen, ~steps=steps + 1, ());
        | true => steps
        };
      };
    };

    let make = values => {
      let seen = StrSet.empty;
      let hash = Hash.make(values);
      let start = seen |> StrSet.add(~value=hash);

      Count.make(values, ~seen=start, ());
    };
  };

  let make = moons => {
    let x = moons |> List.map(~f=(((x, _, _), _)) => (x, 0)) |> Steps.make;
    let y = moons |> List.map(~f=(((_, y, _), _)) => (y, 0)) |> Steps.make;
    let z = moons |> List.map(~f=(((_, _, z), _)) => (z, 0)) |> Steps.make;

    Mathematics.LeastCommonMultiple.fromList([x, y, z]);
  };
};
