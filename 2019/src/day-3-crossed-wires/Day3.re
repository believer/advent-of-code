module PairComparator =
  Id.MakeComparable({
    type t = (int, int);

    let cmp = ((a0, a1), (b0, b1)) =>
      switch (Pervasives.compare(a0, b0)) {
      | 0 => Pervasives.compare(a1, b1)
      | c => c
      };
  });

module GetPoints = {
  let make = wire => {
    let x = ref(0);
    let y = ref(0);
    let length = ref(0);

    let set = Set.make(~id=(module PairComparator))->Set.add((x^, y^));

    wire->List.reduce(
      set,
      (acc, op) => {
        let (cmd, steps) = Direction.make(op);

        List.make(steps, 0)
        ->List.reduce(
            acc,
            (acc, _) => {
              x := x^ + Direction.DX.make(cmd);
              y := y^ + Direction.DY.make(cmd);

              acc->Set.add((x^, y^));
            },
          );
      },
    );
  };
};

module PartOne = {
  let make = ((wireOne, wireTwo)) => {
    let first = GetPoints.make(wireOne);
    let second = GetPoints.make(wireTwo);
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
  let make = ((wireOne, wireTwo)) => {
    let first = GetPoints.make(wireOne);
    let second = GetPoints.make(wireTwo);
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
