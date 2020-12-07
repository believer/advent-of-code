let partOne = () => {
  Day06Data.data->Js.String2.split("\n\n")->Js.Array2.map(group => {
    let yeses = Set.make()

    group->Js.String2.trim->Js.String2.split("\n")->Js.Array2.forEach(person => {
      person->Js.String2.split("")->Js.Array2.forEach(answer => {
        yeses->Set.add(answer)
      })
    })

    yeses.size
  })->Js.Array2.reduce((acc, curr) => acc + curr, 0)
}

let partTwo = () => {
  Day06Data.data->Js.String2.split("\n\n")->Js.Array2.map(group => {
    let yeses = Map.empty()
    let persons = group->Js.String2.trim->Js.String2.split("\n")
    let yes_in_group = ref(0)

    persons->Js.Array2.forEach(person => {
      person->Js.String2.split("")->Js.Array2.forEach(answer => {
        yeses->Map.set(answer, yeses->Map.has(answer) ? yeses->Map.getInt(answer) + 1 : 1)
      })
    })

    yeses->Map.forEach(v => {
      if v == Js.Array2.length(persons) {
        yes_in_group.contents = yes_in_group.contents + 1
      }
    })

    yes_in_group.contents
  })->Js.Array2.reduce((acc, curr) => acc + curr, 0)
}

/* Result.make(partOne, partTwo) */
