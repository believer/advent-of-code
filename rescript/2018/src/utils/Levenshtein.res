let length = s => Js.String.length(s)
let minimum = (a, b, c) => min(a, min(b, c))

/*
 * Levenshtein distance
 * Translated from OCaml example in Rosetta Code
 * https://rosettacode.org/wiki/Levenshtein_distance#OCaml
 * https://en.wikipedia.org/wiki/Levenshtein_distance
 */
let levenshtein = (s, t) => {
  let first = String.length(s)
  and second = String.length(t)
  let matrix = Array.make_matrix(first + 1, second + 1, 0)

  for i in 0 to first {
    matrix[i][0] = i
  }

  for j in 0 to second {
    matrix[0][j] = j
  }

  for j in 1 to second {
    for i in 1 to first {
      if String.get(s, i - 1) == String.get(t, j - 1) {
        matrix[i][j] = matrix[i - 1][j - 1]
      } else {
        matrix[i][j] = minimum(matrix[i - 1][j] + 1, matrix[i][j - 1] + 1, matrix[i - 1][j - 1] + 1)
      }
    }
  }

  matrix[first][second]
}

let similarity = (s1, s2) => {
  let (longer, shorter) = switch (s1, s2) {
  | (first, second) when length(first) < length(second) => (second, first)
  | (first, second) => (first, second)
  }

  let longerLength = length(longer)

  switch longerLength {
  | 0 => 1.0
  | _ =>
    (float_of_int(longerLength) -. float_of_int(levenshtein(longer, shorter))) /.
      float_of_int(longerLength)
  }
}
