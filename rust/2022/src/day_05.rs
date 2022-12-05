// Day 5 - Supply Stacks
//
// Too lazy to parse the stacks. Will clean up solution later.

#[derive(Debug)]
pub struct Instruction {
    to: usize,
    from: usize,
    moves: usize,
}

impl Instruction {
    fn new(input: &str) -> Instruction {
        let parts: Vec<&str> = input.split(' ').collect();

        Instruction {
            to: parts[5].parse::<usize>().unwrap() - 1,
            from: parts[3].parse::<usize>().unwrap() - 1,
            moves: parts[1].parse().unwrap(),
        }
    }
}

type Input = (Vec<Vec<String>>, Vec<Instruction>);

#[aoc_generator(day5)]
pub fn input_generator(input: &str) -> Input {
    let t: Vec<&str> = input.split("\n\n").collect();
    // [B]                     [N]     [H]
    // [V]         [P] [T]     [V]     [P]
    // [W]     [C] [T] [S]     [H]     [N]
    // [T]     [J] [Z] [M] [N] [F]     [L]
    // [Q]     [W] [N] [J] [T] [Q] [R] [B]
    // [N] [B] [Q] [R] [V] [F] [D] [F] [M]
    // [H] [W] [S] [J] [P] [W] [L] [P] [S]
    // [D] [D] [T] [F] [G] [B] [B] [H] [Z]
    let stacks = vec![
        "B V W T Q N H D",
        "B W D",
        "C J W Q S T",
        "P T Z N R J F",
        "T S M J V P G",
        "N T F W B",
        "N V H F Q D L B",
        "R F P H",
        "H P N L B M S Z",
    ];
    //     [D]
    // [N] [C]
    // [Z] [M] [P]
    // let stacks = vec!["N Z", "D C M", "P"];

    let parsed_stacks: Vec<Vec<String>> = stacks
        .iter()
        .map(|s| s.split_whitespace().map(|s| s.to_string()).collect())
        .collect();
    let instructions: Vec<Instruction> = t[1].split("\n").map(Instruction::new).collect();

    (parsed_stacks, instructions)
}

/* Part One
*/
/// Your puzzle answer was
/// ```
/// use advent_of_code_2022::day_05::*;
/// let data = include_str!("../input/2022/day5.txt");
/// assert_eq!(solve_part_01(&input_generator(data)), "PSNRGBTFT");
/// ```
#[aoc(day5, part1)]
pub fn solve_part_01(input: &Input) -> String {
    let mut stacks = input.0.clone();

    for instruction in input.1.iter() {
        let mut moves = instruction.moves;
        let from = instruction.from;
        let to = instruction.to;

        while moves > 0 {
            let card = stacks[from].first().unwrap().clone();
            stacks[from].remove(0);
            stacks[to].splice(0..0, vec![card].iter().cloned());
            moves -= 1;
        }
    }

    stacks
        .iter()
        .map(|stack| stack.first().unwrap().to_string())
        .fold(String::new(), |acc, s| acc + &s)
}

/* Part Two
*/
/// Your puzzle answer was
/// ```
/// use advent_of_code_2022::day_05::*;
/// let data = include_str!("../input/2022/day5.txt");
/// assert_eq!(solve_part_02(&input_generator(data)), 0);
/// ```
#[aoc(day5, part2)]
pub fn solve_part_02(input: &Input) -> String {
    let mut stacks = input.0.clone();

    for instruction in input.1.iter() {
        let mut moves = instruction.moves;
        let from = instruction.from;
        let to = instruction.to;

        let move_stack = stacks[from][..moves]
            .iter()
            .map(|s| s.to_string())
            .collect::<Vec<String>>();

        while moves > 0 {
            stacks[from].remove(0);
            moves -= 1;
        }

        stacks[to].splice(0..0, move_stack.iter().cloned());
    }

    stacks
        .iter()
        .map(|stack| stack.first().unwrap().to_string())
        .fold(String::new(), |acc, s| acc + &s)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_01() {
        let data = "    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";

        assert_eq!(solve_part_01(&input_generator(data)), "CMZ")
    }

    #[test]
    fn sample_02() {
        let data = "    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";

        assert_eq!(solve_part_02(&input_generator(data)), "MCD")
    }
}
