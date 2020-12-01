pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

pub fn part_01(input: &Vec<i32>) -> Result<i32> {
    let mut s = 0;

    for x in input {
        for y in input {
            match x + y {
                2020 => s = x * y,
                _ => continue,
            }
        }
    }

    Ok(s)
}

pub fn part_02(input: &Vec<i32>) -> Result<i32> {
    let mut s = 0;

    for x in input {
        for y in input {
            for z in input {
                match x + y + z {
                    2020 => s = x * y * z,
                    _ => continue,
                }
            }
        }
    }

    Ok(s)
}
