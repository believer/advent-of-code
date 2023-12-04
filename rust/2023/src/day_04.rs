// Day 4: Scratchcards
//
// Part one went fast, but part two took a while. I also took a 20 minutes break
// to watch the latest episode of the Swedish TV advent calendar with the kids :D
//
// I think there are some improvements to be made in the code, which I'll look
// at later today.

type Input = Vec<(Vec<u32>, Vec<u32>)>;

#[aoc_generator(day4)]
pub fn input_generator(input: &str) -> Input {
    let mut numbers = vec![];

    for line in input.lines() {
        let line = &line[5..];
        let (_, line) = line.split_once(": ").unwrap();
        let line = line.trim();
        let (winning, losing) = line.split_once(" | ").unwrap();

        let winning: Vec<u32> = winning
            .split_whitespace()
            .map(|x| x.trim().parse().unwrap())
            .collect();

        let my_cards: Vec<u32> = losing
            .split_whitespace()
            .map(|x| x.trim().parse().unwrap())
            .collect();

        numbers.push((winning, my_cards));
    }

    numbers
}

/* Part One
*
* Find all the winning numbers and sum them together.
* The first winning number is worth 1, and for each additional winning number
* the value doubles.
*
* Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
* Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
* Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
* Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
* Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
* Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
*
* So winning numbers sums for each card is:
* 8 + 2 + 2 + 1 + 0 + 0 = 13
*/
// Your puzzle answer was
/// ```
/// use advent_of_code_2023::day_04::*;
/// let data = include_str!("../input/2023/day4.txt");
/// assert_eq!(solve_part_01(&input_generator(data)), 27454);
/// ```
#[aoc(day4, part1)]
pub fn solve_part_01(input: &Input) -> u32 {
    let mut sum = 0;

    for (winning, my_cards) in input {
        let mut card_sum = 0;

        for card in my_cards {
            if winning.contains(card) {
                if card_sum == 0 {
                    card_sum = 1;
                } else {
                    card_sum *= 2;
                }
            }
        }

        sum += card_sum;
    }

    sum
}

/* Part Two
*
* Now the number of winning cards means we win a copy of the scratchcard
* following card.
*
* So, for card 1, we have 4 winning numbers, which means we win a copy of
* the scratchcards 2, 3, 4, 5.
*
* Find out how many scratchcards we win.
*
*/
// Your puzzle answer was
/// ```
/// use advent_of_code_2023::day_04::*;
/// let data = include_str!("../input/2023/day4.txt");
/// assert_eq!(solve_part_02(&input_generator(data)), 6857330);
/// ```
#[aoc(day4, part2)]
pub fn solve_part_02(input: &Input) -> u32 {
    let mut cards = vec![1; input.len()];

    for (i, (winning, my_cards)) in input.iter().enumerate() {
        let mut winning_cards = 0;

        for card in my_cards {
            if winning.contains(card) {
                winning_cards += 1;
            }
        }

        for x in (i + 1)..=(winning_cards + i) {
            cards[x] += cards[i];
        }
    }

    cards.iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_01() {
        let data = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

        assert_eq!(solve_part_01(&input_generator(data)), 13)
    }

    #[test]
    fn sample_02() {
        let data = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

        assert_eq!(solve_part_02(&input_generator(data)), 30)
    }
}
