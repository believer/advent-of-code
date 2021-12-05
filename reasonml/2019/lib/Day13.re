open Tablecloth;

module PartOne = {
  // @ankjevel provided me with the parsed input data through the Intcode
  // computer since I haven't done some of the previous Intcode problems.
  let make = input =>
    input |> List.filter(~f=((_, _, t)) => t === 2) |> List.length;
};
