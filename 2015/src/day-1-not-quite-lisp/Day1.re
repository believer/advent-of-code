module Level = {
  let make =
    fun
    | "(" => 1
    | ")" => (-1)
    | _ => 0;
};

module PartOne = {
  let make = input => {
    input
    ->Js.String2.split("")
    ->Array.reduce(0, (acc, curr) => {acc + Level.make(curr)});
  };
};

module PartTwo = {
  let make = input => {
    let (_, position) =
      input
      ->Js.String2.split("")
      ->Array.reduceWithIndex((0, None), ((acc, isBasement), curr, i) => {
          switch (acc + Level.make(curr), isBasement) {
          | (level, None) when level < 0 => (level, Some(i + 1))
          | (level, None) => (level, None)
          | (level, Some(_)) => (level, isBasement)
          }
        });

    position;
  };
};
