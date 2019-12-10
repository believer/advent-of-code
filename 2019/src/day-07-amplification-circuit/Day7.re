module Value = Day5Part1.Value;
module Action = Day5Part1.Action;

module OpCode = {
  type t =
    | Addition
    | Multiplication
    | Input
    | Output
    | JumpIfTrue
    | JumpIfFalse
    | LessThan
    | Equals
    | Halt
    | NotImplemented;

  let make = intcode => {
    let code =
      switch (intcode mod 100) {
      | 1 => Addition
      | 2 => Multiplication
      | 3 => Input
      | 4 => Output
      | 5 => JumpIfTrue
      | 6 => JumpIfFalse
      | 7 => LessThan
      | 8 => Equals
      | 99 => Halt
      | _ => NotImplemented
      };

    let p1 = Day5Part1.Mode.make(intcode / 100 mod 10);
    let p2 = Day5Part1.Mode.make(intcode / 1000);

    (code, p1, p2);
  };
};

module Steps = {
  let make = (opcode, ~doJump=false, ()) =>
    switch (opcode) {
    | OpCode.Addition
    | Multiplication
    | LessThan
    | Equals => 4
    | Output
    | Input => 2
    | JumpIfTrue => doJump ? 1 : 3
    | JumpIfFalse => doJump ? 1 : 3
    | Halt
    | NotImplemented => 0
    };
};

module Computer = {
  let stepSize = ref(4);
  let output = ref(`run(0));
  let isFirst = ref(true);
  let break = ref(false);

  let rec make = (program, ~position=0, ~input, ()) => {
    let (inputValue, prevValue) = input;
    let pos = ref(position);

    switch (break^, position < Array.length(program)) {
    | (_, false)
    | (true, _) =>
      let out = output^;

      isFirst := true;
      output := `run(0);
      stepSize := 4;
      break := false;

      out;
    | (_, true) =>
      switch (program->Belt.Array.slice(~len=stepSize^, ~offset=position)) {
      | arr =>
        let (opcode, mode1, mode2) =
          OpCode.make(arr->Belt.Array.getUnsafe(0));
        let (v1, v2, updateProgram) =
          Action.make(program, ~position, ~modes=(mode1, mode2));

        switch (opcode) {
        | Addition =>
          updateProgram(v1 + v2);
          stepSize := Steps.make(opcode, ());

        | Multiplication =>
          updateProgram(v1 * v2);
          stepSize := Steps.make(opcode, ());

        | LessThan =>
          updateProgram(v1 < v2 ? 1 : 0);
          stepSize := Steps.make(opcode, ());

        | Equals =>
          updateProgram(v1 === v2 ? 1 : 0);
          stepSize := Steps.make(opcode, ());

        | Input =>
          let v = isFirst^ ? inputValue : prevValue;

          program
          ->Belt.Array.set(program->Belt.Array.getUnsafe(position + 1), v)
          ->ignore;

          isFirst := false;
          stepSize := Steps.make(opcode, ());

        | Output =>
          switch (output^) {
          | `run(0) => output := `halt(v1)
          | _ => ()
          };

          break := true;

          stepSize := Steps.make(opcode, ());

        | JumpIfTrue =>
          let doJump = v1 !== 0;

          if (doJump) {
            pos := v2;
          };

          stepSize := Steps.make(opcode, ~doJump, ());

        | JumpIfFalse =>
          let doJump = v1 === 0;

          if (doJump) {
            pos := v2;
          };

          stepSize := Steps.make(opcode, ~doJump, ());

        | NotImplemented
        | Halt => ()
        };
      };

      let position =
        switch (stepSize^) {
        | 1 => pos^
        | _ => pos^ + stepSize^
        };

      make(program, ~position, ~input, ());
    };
  };
};

module PartOne = {
  let process = (program, settings) => {
    let input = ref(0);

    settings->Belt.Array.forEach(setting => {
      input :=
        (
          switch (
            Computer.make(
              program->Belt.Array.copy,
              ~input=(setting, input^),
              (),
            )
          ) {
          | `halt(value) => value
          | _ => 0
          }
        )
    });

    input^;
  };

  let make = program => {
    let permutations = Heaps.make([|0, 1, 2, 3, 4|]);

    let t =
      permutations
      ->Belt.Array.map(process(program))
      ->Belt.List.fromArray
      ->Belt.List.sort((a, b) => b - a)
      ->Belt.List.get(0);

    switch (t) {
    | Some(v) => v
    | None => 0
    };
  };
};

module PartTwo = {
  let getValue =
    fun
    | `halt(v) => v
    | `run(v) => v;

  let process = (program, settings) => {
    let input = ref(0);
    let break = ref(false);

    while (! break^) {
      let one =
        Computer.make(
          program->Belt.Array.copy,
          ~input=(settings->Belt.Array.getUnsafe(0), input^),
          (),
        );
      let two =
        Computer.make(
          program->Belt.Array.copy,
          ~input=(settings->Belt.Array.getUnsafe(1), one->getValue),
          (),
        );
      let three =
        Computer.make(
          program->Belt.Array.copy,
          ~input=(settings->Belt.Array.getUnsafe(2), two->getValue),
          (),
        );
      let four =
        Computer.make(
          program->Belt.Array.copy,
          ~input=(settings->Belt.Array.getUnsafe(3), three->getValue),
          (),
        );
      let out =
        Computer.make(
          program->Belt.Array.copy,
          ~input=(settings->Belt.Array.getUnsafe(4), four->getValue),
          (),
        );

      input := out->getValue;

      switch (out) {
      | `halt(_) => break := true
      | _ => ()
      };
    };

    input^;
  };

  let make = program => {
    let permutations = Heaps.make([|0, 1, 2, 3, 4|]);

    let t =
      permutations
      ->Belt.Array.map(process(program))
      ->Belt.List.fromArray
      ->Belt.List.sort((a, b) => b - a)
      ->Belt.List.get(0);

    switch (t) {
    | Some(v) => v
    | None => 0
    };
  };
};
