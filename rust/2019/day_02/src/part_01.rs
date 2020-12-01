use crate::int_code::IntCode;

pub fn main(input: &Vec<u32>, noun: u32, verb: u32) -> u32 {
    let mut part_01 = IntCode::new(input);

    part_01.patch(1, noun);
    part_01.patch(2, verb);

    part_01.run()
}
