open Tablecloth;

module Orbit = {
  let make = value =>
    switch (value |> String.split(~on=")")) {
    | [a, b] => (a, b)
    | _ => ("", "")
    };
};

module PartOne = {
  module WalkTree = {
    let rec make = (key, map) =>
      switch (map |> StrDict.get(~key)) {
      | Some(deps) =>
        deps |> List.foldl(~init=0, ~f=(c, acc) => {acc + make(c, map) + 1})
      | None => 0
      };
  };

  let make = input => {
    let map = StrDict.empty;

    let out =
      input->Array.foldLeft(
        ~initial=map,
        ~f=(curr, acc) => {
          let (key, target) = Orbit.make(curr);

          switch (acc->StrDict.get(~key)) {
          | Some(v) => acc->StrDict.insert(~key, ~value=[target, ...v])
          | None => acc->StrDict.insert(~key, ~value=[target])
          };
        },
      );

    out
    ->StrDict.toList
    ->List.foldl(~init=0, ~f=((id, _), acc) => {
        acc + WalkTree.make(id, out)
      });
  };
};

module PartTwo = {
  module FindPath = {
    let rec make = (planet: string, orbits, ~previous=[], ()) => {
      switch (orbits |> StrDict.get(~key=planet)) {
      | Some(orbit) =>
        make(orbit, orbits, ~previous=[planet, ...previous], ())

      | None => previous
      };
    };
  };

  module FindDistance = {
    let rec make = (paths, ~steps=0, ~index=0, ()) => {
      let (you, santa) = paths;

      switch (steps) {
      | x when x !== 0 => steps
      | _ =>
        switch (you |> List.getAt(~index), santa |> List.getAt(~index)) {
        | (Some(x), Some(y)) when x === y =>
          make(paths, ~steps, ~index=index + 1, ())
        | _ =>
          // -2 because we want them to orbit the same planet
          let steps =
            List.length(santa) - index + (List.length(you) - index) - 2;

          make(paths, ~steps, ~index, ());
        }
      };
    };
  };

  let make = input => {
    let map = StrDict.empty;

    let orbits =
      input
      |> List.foldl(
           ~init=map,
           ~f=(orbit, acc) => {
             let (value, key) = Orbit.make(orbit);
             acc |> StrDict.insert(~key, ~value);
           },
         );

    let pathToYou = FindPath.make("YOU", orbits, ());
    let pathToSanta = FindPath.make("SAN", orbits, ());

    FindDistance.make((pathToYou, pathToSanta), ());
  };
};
