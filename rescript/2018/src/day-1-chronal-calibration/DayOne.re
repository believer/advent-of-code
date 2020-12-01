module One = {
  let make = input => input->Belt.List.reduce(0, (+));
};

module Two = {
  open Belt.List;

  let rec make = (input, ~seenSums=[0], ~total=0, ()) => {
    let values =
      input->reduce(
        (total, seenSums, None),
        (acc, curr) => {
          let (total, prevSums, hasSeen) = acc;

          switch (hasSeen) {
          | Some(_) => acc
          | None =>
            let sum = total + curr;
            let hasSeenSum =
              prevSums->some(i => i === sum) ? Some(sum) : None;

            (sum, [sum, ...prevSums], hasSeenSum);
          };
        },
      );

    switch (values) {
    | (total, seenSums, None) => make(input, ~seenSums, ~total, ())
    | (_, _, Some(sum)) => sum
    };
  };
};
