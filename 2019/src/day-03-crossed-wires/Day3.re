module GetPoints = {
  let make = wire => {
    let x = ref(0);
    let y = ref(0);
    let length = ref(0);

    let set = Set.make(~id=(module Cmp.IntPair))->Set.add((x^, y^));
    let lengthMap = Map.make(~id=(module Cmp.IntPair));

    wire->List.reduce(
      (set, lengthMap),
      ((acc, accLength), op) => {
        let (cmd, steps) = Direction.make(op);

        List.make(steps, 0)
        ->List.reduce(
            (acc, accLength),
            ((acc, accLength), _) => {
              x := x^ + Direction.DX.make(cmd);
              y := y^ + Direction.DY.make(cmd);
              length := length^ + 1;

              switch (accLength->Map.get((x^, y^))) {
              | None => (
                  acc->Set.add((x^, y^)),
                  accLength->Map.set((x^, y^), length^),
                )
              | Some(_) => (acc->Set.add((x^, y^)), accLength)
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
    let intersections = Set.intersect(first, second)->Set.toArray;

    intersections->Array.reduce(0, (acc, (x, y)) =>
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
    map->Map.keysToArray->Set.fromArray(~id=(module Cmp.IntPair));

  let make = ((wireOne, wireTwo)) => {
    let (_, firstLength) = GetPoints.make(wireOne);
    let (_, secondLength) = GetPoints.make(wireTwo);
    let intersections =
      Set.intersect(firstLength->mapToSet, secondLength->mapToSet)
      ->Set.toArray;

    intersections->Array.reduce(
      0,
      (acc, k) => {
        let x = firstLength->Map.get(k);
        let y = secondLength->Map.get(k);

        switch (acc, x, y) {
        | (0, Some(v1), Some(v2)) => v1 + v2
        | (acc, Some(v1), Some(v2)) when v1 + v2 < acc => v1 + v2
        | _ => acc
        };
      },
    );
  };
};
