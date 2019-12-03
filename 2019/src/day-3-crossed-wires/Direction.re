type t =
  | Right
  | Left
  | Up
  | Down;

module DX = {
  let make =
    fun
    | Right => 1
    | Left => (-1)
    | Up => 0
    | Down => 0;
};

module DY = {
  let make =
    fun
    | Right => 0
    | Left => 0
    | Up => 1
    | Down => (-1);
};

let make = cmd => {
  switch (
    cmd->Js.String.get(0),
    cmd |> Js.String.sliceToEnd(~from=1) |> int_of_string,
  ) {
  | ("R", value) => (Right, value)
  | ("L", value) => (Left, value)
  | ("U", value) => (Up, value)
  | (_, value) => (Down, value)
  };
};
