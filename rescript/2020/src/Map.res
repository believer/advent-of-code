type t = {size: int}

@new external make: array<array<string>> => t = "Map"
@new external empty: unit => t = "Map"

@send external get: (t, string) => string = "get"
@send external getSet: (t, string) => option<Set.t> = "get"
@send external getMap: (t, string) => option<t> = "get"
@send external getInt: (t, string) => int = "get"
@send external set: (t, string, 'a) => unit = "set"
@send external has: (t, string) => bool = "has"
@send external forEach: (t, 'a => unit) => unit = "forEach"
@send external forEachK: (t, ('a, string) => unit) => unit = "forEach"
