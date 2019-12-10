open Tablecloth;

let make = () => Belt.Set.make(~id=(module Cmp.IntPair));

let add = (~value, set) => set->Belt.Set.add(value);

let length = set => set->Belt.Set.toArray->Array.length;
