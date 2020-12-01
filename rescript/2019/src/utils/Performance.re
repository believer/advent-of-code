type t = float;

[@bs.val] [@bs.scope "performance"] external now: unit => t = "now";

module Time = {
  let make = (startTime, endTime) =>
switch ((endTime -. startTime) *. 100. |> Js.Math.round) {
| time when time >= 10000. => (time /. 100.)->Belt.Float.toString ++ "ms"
| time => time->Belt.Float.toString ++ {j| µs|j}
} |> Js.log
};
