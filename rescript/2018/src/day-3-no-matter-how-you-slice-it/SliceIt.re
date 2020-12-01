open Belt.Array;

let ios = int_of_string;

let parseLine = row =>
  switch (
    Js.String.match([%re "/\\#(\\d+) @ (\\d+),(\\d+)\\: (\\d+)x(\\d+)/"], row)
  ) {
  | Some(v) => (v[1], ios(v[2]), ios(v[3]), ios(v[4]), ios(v[5]))
  | None => ("0", 0, 0, 0, 0)
  };

let updateValue = (current, newValue) =>
  switch (current) {
  | "0" => newValue
  | x when x == newValue => newValue
  | _ => "x"
  };

let createMatrix = input => {
  let matrix = Array.make_matrix(1000, 1000, "0");

  input->map(row => {
    let (id, x, y, sx, sy) = parseLine(row);

    for (i in 0 to sx - 1) {
      let currentX = matrix[y][x + i];
      matrix[y][x + i] = updateValue(currentX, id);

      for (j in 0 to sy - 1) {
        let currentY = matrix[y + j][x + i];
        matrix[y + j][x + i] = updateValue(currentY, id);
      };
    };
  })
  |> ignore;

  matrix;
};

let findOverlap = input =>
  createMatrix(input)->Utils.flatten->keep(v => v == "x")->length;

let findFabric = input => {
  let matrix = createMatrix(input);

  let output =
    input
    ->map(row => {
        let (id, x, y, sx, sy) = parseLine(row);
        let currentMatrix = Array.make_matrix(sy, sx, "0");

        for (i in 0 to sx - 1) {
          currentMatrix[0][i] = matrix[y][x + i];

          for (j in 0 to sy - 1) {
            currentMatrix[j][i] = matrix[y + j][x + i];
          };
        };

        (currentMatrix, id);
      })
    ->map(((block, id)) =>
        (block->keep(row => Js.Array.includes("x", row)), id)
      )
    ->keep(((block, _)) => length(block) == 0)
    ->get(0);

  switch (output) {
  | Some((_, id)) => id
  | None => "Couldn't find a fabric without overlap"
  };
};
