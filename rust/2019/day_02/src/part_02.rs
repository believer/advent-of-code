pub fn main(input: &Vec<u32>) -> u32 {
    let needle = 19690720;
    let mut result = 0;

    for noun in 0..=99 {
        for verb in 0..=99 {
            if crate::part_01::main(input, noun, verb) == needle {
                result = 100 * noun + verb;
                break;
            }
        }
    }

    result
}
