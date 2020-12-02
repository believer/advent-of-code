module One = {
  let make = input => input->Belt.Array.reduce(0, (acc, i) => acc + i)
}

module IntCmp = Belt.Id.MakeComparable({
  type t = int
  let cmp = Pervasives.compare
})

module Two = {
  let set = Belt.Set.make(~id=module(IntCmp))

  let rec make = (input, ~start=0, ~set=set, ()) => {
    let (seen, o, v) = input->Js.Array2.reduce(((seen, accV, hasValue), v) => {
      let sum = accV + v

      if Belt.Option.isSome(hasValue) {
        (seen, accV, hasValue)
      } else {
        switch seen->Belt.Set.has(sum) {
        | true => (seen, sum, Some(sum))
        | false =>
          let updated = seen->Belt.Set.add(sum)
          (updated, sum, None)
        }
      }
    }, (set, start, None))

    switch v {
    | None => make(input, ~start=o, ~set=seen, ())
    | Some(v) => v
    }
  }
}
