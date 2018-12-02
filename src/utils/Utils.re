let flatten = input =>
  input->Belt.Array.reduce([||], (acc, curr) =>
    Belt.Array.concat(acc, curr)
  );
