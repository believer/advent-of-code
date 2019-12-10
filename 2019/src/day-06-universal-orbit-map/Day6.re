module PartOne = {
  module WalkTree = {
    let rec make = (id, map) => {
      let ans = ref(0);

      switch (map->Belt.Map.get(id)) {
      | Some(deps) =>
        deps->Belt.List.forEach(c => {
          ans := ans^ + make(c, map);
          ans := ans^ + 1;
        })
      | None => ()
      };

      ans^;
    };
  };

  let make = input => {
    let map = Belt.Map.make(~id=(module Cmp.Str));

    let out =
      input->Belt.Array.reduce(
        map,
        (acc, curr) => {
          let (a, b) =
            switch (curr->Js.String2.split(")")) {
            | [|a, b|] => (a, b)
            | _ => ("", "")
            };

          switch (acc->Belt.Map.get(a)) {
          | Some(v) => acc->Belt.Map.set(a, [b, ...v])
          | None => acc->Belt.Map.set(a, [b])
          };
        },
      );

    out
    ->Belt.Map.toArray
    ->Belt.Array.reduce(0, (acc, (id, _)) => {
        acc + WalkTree.make(id, out)
      });
  };
};
