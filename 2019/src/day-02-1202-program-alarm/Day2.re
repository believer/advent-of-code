module Memory = {
  let update = (mem, pos, value) => mem->Array.set(pos, value)->ignore;

  let make = (input) => input->Array.copy;
}

module Computer = {
  let rec make = (input, ~noun, ~verb, ~offset=0, ()) => {
    let memory = Memory.make(input);
    let memoryUpdate = Memory.update(memory);

    memoryUpdate(1, noun);
    memoryUpdate(2, verb);

    switch (offset <= Array.length(memory)) {
    | false => memory
    | true =>
      switch (memory->Array.slice(~len=4, ~offset)) {
      | [|cmd, p1, p2, pos|] =>
        let v1 = memory->Array.get(p1);
        let v2 = memory->Array.get(p2);

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
