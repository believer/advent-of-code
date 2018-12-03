let handleRegex = (row, re, split) =>
  switch (Js.String.match(re, row)) {
  | Some(c) =>
    let values = c[0] |> Js.String.split(split);
    (int_of_string(values[0]), int_of_string(values[1]));
  | None => (0, 0)
  };

let getId = row =>
  switch (Js.String.match([%re "/#\\d+/"], row)) {
  | Some(c) => Js.String.replace("#", "", c[0])
  | None => "0"
  };

let coordinates = row => row->handleRegex([%re "/\\d+,\\d+/"], ",");
let size = row => row->handleRegex([%re "/\\d+x\\d+/"], "x");

let updateValue = (current, newValue) =>
  switch (current) {
  | "0" => newValue
  | x when x == newValue => newValue
  | _ => "x"
  };

let findOverlap = input => {
  let matrix = Array.make_matrix(1000, 1000, "0");

  input->Belt.Array.map(row => {
    let (x, y) = coordinates(row);
    let (sx, sy) = size(row);
    let id = getId(row);

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

  matrix->Utils.flatten->Belt.Array.keep(v => v == "x")->Belt.Array.length;
};
