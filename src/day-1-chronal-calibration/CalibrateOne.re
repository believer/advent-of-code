type t = {
  duplicate: option(int),
  seenFrequencies: array(int),
  result: int,
};

let parseNumber = v =>
  Js.String.replaceByRe([%re "/\\+/g"], "", v)->int_of_string;

let deviceFrequency =
    (~input, ~result=0, ~duplicate=None, ~seenFrequencies=[|0|], ()) =>
  Js.String.split(", ", input)
  ->Belt.Array.map(parseNumber)
  ->Belt.Array.reduce(
      {result, duplicate, seenFrequencies},
      (acc, curr) => {
        let result = acc.result + curr;

        switch (
          acc.duplicate,
          Js.Array.includes(result, acc.seenFrequencies),
        ) {
        | (None, true) => {
            result,
            duplicate: Some(result),
            seenFrequencies:
              Belt.Array.concat(acc.seenFrequencies, [|result|]),
          }
        | _ => {
            ...acc,
            result,
            seenFrequencies:
              Belt.Array.concat(acc.seenFrequencies, [|result|]),
          }
        };
      },
    );
