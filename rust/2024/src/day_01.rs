use std::collections::HashMap;

#[aoc_generator(day1)]
pub fn input_generator(input: &str) -> (Vec<usize>, Vec<usize>) {
    let mut left = vec![];
    let mut right = vec![];

    input.lines().for_each(|line| {
        let mut values = line.split_whitespace();

        left.push(values.next().unwrap().parse::<usize>().unwrap());
        right.push(values.next().unwrap().parse::<usize>().unwrap());
    });

    (left, right)
}

#[aoc(day1, part1)]
pub fn solve_part_01(input: &(Vec<usize>, Vec<usize>)) -> usize {
    let (mut left, mut right) = input.clone();

    left.sort();
    right.sort();

    left.iter().zip(right).map(|(l, r)| l.abs_diff(r)).sum()
}

#[aoc(day1, part2)]
pub fn solve_part_02(input: &(Vec<usize>, Vec<usize>)) -> usize {
    let (left, right) = input.clone();
    let mut appears: HashMap<usize, usize> = HashMap::new();

    for r in right {
        appears.entry(r).and_modify(|e| *e += 1).or_insert(1);
    }

    left.iter()
        .map(|l| match appears.get(l) {
            Some(v) => l * v,
            None => 0,
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const DATA: &str = "3   4
4   3
2   5
1   3
3   9
3   3";

    #[test]
    fn sample_01() {
        assert_eq!(solve_part_01(&input_generator(DATA)), 11)
    }

    #[test]
    fn sample_02() {
        assert_eq!(solve_part_02(&input_generator(DATA)), 31)
    }
}
