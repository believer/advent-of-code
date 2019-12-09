module Mode = {
  type t =
    | Immediate
    | Position;
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

    let p1 =
      switch (intcode / 100 mod 10) {
      | 0 => Mode.Position
      | _ => Immediate
      };
    let p2 =
      switch (intcode / 1000) {
      | 0 => Mode.Position
      | _ => Immediate
      };

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

module PartOne = {
  let stepSize = ref(4);
  let output = ref(0);
  let input = 1;

  let rec make = (program, ~position=0, ()) => {
    switch (position < Array.length(program)) {
    | false => output^
    | true =>
      switch (program->Array.slice(~len=stepSize^, ~offset=position)) {
      | arr =>
        let (opcode, mode1, mode2) = OpCode.make(arr->Array.getUnsafe(0));

        switch (opcode) {
        | Addition =>
          let v1 =
            Value.make(
              program,
              program->Array.getUnsafe(position + 1),
              mode1,
            );
          let v2 =
            Value.make(
              program,
              program->Array.getUnsafe(position + 2),
              mode2,
            );

          program
          ->Array.set(program->Array.getUnsafe(position + 3), v1 + v2)
          ->ignore;

          stepSize := 4;

        | Multiplication =>
          let v1 =
            Value.make(
              program,
              program->Array.getUnsafe(position + 1),
              mode1,
            );
          let v2 =
            Value.make(
              program,
              program->Array.getUnsafe(position + 2),
              mode2,
            );

          program
          ->Array.set(program->Array.getUnsafe(position + 3), v1 * v2)
          ->ignore;

          stepSize := 4;

        | Input =>
          program
          ->Array.set(program->Array.getUnsafe(position + 1), input)
          ->ignore;

          stepSize := 2;

        | Output =>
          switch (output^) {
          | 0 =>
            output :=
              Value.make(
                program,
                program->Array.getUnsafe(position + 1),
                mode1,
              )
          | _ => ()
          };

          stepSize := 2;

        | NotImplemented
        | Halt => ()
        };
      };

      make(program, ~position=position + stepSize^, ());
    };
  };
};
