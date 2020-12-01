open HelpersFour;

let findSleepy = input => {
  let sleepyGuard = Js.Dict.empty();

  let laziestGuard =
    input
    ->createList
    ->Utils.sortInPlaceWith(((_, (totalA, _)), (_, (totalB, _))) =>
        totalB - totalA
      )
    ->Belt.Array.get(0);

  let (guardId, (_total, lazyGuardIntervals)) =
    switch (laziestGuard) {
    | Some(lazyGuard) => lazyGuard
    | None => ("0", (0, [||]))
    };

  lazyGuardIntervals
  ->createInterval
  ->Belt.Array.forEach(time =>
      sleepyGuard->updateDictByOne(string_of_int(time))
    );

  switch (sleepyGuard->getMostSleepy) {
  | Some((key, _count)) => int_of_string(key) * int_of_string(guardId)
  | None => 0
  };
};
