type guardT =
  | FallsAsleep
  | WakeUp
  | UnknownState;

let dateRe = [%re "/\\d{4}-\\d{2}-\\d{2} \\d{2}:(\\d{2})/"];
let guardIdRe = [%re "/\\d+/"];
let firstWordRe = [%re "/^\\w+/"];

let parseDate = row =>
  switch (dateRe->Js.String.match(row)) {
  | Some(date) => (date[0], date[1])
  | None => ("", "")
  };

let guardState = guard =>
  switch (firstWordRe->Js.String.match(guard)) {
  | Some(w) =>
    switch (w[0]) {
    | "falls" => FallsAsleep
    | "wakes" => WakeUp
    | _ => UnknownState
    }
  | _ => UnknownState
  };

let latestGuard = ref("");
let guardId = row =>
  switch (guardIdRe->Js.String.match(row)) {
  | Some(id) =>
    latestGuard := id[0];
    id[0] |> Js.String.replace("#", "");
  | None => latestGuard^
  };

let sortByDate = (((a, _), _), ((b, _), _)) =>
  Utils.dateAsInt(a) - Utils.dateAsInt(b);

let createList = input => {
  let list = Js.Dict.empty();
  let fellAsleep = ref(0);

  input
  ->Belt.Array.map(row => {
      let split = Js.String.split("] ", row);
      (parseDate(split[0]), split[1]);
    })
  ->Utils.sortInPlaceWith(sortByDate)
  ->Belt.Array.map((((_, minute), row)) =>
      (minute, guardState(row), guardId(row))
    )
  ->Belt.Array.forEach(((minute, state, id)) =>
      switch (state) {
      | FallsAsleep =>
        switch (list->Js.Dict.get(id)) {
        | Some(_) => ()
        | None => list->Js.Dict.set(id, (0, [||]))
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
            (
              diff + prevTotal,
              Belt.Array.concat([|newInterval|], intervals),
            ),
          );
        | None => ()
        }
      | _ => ()
      }
    );

  list->Js.Dict.entries;
};

let createInterval = arr =>
  arr->Belt.Array.map(Belt.List.toArray)->Utils.flatten;

let getMostSleepy = guard =>
  guard
  ->Js.Dict.entries
  ->Utils.sortInPlaceWith(((_keyA, iA), (_keyB, iB)) => iB - iA)
  ->Belt.Array.keep(((_key, i)) => i > 0)
  ->Belt.Array.get(0);

let updateDictByOne = (dict, id) =>
  (
    switch (dict->Js.Dict.get(id)) {
    | Some(i) => i + 1
    | None => 1
    }
  )
  |> dict->Js.Dict.set(id);
