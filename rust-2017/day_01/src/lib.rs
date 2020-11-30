pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn char_to_int(value: &char) -> u32 {
    match value.to_digit(10) {
        Some(v) => v,
        None => panic!("Only numbers are allowed"),
    }
}

pub fn part_01(input: &Vec<char>) -> Result<u32> {
    let mut sum = 0;

    for (i, v) in input.iter().enumerate() {
        match (input.get(0), input.get(i + 1)) {
            (Some(v1), None) | (_, Some(v1)) if v == v1 => sum += char_to_int(v),
            _ => (),
        }
    }

    Ok(sum)
}

pub fn part_02(input: &Vec<char>) -> Result<u32> {
    let mut sum = 0;

    for (i, v) in input.iter().enumerate() {
        match input.get((i + input.len() / 2) % input.len()) {
            Some(v1) if v == v1 => sum += char_to_int(v),
            _ => (),
        }
    }

    Ok(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_number_char_to_int() {
        assert_eq!(char_to_int(&'8'), 8);
    }

    #[test]
    #[should_panic(expected = "Only numbers are allowed")]
    fn only_parses_numbers() {
        char_to_int(&'a');
    }
}
