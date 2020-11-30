pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

pub fn part_01(input: &Vec<Vec<i32>>) -> i32 {
    input.iter().fold(0, |acc, r| {
        let lowest = r.first().unwrap();
        let biggest = r.last().unwrap();

        acc + biggest - lowest
    })
}

pub fn part_02(input: &Vec<Vec<i32>>) -> i32 {
    let mut sum = 0.0;

    for row in input {
        for value in row {
            for other in row {
                if value == other {
                    continue;
                }

                let fr = *value as f32;
                let fo = *other as f32;
                let output = fr / fo;

                if output % 1.0 == 0.0 {
                    sum += output;
                }
            }
        }
    }

    sum as i32
}
