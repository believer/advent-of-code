module Map = {
  type t

  @new external make: array<array<string>> => t = "Map"

  @send external get: (t, string) => string = "get"
  @send external has: (t, string) => bool = "has"
}

module Eye = {
  let isValid = eyeColor => {
    switch eyeColor {
    | "amb" | "blu" | "brn" | "gry" | "grn" | "hzl" | "oth" => true
    | _ => false
    }
  }
}

module Height = {
  let isValid = height => {
    switch height {
    | h when h->Js.String2.includes("cm") => {
        let h = h->Js.String2.replace("cm", "")->int_of_string

        if h < 150 || h > 193 {
          false
        } else {
          true
        }
      }
    | h when h->Js.String2.includes("in") => {
        let h = h->Js.String2.replace("in", "")->int_of_string

        if h < 59 || h > 76 {
          false
        } else {
          true
        }
      }
    | _ => false
    }
  }
}

module Hair = {
  let isValid = color => %re(`/^#\w{6}$/`)->Js.Re.test_(color)
}

module PassportId = {
  let isValid = id => %re(`/^\d{9}$/`)->Js.Re.test_(id)
}

module PassportDates = {
  let areValid = (~birth, ~issue, ~expiration) => {
    switch (
      birth < 1920 || birth > 2002,
      issue < 2010 || issue > 2020,
      expiration < 2020 || expiration > 2030,
    ) {
    | (false, false, false) => true
    | _ => false
    }
  }
}

let validator = passport => {
  let byr = passport->Map.get("byr")->int_of_string
  let iyr = passport->Map.get("iyr")->int_of_string
  let eyr = passport->Map.get("eyr")->int_of_string
  let hgt = passport->Map.get("hgt")
  let hcl = passport->Map.get("hcl")
  let ecl = passport->Map.get("ecl")
  let pid = passport->Map.get("pid")

  PassportDates.areValid(~birth=byr, ~issue=iyr, ~expiration=eyr) &&
  Height.isValid(hgt) &&
  Hair.isValid(hcl) &&
  Eye.isValid(ecl) &&
  PassportId.isValid(pid)
}

let parseInput = () => {
  let valid = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid", "cid"]
  let validWithoutCID = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"]

  Day04Data.data->Js.String2.split("\n\n")->Js.Array2.map(l => {
    Map.make(
      l
      ->Js.String2.replaceByRe(%re(`/\n/g`), " ")
      ->Js.String2.trim
      ->Js.String2.split(" ")
      ->Js.Array2.map(l => l->Js.String2.split(":")),
    )
  })->Js.Array2.filter(l => {
    valid->Js.Array2.filter(v => l->Map.has(v))->Js.Array2.length === 8 ||
      validWithoutCID->Js.Array2.filter(v => l->Map.has(v))->Js.Array2.length === 7
  })
}

let partOne = () => parseInput()->Js.Array2.length

let partTwo = () => parseInput()->Js.Array2.filter(validator)->Js.Array2.length

/* Result.make(partOne, partTwo) */
