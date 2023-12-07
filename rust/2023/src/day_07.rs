use itertools::Itertools;
use std::{ops::Add, str::Chars};

// Day 7: Camel Cards
//
// The first parts was pretty straight forward, not too many cases to handle.
// The second part was a bit more tricky, mostly due to convert the jokers to
// the best possible hand.
//
// I had bunch of match statements, but I refactored them using Add and From
// traits (gotta love traits). This made the code a lot more readable and
// easier to handle.
//
// I also used Itertools to simplify counting and sorting. It's actually
// a pretty powerful crate, and I should probably use it more often.

#[derive(Debug)]
pub struct Input {
    hands: Vec<Hand>,
}

// The order here is important, since we want to sort the hands by rank
#[derive(Debug, Eq, PartialEq, Clone, Hash, Copy, Ord, PartialOrd)]
enum Card {
    Joker,
    Number(u8),
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

fn parse_card<T: Into<Card>>(c: char, j_variant: T) -> Card {
    match c {
        'T' => Card::Ten,
        'J' => j_variant.into(),
        'Q' => Card::Queen,
        'K' => Card::King,
        'A' => Card::Ace,
        _ => Card::Number(c.to_digit(10).unwrap() as u8),
    }
}

#[derive(Debug, Eq, PartialEq, Copy, Clone, Ord, PartialOrd)]
enum HandType {
    HighCard,
    OnePair,
    TwoPairs,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

// Used to convert a vector of card counts to a hand type
// It's important that the vector is sorted, otherwise we
// have to handle more cases.
impl From<&Chars<'_>> for HandType {
    fn from(value: &Chars<'_>) -> Self {
        // Simplify counting and sorting using Itertools
        let hand_type = value.clone().counts().values().sorted().join("");

        match &hand_type[..] {
            "11111" => Self::HighCard,
            "1112" => Self::OnePair,
            "122" => Self::TwoPairs,
            "113" => Self::ThreeOfAKind,
            "23" => Self::FullHouse,
            "14" => Self::FourOfAKind,
            "5" => Self::FiveOfAKind,
            _ => panic!("Unknown hand type"),
        }
    }
}

// Used to convert a hand with jokers to the best possible hand
// If jokers is 0, return the initial hand type
impl Add<u8> for HandType {
    type Output = Self;

    fn add(self, number: u8) -> Self::Output {
        match (self, number) {
            (initial, 0) => initial,
            (Self::FiveOfAKind, _) => Self::FiveOfAKind,

            (Self::HighCard, 1) => Self::OnePair,
            (Self::OnePair, 1) => Self::ThreeOfAKind,
            (Self::TwoPairs, 1) => Self::FullHouse,
            (Self::ThreeOfAKind, 1) => Self::FourOfAKind,
            (Self::FourOfAKind, 1) => Self::FiveOfAKind,

            (Self::OnePair, 2) => Self::ThreeOfAKind,
            (Self::TwoPairs, 2) => Self::FourOfAKind,
            (Self::ThreeOfAKind, 2) => Self::FiveOfAKind,
            (Self::FullHouse, 2) => Self::FiveOfAKind,

            (Self::ThreeOfAKind, 3) => Self::FourOfAKind,
            (Self::FullHouse, 3) => Self::FiveOfAKind,
            (Self::FourOfAKind, 4) => Self::FiveOfAKind,

            (c, j) => panic!("Unhandled card addition {:?} + {:?}", c, j),
        }
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Hand {
    hand_type: HandType,
    cards: Vec<Card>,
    bid: u64,
}

impl Hand {
    fn new(s: &str, j_variant: Card) -> Self {
        // Split the line into hand and bid
        let mut parts = s.split_whitespace();
        let hand = parts.next().unwrap().chars();
        let bid = parts.next().unwrap().parse::<u64>().unwrap();

        // Find what type of hand we have and its cards
        // j_variant is used to convert either jacks for part 1 or jokers for part 2
        let mut hand_type = HandType::from(&hand);
        let cards = hand.map(|c| parse_card(c, j_variant)).collect::<Vec<_>>();

        // If we have jokers, convert the hand type to the best possible hand
        if cards.contains(&Card::Joker) {
            let jokers = cards.iter().filter(|c| **c == Card::Joker).count() as u8;
            hand_type = hand_type + jokers;
        }

        Self {
            cards,
            bid,
            hand_type,
        }
    }
}

#[aoc_generator(day7, part1)]
pub fn input_generator(input: &str) -> Input {
    let hands = input
        .lines()
        .map(|line| Hand::new(line, Card::Jack))
        .collect();

    Input { hands }
}

#[aoc_generator(day7, part2)]
pub fn input_generator_part2(input: &str) -> Input {
    let hands = input
        .lines()
        .map(|line| Hand::new(line, Card::Joker))
        .collect();

    Input { hands }
}

fn play(input: &Input) -> u64 {
    input
        .hands
        .iter()
        // Sort hands by rank and then by cards
        .sorted_by_key(|h| (h.hand_type, h.cards.clone()))
        .enumerate()
        // Multiply the bid by the rank
        .map(|(i, hand)| hand.bid * (i + 1) as u64)
        .sum::<u64>()
}

/* Part One
*
* We have a list of cards, and we need to sort them by rank.
* Then we need to multiply the rank by the bid (the second column)
* and sum the total.
*
* 32T3K 765
* T55J5 684
* KK677 28
* KTJJT 220
* QQQJA 483
*
* ## Sort by rank
*
* 1. 32T3K 765 -> One pair
* 2. KTJJT 220 -> Two pairs
* 3. KK677 28 -> Two pairs (K is higher than T)
* 4. T55J5 684 -> Three of a kind
* 5. QQQJA 483 -> Three of a kind (Q is higher than T)
*
* ## Multiply by bid
*
* 1. 765 * 1 = 765
* 2. 220 * 2 = 440
* 3. 28 * 3 = 84
* 4. 684 * 4 = 2736
* 5. 483 * 5 = 2415
*
* ## Sum
*
* 765 + 440 + 84 + 2736 + 2415 = 6440
*
*/
// Your puzzle answer was
/// ```
/// use advent_of_code_2023::day_07::*;
/// let data = include_str!("../input/2023/day7.txt");
/// assert_eq!(solve_part_01(&input_generator(data)), 250474325);
/// ```
#[aoc(day7, part1)]
pub fn solve_part_01(input: &Input) -> u64 {
    play(input)
}

/* Part Two
*
* Now the cards that were previously jacks are jokers. These are wild cards
* and should be used to create the best possible hand. Otherwise the rules
* are the same.
*
*/
/// ```
/// use advent_of_code_2023::day_07::*;
/// let data = include_str!("../input/2023/day7.txt");
/// assert_eq!(solve_part_02(&input_generator_part2(data)), 248909434);
/// ```
#[aoc(day7, part2)]
pub fn solve_part_02(input: &Input) -> u64 {
    play(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    const DATA: &str = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";

    #[test]
    fn sample_01() {
        assert_eq!(solve_part_01(&input_generator(DATA)), 6440)
    }

    #[test]
    fn sample_02() {
        assert_eq!(solve_part_02(&input_generator_part2(DATA)), 5905)
    }
}
