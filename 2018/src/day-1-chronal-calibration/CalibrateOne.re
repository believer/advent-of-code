open Belt.Array;

type t = {
  duplicate: option(int),
  seenFrequencies: array(int),
  result: int,
};

let deviceFrequency =
    (~input, ~result=0, ~duplicate=None, ~seenFrequencies=[|0|], ()) =>
  Js.String.split(", ", input)
  ->map(Utils.parseNumber)
  ->reduce(
      {result, duplicate, seenFrequencies},
      (acc, curr) => {
        let result = acc.result + curr;
        let hasValue = some(acc.seenFrequencies, v => v == result);
        let seenFrequencies = concat(acc.seenFrequencies, [|result|]);

        switch (acc.duplicate, hasValue) {
        | (None, true) => {result, seenFrequencies, duplicate: Some(result)}
        | _ => {...acc, result, seenFrequencies}
        };
      },
    );
