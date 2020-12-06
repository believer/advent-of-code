type t

@new external make: array<array<string>> => t = "Map"
@new external empty: unit => t = "Map"

@send external get: (t, string) => string = "get"
@send external getInt: (t, string) => int = "get"
@send external set: (t, string, 'a) => unit = "set"
@send external has: (t, string) => bool = "has"
@send external forEach: (t, 'a => unit) => unit = "forEach"
