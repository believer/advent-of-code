use std::io;

pub fn fuel(mass: i32) -> i32 {
    (mass as f64 / 3.0).floor() as i32 - 2
}

pub fn main(input: &Vec<i32>) -> io::Result<i32> {
    Ok(input.iter().fold(0, |acc, &mass| acc + fuel(mass)))
}
