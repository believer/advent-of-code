module Password = {
  type t = {
    password: string,
    policy: string,
    bounds: array<int>,
  }

  let make = v => {
    let parts = v->Js.String2.split(" ")
    let bounds = parts[0]->Js.String2.split("-")->Js.Array2.map(x => int_of_string(x))
    let password = parts[2]->Js.String2.trim
    let policy = parts[1]->Js.String2.substring(~from=0, ~to_=1)

    {password: password, policy: policy, bounds: bounds}
  }
}

let partOne = () => {
  Day02Input.data->Js.Array2.map(Password.make)->Js.Array2.reduce((acc, {
    password,
    policy,
    bounds,
  }) => {
    let matcher = Js.Re.fromStringWithFlags(policy, ~flags="ig")
    let occurences = password->Js.String2.match_(matcher)

    switch occurences {
    | Some(occurences) =>
      switch (
        Js.Array2.length(occurences) >= bounds[0],
        Js.Array2.length(occurences) <= bounds[1],
      ) {
      | (true, true) => acc + 1
      | _ => acc
      }
    | None => acc
    }
  }, 0)
}

let partTwo = () => {
  Day02Input.data->Js.Array2.map(Password.make)->Js.Array2.reduce((acc, {
    password,
    policy,
    bounds,
  }) => {
    let indices = password->Js.String2.split("")->Js.Array2.mapi((v, i) => {
      if v === policy {
        i
      } else {
        -1
      }
    })->Js.Array2.filter(v => v !== -1)

    let matched = indices->Js.Array2.reduce((acc, v) => {
      if v + 1 === bounds[0] || v + 1 === bounds[1] {
        acc + 1
      } else {
        acc
      }
    }, 0)

    if matched === 1 {
      acc + 1
    } else {
      acc
    }
  }, 0)
}

Result.make(partOne, partTwo)
