use nom::{
    bytes::streaming::tag, character::complete::newline, multi::separated_list1,
    sequence::separated_pair, IResult,
};

fn day_01<'a>() -> IResult<&'a str, Vec<u32>> {
    let example = "1000
2000
3000

4000

5000
6000

7000
8000
9000

10000";

    let (input, output) = nom::multi::separated_list1(
        tag("\n\n"),
        separated_list1(newline, nom::character::complete::u32),
    )(example)?;

    println!("{output:?}");

    Ok((example, vec![]))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_day_01() {
        assert_eq!(day_01().unwrap().1, vec![6000, 4000, 11000, 24000, 10000])
    }
}
