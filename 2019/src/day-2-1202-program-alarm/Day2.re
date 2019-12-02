module Computer = {
  let rec make = (input, ~noun, ~verb, ~offset=0, ()) => {
    let memory = input->Array.copy;

    memory->Array.set(1, noun)->ignore;
    memory->Array.set(2, verb)->ignore;

    switch (Array.length(input) >= offset) {
    | false => memory
    | true =>
      switch (input->Array.slice(~len=4, ~offset)) {
      | [|cmd, p1, p2, pos|] =>
        let v1 = input->Array.get(p1);
        let v2 = input->Array.get(p2);

        let value =
          switch (cmd, v1, v2) {
          | (1, Some(v1), Some(v2)) => Some(v1 + v2)
          | (2, Some(v1), Some(v2)) => Some(v1 * v2)
          | _ => None
          };

        switch (value) {
        | Some(v) => memory->Array.set(pos, v)->ignore
        | None => ()
        };
      | _ => ()
      };

      make(memory, ~noun, ~verb, ~offset=offset + 4, ());
    };
  };
};

module PartOne = {
  let make = input =>
    Computer.make(input, ~noun=12, ~verb=2, ())->Array.get(0);
};

module PartTwo = {
  let make = input => {
    let nouns = Array.range(0, 99);
    let verbs = Array.range(0, 99);

    let res =
      nouns
      ->Array.map(noun => {
          verbs->Array.map(verb => 
            switch (Computer.make(input, ~noun, ~verb, ())->Array.get(0)) {
            | Some(19690720) => Some(noun->Int.toString ++ verb->Int.toString)
            | None
            | Some(_) => None
            }
          )
        })
      ->Array.concatMany
      ->Array.keep(Option.isSome);

    switch (res->Array.get(0)) {
    | Some(v) => v
    | None => None
    };
  };
};
