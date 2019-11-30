open Belt.Array;

let rec findDuplicateFrequency =
        (
          ~input,
          ~lastResult=0,
          ~lastDuplicate=None,
          ~lastSeenFrequencies=[|0|],
          (),
        ) =>
  switch (lastDuplicate) {
  | Some(dupe) => dupe
  | None =>
    let {duplicate, seenFrequencies}: CalibrateOne.t =
      CalibrateOne.deviceFrequency(
        ~input,
        ~result=lastResult,
        ~duplicate=lastDuplicate,
        ~seenFrequencies=lastSeenFrequencies,
        (),
      );

    let finalValue =
      seenFrequencies
      ->get(length(seenFrequencies) - 1)
      ->Belt.Option.getWithDefault(0);

    findDuplicateFrequency(
      ~input,
      ~lastResult=finalValue,
      ~lastDuplicate=duplicate,
      ~lastSeenFrequencies=seenFrequencies,
      (),
    );
  };
