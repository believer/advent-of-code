type t = float;

[@bs.val] [@bs.scope "performance"] external now: unit => t = "now";

module Time = {
  let make = (startTime, endTime) =>
    Js.log(
      ((endTime -. startTime) *. 100. |> Js.Math.round |> Belt.Float.toString)
      ++ {j| µs|j},
    );
};
