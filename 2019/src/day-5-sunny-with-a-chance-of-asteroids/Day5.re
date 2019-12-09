module OpCode = {
  type t =
    | Addition
    | Multiplication
    | NotImplemented
    | Halt;

  let make = intcode =>
    switch (intcode->Int.toString->Js.String2.slice(~from=-2, ~to_=2)) {
    | "1" => Addition
    | "2" => Multiplication
    | "99" => Halt
    | _ => NotImplemented
    };
};

module Memory = {
  let update = (mem, pos, value) => mem->Array.set(pos, value)->ignore;

  let make = input => input->Array.copy;
};

module Parameters = {
  let make = (~opcode, ~program, ~position) => {
    switch (opcode) {
    | OpCode.Multiplication
    | Addition =>
      let v1 = program->Array.get(program->Array.getUnsafe(position + 1));
      let v2 = program->Array.get(program->Array.getUnsafe(position + 2));
      let output = program->Array.getUnsafe(position + 3);

      (v1, v2, output);
    | Halt
    | NotImplemented => (None, None, 0)
    };
  };
};

module Operation = {
  let make = (~opcode, ~program, ~position) => {
    switch (opcode) {
    | OpCode.Addition =>
      let (v1, v2, output) = Parameters.make(~opcode, ~program, ~position);

      switch (v1, v2) {
      | (Some(v1), Some(v2)) => program->Array.set(output, v1 + v2)->ignore
      | _ => ()
      };

    | Multiplication =>
      let (v1, v2, output) = Parameters.make(~opcode, ~program, ~position);

      switch (v1, v2) {
      | (Some(v1), Some(v2)) => program->Array.set(output, v1 * v2)->ignore
      | _ => ()
      };
    | _ => ()
    };

    program;
  };
};

module Movement = {
  let make =
    fun
    | OpCode.Addition => 4
    | Multiplication => 4
    | Halt
    | NotImplemented => 0;
};

module Computer = {
  let rec make = (input, ~position=0, ()) => {
    let program = Memory.make(input);
    let cmd = program->Array.get(position);

    switch (cmd) {
    | Some(cmd) =>
      let opcode = OpCode.make(cmd);

      switch (opcode) {
      | Halt => program
      | _ =>
        let program = Operation.make(~opcode, ~program, ~position);
        let movement = Movement.make(opcode);

        make(program, ~position=position + movement, ());
      };
    | None => program
    };
  };
};

module PartOne = {
  let make = input => Computer.make(input, ());
};
