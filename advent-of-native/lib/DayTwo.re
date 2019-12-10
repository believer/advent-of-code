let copy = Array.copy;
open Tablecloth;

module Memory = {
  let update = (mem, pos, value) => mem |> Array.set(~index=pos, ~value);

  let make = input => input |> copy;
};

module Computer = {
  let rec make = (input, ~noun, ~verb, ~offset=0, ()) => {
    let memory = Memory.make(input);
    let memoryUpdate = Memory.update(memory);

    memoryUpdate(1, noun);
    memoryUpdate(2, verb);

    switch (offset <= Array.length(memory)) {
    | false => memory
    | true =>
      switch (memory |> Array.slice(~to_=offset + 4, ~from=offset)) {
      | [|cmd, p1, p2, pos|] =>
        let v1 = memory |> Array.get(~index=p1);
        let v2 = memory |> Array.get(~index=p2);

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
    Computer.make(input, ~noun=12, ~verb=2, ()) |> Array.get(~index=0);
};

module PartTwo = {
  let make = input => {
    let nouns = Array.range(99);
    let verbs = Array.range(99);

    let res =
      nouns
      |> Array.map(~f=noun => {
           verbs
           |> Array.map(~f=verb =>
                switch (
                  Computer.make(input, ~noun, ~verb, ())
                  |> Array.get(~index=0)
                ) {
                | Some(19690720) =>
                  Some((noun |> string_of_int) ++ (verb |> string_of_int))
                | None
                | Some(_) => None
                }
              )
         })
      |> Array.concatenate
      |> Array.filter(~f=Option.isSome);

    switch (res |> Array.get(~index=0)) {
    | Some(v) => v
    | None => None
    };
  };
};
