module Moon = {
  let make = (x, y, z) => ((x, y, z), (0., 0., 0.));
};

let data =
  Moon.(
    [|
      make(-10., -10., -13.),
      make(5., 5., -9.),
      make(3., 8., -16.),
      make(1., 3., -3.),
    |]
  );
