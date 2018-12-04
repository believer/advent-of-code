type guardT =
  | StartShift
  | FallsAsleep
  | WakeUp
  | UnknownState;

let findFrequentSleeper = input => {
  let list = Js.Dict.empty();
  let fellAsleep = ref(0);

  input
  ->GuardOne.createList
  ->Belt.Array.forEach(((_month, _date, minute, state, id)) =>
      switch (state) {
      | FallsAsleep =>
        switch (list->Js.Dict.get(id)) {
        | Some(_) => ()
        | None => list->Js.Dict.set(id, (0, []))
        };

        fellAsleep := int_of_string(minute);
      | WakeUp =>
        switch (list->Js.Dict.get(id)) {
        | Some((prevTotal, intervals)) =>
          let now = int_of_string(minute);
          let diff = now - fellAsleep^;
          let newInterval = Utils.range(fellAsleep^, now);

          list->Js.Dict.set(
            id,
            (diff + prevTotal, [newInterval, ...intervals]),
          );
        | None => ()
        }
      | _ => ()
      }
    );

  let sleepies = Js.Dict.empty();

  list
  ->Js.Dict.entries
  ->Belt.Array.map(((key, (_total, intervals))) =>
      (
        key,
        intervals
        ->Belt.List.toArray
        ->Belt.Array.map(inner => inner->Belt.List.toArray)
        ->Utils.flatten,
      )
    )
  ->Belt.Array.forEach(((key, time)) =>
      time->Belt.Array.forEach(t => {
        let tVal = string_of_int(t);

        switch (sleepies->Js.Dict.get(key)) {
        | Some(inner) =>
          (
            switch (inner->Js.Dict.get(tVal)) {
            | Some(i) => i + 1
            | None => 1
            }
          )
          |> inner->Js.Dict.set(tVal)
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
    ->Belt.Array.map(((id, minutes)) => {
        let mostTime =
          minutes
          ->Js.Dict.entries
          ->Utils.sortInPlaceWith(((_keyA, iA), (_keyB, iB)) => iB - iA)
          ->Belt.Array.keep(((_key, i)) => i > 0)
          ->Belt.Array.get(0);

        switch (mostTime) {
        | Some((day, count)) => (id, day, count)
        | None => (id, "", 0)
        };
      })
    ->Utils.sortInPlaceWith(((_, _, countA), (_, _, countB)) =>
        countB - countA
      )
    ->Belt.Array.get(0);

  switch (output) {
  | Some((id, time, _count)) => int_of_string(id) * int_of_string(time)
  | None => 0
  };
};
