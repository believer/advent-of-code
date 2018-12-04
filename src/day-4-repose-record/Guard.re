type guardT =
  | StartShift
  | FallsAsleep
  | WakeUp
  | UnknownState;

let timeAsInt = date =>
  int_of_float(Js.Date.fromString(date) |> Js.Date.getTime);

let parseDate = row =>
  switch (
    Js.String.match([%re "/\\d{4}-(\\d{2})-(\\d{2}) \\d{2}:(\\d{2})/"], row)
  ) {
  | Some(date) => (date[0], date[1], date[2], date[3])
  | None => ("", "", "", "")
  };

let latestGuard = ref("");
let parseGuard = row =>
  switch (Js.String.match([%re "/^\\w+/i"], row)) {
  | Some(guard) =>
    let state =
      switch (guard[0] |> Js.String.toLowerCase) {
      | "guard" => StartShift
      | "falls" => FallsAsleep
      | "wakes" => WakeUp
      | _ => UnknownState
      };

    let id =
      switch (Js.String.match([%re "/\\d+/"], row)) {
      | Some(id) =>
        latestGuard := id[0];
        id[0] |> Js.String.replace("#", "");
      | None => latestGuard^
      };

    (state, id);
  | None => (UnknownState, "")
  };

let findSleepy = input => {
  let list = Js.Dict.empty();
  let fellAsleep = ref(0);

  input
  ->Belt.Array.map(row => {
      let split = Js.String.split("] ", row);

      (parseDate(split[0]), split[1]);
    })
  ->Utils.sortInPlaceWith((((a, _, _, _), _), ((b, _, _, _), _)) =>
      timeAsInt(a) - timeAsInt(b)
    )
  ->Belt.Array.map((((_, month, date, minute), row)) => {
      let (state, id) = parseGuard(row);
      (month, date, minute, state, id);
    })
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

  let laziest =
    list
    ->Js.Dict.entries
    ->Utils.sortInPlaceWith(((_, (totalA, _)), (_, (totalB, _))) =>
        totalB - totalA
      )
    ->Belt.Array.get(0);

  let (id, (_total, list)) =
    switch (laziest) {
    | Some(lazyGuard) => lazyGuard
    | None => ("0", (0, []))
    };

  let sleepies = Js.Dict.empty();

  list
  ->Belt.List.map(item => item->Belt.List.toArray)
  ->Belt.List.toArray
  ->Utils.flatten
  ->Belt.Array.forEach(time =>
      (
        switch (sleepies->Js.Dict.get(string_of_int(time))) {
        | Some(t) => t + 1
        | None => 0
        }
      )
      |> sleepies->Js.Dict.set(string_of_int(time))
    );

  let mostSleepyTime =
    sleepies
    ->Js.Dict.entries
    ->Utils.sortInPlaceWith(((_keyA, iA), (_keyB, iB)) => iB - iA)
    ->Belt.Array.keep(((_key, i)) => i > 0)
    ->Belt.Array.get(0);

  let sleepyMinute =
    switch (mostSleepyTime) {
    | Some((key, _count)) => int_of_string(key)
    | None => 0
    };

  int_of_string(id) * sleepyMinute;
};
