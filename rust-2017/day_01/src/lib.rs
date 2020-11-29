pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn parse_value(value: &char) -> u32 {
    value.to_digit(10).unwrap()
}

pub fn part_01(input: &Vec<char>) -> Result<u32> {
    let mut sum = 0;

    for (i, v) in input.iter().enumerate() {
        match (input.get(0), input.get(i + 1)) {
            (Some(v1), None) | (_, Some(v1)) if v == v1 => {
                sum += parse_value(v);
            }
            _ => (),
        }
    }

    Ok(sum)
}

pub fn part_02(_input: &Vec<char>) -> Result<u32> {
    Ok(0)
}
