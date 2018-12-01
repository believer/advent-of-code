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
    let first =
      CalibrateOne.deviceFrequency(
        ~input,
        ~result=lastResult,
        ~duplicate=lastDuplicate,
        ~seenFrequencies=lastSeenFrequencies,
        (),
      );

    let finalValue =
      first.seenFrequencies
      ->Belt.Array.get(Belt.Array.length(first.seenFrequencies) - 1);

    findDuplicateFrequency(
      ~input,
      ~lastResult=Belt.Option.getWithDefault(finalValue, 0),
      ~lastDuplicate=first.duplicate,
      ~lastSeenFrequencies=first.seenFrequencies,
      (),
    );
  };
