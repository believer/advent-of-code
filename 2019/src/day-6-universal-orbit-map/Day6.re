module StrCmp =
  Belt.Id.MakeComparable({
    type t = string;
    let cmp = (a, b) => Pervasives.compare(a, b);
  });

module PartOne = {
  module WalkTree = {
    let rec make = (id, map) => {
      let ans = ref(0);

      switch (map->Map.get(id)) {
      | Some(deps) =>
        deps->List.forEach(c => {
          ans := ans^ + make(c, map);
          ans := ans^ + 1;
        })
      | None => ()
      };

      ans^;
    };
  };

  let make = input => {
    let map = Map.make(~id=(module StrCmp));

    let out =
      input->Array.reduce(
        map,
        (acc, curr) => {
          let (a, b) =
            switch (curr->Js.String2.split(")")) {
            | [|a, b|] => (a, b)
            | _ => ("", "")
            };

          switch (acc->Map.get(a)) {
          | Some(v) => acc->Map.set(a, [b, ...v])
          | None => acc->Map.set(a, [b])
          };
        },
      );

    out
    ->Map.toArray
    ->Array.reduce(0, (acc, (id, _)) => {acc + WalkTree.make(id, out)});
  };
};
