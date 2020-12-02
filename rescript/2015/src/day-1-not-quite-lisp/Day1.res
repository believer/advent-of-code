module Level = {
  let make = input =>
    switch input {
    | "(" => 1
    | ")" => -1
    | _ => 0
    }
}

module PartOne = {
  let make = input => {
    input->Js.String2.split("")->Js.Array2.reduce((acc, curr) => acc + Level.make(curr), 0)
  }
}

module PartTwo = {
  let make = input => {
    let (_, position) = input->Js.String2.split("")->Js.Array2.reducei(((
      acc,
      isBasement,
    ), curr, i) => {
      switch (acc + Level.make(curr), isBasement) {
      | (level, None) when level < 0 => (level, Some(i + 1))
      | (level, None) => (level, None)
      | (level, Some(_)) => (level, isBasement)
      }
    }, (0, None))

    position
  }
}
