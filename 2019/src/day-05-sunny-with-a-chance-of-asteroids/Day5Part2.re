module Value = Day5Part1.Value;
module Action = Day5Part1.Action;

module OpCode = {
  type t =
    | Addition
    | Multiplication
    | NotImplemented
    | Input
    | Output
    | JumpIfTrue
    | JumpIfFalse
    | LessThan
    | Equals
    | Halt;

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
  let output = ref(0);

  let rec make = (program, ~position=0, ~input, ()) => {
    let pos = ref(position);

    switch (position < Belt.Array.length(program)) {
    | false => output^
    | true =>
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
          program
          ->Belt.Array.set(
              program->Belt.Array.getUnsafe(position + 1),
              input,
            )
          ->ignore;

          stepSize := Steps.make(opcode, ());

        | Output =>
          switch (output^) {
          | 0 => output := v1
          | _ => ()
          };

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

let make = program => Computer.make(program->Belt.Array.copy, ~input=5, ());
