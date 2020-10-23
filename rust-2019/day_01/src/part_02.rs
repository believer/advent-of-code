use std::io;

fn calculate_fuel(input: i32, total_fuel: i32) -> i32 {
   match crate::part_01::fuel(input) {
      f if f <= 0 => total_fuel,
      f => calculate_fuel(f, f + total_fuel),
   }
}

pub fn main(input: &Vec<i32>) -> io::Result<i32> {
   Ok(input
      .iter()
      .fold(0, |acc, &mass| acc + calculate_fuel(mass.clone(), 0)))
}
