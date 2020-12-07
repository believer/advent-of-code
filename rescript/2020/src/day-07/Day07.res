let sample = `light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags.`

let canContain = Map.empty()
let bagCosts = Map.empty()

let parser = () => {
  open Js.String2

  Day07Data.data->split("\n")->Js.Array2.filter(r => r->length !== 0)->Js.Array2.forEach(rule => {
    switch rule->split(" bags contain ") {
    | [color, bags] =>
      bags
      ->split(",")
      ->Js.Array2.map(b => trim(b))
      ->Js.Array2.map(l => l->replaceByRe(%re(`/bags?[,.]?/g`), "")->trim)
      ->Js.Array2.forEach(l => {
        let n = l->match_(%re(`/\d+/`))
        let c = l->replaceByRe(%re(`/\d+/g`), "")->trim

        switch n {
        | Some([n]) =>
          switch canContain->Map.getSet(c) {
          | Some(cc) => cc->Set.add(color)
          | None => {
              let d = Set.make()
              d->Set.add(color)
              canContain->Map.set(c, d)
            }
          }

          switch bagCosts->Map.getMap(color) {
          | Some(cc) => cc->Map.set(c, n)
          | None => {
              let d = Map.empty()
              d->Map.set(c, n)
              bagCosts->Map.set(color, d)
            }
          }
        | _ => canContain->Map.set(color, Set.make())
        }
      })

    | _ => ()
    }
  })

  (canContain, bagCosts)
}

let holdsGold = Set.make()

let rec searcher = (bags: Map.t, color) => {
  let c = bags->Map.getSet(color)

  switch c {
  | Some(c) => c->Set.forEach(v => {
      holdsGold->Set.add(v)
      searcher(bags, v)
    })
  | None => ()
  }
}

let rec calculate_cost = (bags: Map.t, color) => {
  let total = ref(0)

  switch bags->Map.getMap(color) {
  | Some(bag) => {
      bag->Map.forEachK((v, k) => {
        let c = int_of_string(v)
        total := total.contents + c
        total := total.contents + c * calculate_cost(bags, k)
      })

      total.contents
    }
  | None => total.contents
  }
}

let partOne = () => {
  let (bags, _) = parser()
  searcher(bags, "shiny gold")

  holdsGold.size
}

let partTwo = () => {
  let (_, bags) = parser()
  calculate_cost(bags, "shiny gold")
}

Result.make(partOne, partTwo)
