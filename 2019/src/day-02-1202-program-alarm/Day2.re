module Memory = {
  let update = (mem, pos, value) => mem->Belt.Array.set(pos, value)->ignore;

  let make = input => input->Belt.Array.copy;
};

module Computer = {
  let rec make = (input, ~noun, ~verb, ~offset=0, ()) => {
    let memory = Memory.make(input);
    let memoryUpdate = Memory.update(memory);

    memoryUpdate(1, noun);
    memoryUpdate(2, verb);

    switch (offset <= Belt.Array.length(memory)) {
    | false => memory
    | true =>
      switch (memory->Belt.Array.slice(~len=4, ~offset)) {
      | [|cmd, p1, p2, pos|] =>
        let v1 = memory->Belt.Array.get(p1);
        let v2 = memory->Belt.Array.get(p2);

        switch (cmd, v1, v2) {
        | (1, Some(v1), Some(v2)) => memoryUpdate(pos, v1 + v2)
        | (2, Some(v1), Some(v2)) => memoryUpdate(pos, v1 * v2)
        | _ => ()
        };
      | _ => ()
      };

      make(memory, ~noun, ~verb, ~offset=offset + 4, ());
    };
  };
};

module PartOne = {
  let make = input =>
    Computer.make(input, ~noun=12, ~verb=2, ())->Belt.Array.get(0);
};

module PartTwo = {
  let make = input => {
    let nouns = Belt.Array.range(0, 99);
    let verbs = Belt.Array.range(0, 99);

    let res =
      nouns
      ->Belt.Array.map(noun => {
          verbs->Belt.Array.map(verb =>
            switch (
              Computer.make(input, ~noun, ~verb, ())->Belt.Array.get(0)
            ) {
            | Some(19690720) =>
              Some(noun->Belt.Int.toString ++ verb->Belt.Int.toString)
            | None
            | Some(_) => None
            }
          )
        })
      ->Belt.Array.concatMany
      ->Belt.Array.keep(Belt.Option.isSome);

    switch (res->Belt.Array.get(0)) {
    | Some(v) => v
    | None => None
    };
  };
};
