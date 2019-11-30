open HelpersFour;

let findFrequentSleeper = input => {
  let sleepies = Js.Dict.empty();

  input
  ->createList
  ->Belt.Array.map(((key, (_total, intervals))) =>
      (key, intervals->createInterval)
    )
  ->Belt.Array.forEach(((key, time)) =>
      time->Belt.Array.forEach(t => {
        let tVal = string_of_int(t);

        switch (sleepies->Js.Dict.get(key)) {
        | Some(inner) => inner->updateDictByOne(tVal)
        | None =>
          let first = Js.Dict.empty();
          first->Js.Dict.set(tVal, 1);
          sleepies->Js.Dict.set(key, first);
        };
      })
    );

  let output =
    sleepies
    ->Js.Dict.entries
    ->Belt.Array.map(((id, minutes)) =>
        switch (minutes->getMostSleepy) {
        | Some((day, count)) => (id, day, count)
        | None => (id, "", 0)
        }
      )
    ->Utils.sortInPlaceWith(((_, _, countA), (_, _, countB)) =>
        countB - countA
      )
    ->Belt.Array.get(0);

  switch (output) {
  | Some((id, time, _count)) => int_of_string(id) * int_of_string(time)
  | None => 0
  };
};
