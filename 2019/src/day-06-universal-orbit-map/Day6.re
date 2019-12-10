open Tablecloth;

module PartOne = {
  module WalkTree = {
    let rec make = (key, map) => {
      let ans = ref(0);

      switch (map->StrDict.get(~key)) {
      | Some(deps) =>
        deps->List.iter(~f=c => {
          ans := ans^ + make(c, map);
          ans := ans^ + 1;
        })
      | None => ()
      };

      ans^;
    };
  };

  let make = input => {
    let map = StrDict.empty;

    let out =
      input->Array.foldLeft(
        ~initial=map,
        ~f=(curr, acc) => {
          let (a, b) =
            switch (curr->String.split(~on=")")) {
            | [a, b] => (a, b)
            | _ => ("", "")
            };

          switch (acc->StrDict.get(~key=a)) {
          | Some(v) => acc->StrDict.insert(~key=a, ~value=[b, ...v])
          | None => acc->StrDict.insert(~key=a, ~value=[b])
          };
        },
      );

    out
    ->StrDict.toList
    ->List.foldl(~init=0, ~f=((id, _), acc) => {
        acc + WalkTree.make(id, out)
      });
  };
};
