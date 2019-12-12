open Tablecloth;

/* I first did the solution using native code with ints and lists. Using the
 * same code with BuckleScript created all sorts of errors. First, wrong answer
 * due to the value being too big for ints. So everything had to be converted to
 * floats. Second, using list made the heap overflow (represented as array of
 * array of array...) so regular arrays had to be used. */

module Coordinates = {
  let make = ((coords, _)) => coords;
};

module Velocity = {
  let make = (x, y) => {
    switch (x, y) {
    | (x, y) when x === y => 0.
    | (x, y) when y > x => 1.
    | _ => (-1.)
    };
  };
};

module Energy = {
  let make = ((x, y, z)) => abs_float(x) +. abs_float(y) +. abs_float(z);
};

module Step = {
  let make = moons =>
    moons
    |> Array.map(~f=(((x, y, z), v)) => {
         let (vx, vy, vz) =
           moons
           |> Array.foldLeft(
                ~initial=v,
                ~f=(moon, (ax, ay, az)) => {
                  let (xx, yy, zz) = Coordinates.make(moon);

                  (
                    ax +. Velocity.make(x, xx),
                    ay +. Velocity.make(y, yy),
                    az +. Velocity.make(z, zz),
                  );
                },
              );

         ((x +. vx, y +. vy, z +. vz), (vx, vy, vz));
       });
};

module PartOne = {
  let rec make = (moons, ~iterations) => {
    switch (iterations) {
    | 0. =>
      moons
      |> Array.map(~f=((potential, kinetic)) => {
           Energy.make(potential) *. Energy.make(kinetic)
         })
      |> Array.foldLeft(~initial=0., ~f=(+.))
    | _ => make(Step.make(moons), ~iterations=iterations -. 1.)
    };
  };
};

module PartTwo = {
  module Hash = {
    let make = values => {
      values
      |> Array.foldLeft(~initial="", ~f=((x, v), acc) => {
           acc ++ (x |> Belt.Float.toString) ++ (v |> Belt.Float.toString)
         });
    };
  };

  module Steps = {
    module Count = {
      let rec make = (values, ~seen, ~steps=1., ()) => {
        let nextValues =
          values
          |> Array.map(~f=((x, v)) => {
               let vx =
                 values
                 |> Array.foldLeft(~initial=v, ~f=((y, _), acc) =>
                      acc +. Velocity.make(x, y)
                    );

               (x +. vx, vx);
             });

        let nextHash = Hash.make(nextValues);

        switch (seen |> StrSet.has(~value=nextHash)) {
        | false =>
          let seen = seen |> StrSet.add(~value=nextHash);
          make(nextValues, ~seen, ~steps=steps +. 1., ());
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
    let x =
      moons |> Array.map(~f=(((x, _, _), _)) => (x, 0.)) |> Steps.make;
    let y =
      moons |> Array.map(~f=(((_, y, _), _)) => (y, 0.)) |> Steps.make;
    let z =
      moons |> Array.map(~f=(((_, _, z), _)) => (z, 0.)) |> Steps.make;

    Mathematics.LeastCommonMultiple.fromList([x, y, z]);
  };
};
