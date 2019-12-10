module Mode = {
  type t =
    | Immediate
    | Position;

  let make =
    fun
    | 0 => Position
    | _ => Immediate;
};

module OpCode = {
  type t =
    | Addition
    | Multiplication
    | NotImplemented
    | Input
    | Output
    | Halt;

  let make = intcode => {
    let code =
      switch (intcode mod 100) {
      | 1 => Addition
      | 2 => Multiplication
      | 3 => Input
      | 4 => Output
      | 99 => Halt
      | _ => NotImplemented
      };

    let p1 = Mode.make(intcode / 100 mod 10);
    let p2 = Mode.make(intcode / 1000);

    (code, p1, p2);
  };
};

module Value = {
  let make = (program, value, mode) => {
    switch (mode) {
    | Mode.Position => program->Array.getUnsafe(value)
    | Immediate => value
    };
  };
};

module Action = {
  let make = (program, ~position, ~modes) => {
    let (mode1, mode2) = modes;
    let v1 =
      Value.make(program, program->Array.getUnsafe(position + 1), mode1);
    let v2 =
      Value.make(program, program->Array.getUnsafe(position + 2), mode2);

    (
      v1,
      v2,
      value =>
        program
        ->Array.set(program->Array.getUnsafe(position + 3), value)
        ->ignore,
    );
  };
};

module Steps = {
  let make = opcode =>
    switch (opcode) {
    | OpCode.Addition
    | Multiplication => 4
    | Output
    | Input => 2
    | Halt
    | NotImplemented => 0
    };
};

module Computer = {
  let stepSize = ref(4);
  let output = ref(0);

  let rec make = (program, ~position=0, ~input, ()) => {
    switch (position < Array.length(program)) {
    | false => output^
    | true =>
      switch (program->Array.slice(~len=stepSize^, ~offset=position)) {
      | arr =>
        let (opcode, mode1, mode2) = OpCode.make(arr->Array.getUnsafe(0));
        let (v1, v2, updateProgram) =
          Action.make(program, ~position, ~modes=(mode1, mode2));

        switch (opcode) {
        | Addition =>
          updateProgram(v1 + v2);
          stepSize := Steps.make(opcode);

        | Multiplication =>
          updateProgram(v1 * v2);
          stepSize := Steps.make(opcode);

        | Input =>
          program
          ->Array.set(program->Array.getUnsafe(position + 1), input)
          ->ignore;

          stepSize := Steps.make(opcode);

        | Output =>
          switch (output^) {
          | 0 => output := v1
          | _ => ()
          };

          stepSize := Steps.make(opcode);

        | NotImplemented
        | Halt => ()
        };
      };

      make(program, ~position=position + stepSize^, ~input, ());
    };
  };
};

let make = program => Computer.make(program->Array.copy, ~input=1, ());
