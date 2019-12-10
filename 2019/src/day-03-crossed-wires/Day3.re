open Tablecloth;

module GetPoints = {
  let make = wire => {
    let x = ref(0);
    let y = ref(0);
    let length = ref(0);

    let set =
      Belt.Set.make(~id=(module Cmp.IntPair))->Belt.Set.add((x^, y^));
    let lengthMap = Belt.Map.make(~id=(module Cmp.IntPair));

    wire->Belt.List.reduce(
      (set, lengthMap),
      ((acc, accLength), op) => {
        let (cmd, steps) = Direction.make(op);

        List.initialize(steps, _ => 0)
        ->List.foldr(
            ~init=(acc, accLength),
            ~f=(_, (acc, accLength)) => {
              x := x^ + Direction.DX.make(cmd);
              y := y^ + Direction.DY.make(cmd);
              length := length^ + 1;

              switch (accLength->Belt.Map.get((x^, y^))) {
              | None => (
                  acc->Belt.Set.add((x^, y^)),
                  accLength->Belt.Map.set((x^, y^), length^),
                )
              | Some(_) => (acc->Belt.Set.add((x^, y^)), accLength)
              };
            },
          );
      },
    );
  };
};

module PartOne = {
  let make = ((wireOne, wireTwo)) => {
    let (first, _) = GetPoints.make(wireOne);
    let (second, _) = GetPoints.make(wireTwo);
    let intersections = Belt.Set.intersect(first, second)->Belt.Set.toArray;

    intersections->Array.foldLeft(~initial=0, ~f=((x, y), acc) =>
      switch (acc, Js.Math.abs_int(x) + Js.Math.abs_int(y)) {
      | (0, value) => value
      | (acc, value) when value < acc => value
      | _ => acc
      }
    );
  };
};

module PartTwo = {
  let mapToSet = map =>
    map->Belt.Map.keysToArray->Belt.Set.fromArray(~id=(module Cmp.IntPair));

  let make = ((wireOne, wireTwo)) => {
    let (_, firstLength) = GetPoints.make(wireOne);
    let (_, secondLength) = GetPoints.make(wireTwo);
    let intersections =
      Belt.Set.intersect(firstLength->mapToSet, secondLength->mapToSet)
      ->Belt.Set.toArray;

    intersections->Array.foldLeft(
      ~initial=0,
      ~f=(k, acc) => {
        let x = firstLength->Belt.Map.get(k);
        let y = secondLength->Belt.Map.get(k);

        switch (acc, x, y) {
        | (0, Some(v1), Some(v2)) => v1 + v2
        | (acc, Some(v1), Some(v2)) when v1 + v2 < acc => v1 + v2
        | _ => acc
        };
      },
    );
  };
};
