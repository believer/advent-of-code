use std::collections::BTreeMap;

// Day 7:

#[derive(Debug, Eq, PartialEq, Clone, Hash, Ord, PartialOrd)]
enum Card {
    Number(u8),
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

#[derive(Debug, Eq, PartialEq, Clone, Ord, PartialOrd)]
enum HandType {
    HighCard,
    OnePair,
    TwoPairs,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Hand {
    hand_type: HandType,
    cards: Vec<Card>,
    bid: u64,
}

impl From<&str> for Hand {
    fn from(s: &str) -> Self {
        let mut parts = s.split_whitespace();
        let hand = parts.next().unwrap().to_string();
        let bid = parts.next().unwrap().parse::<u64>().unwrap();

        let cards = hand
            .chars()
            .map(|c| match c {
                'T' => Card::Ten,
                'J' => Card::Jack,
                'Q' => Card::Queen,
                'K' => Card::King,
                'A' => Card::Ace,
                _ => Card::Number(c.to_digit(10).unwrap() as u8),
            })
            .collect::<Vec<_>>();

        let mut cards_by_type: BTreeMap<Card, u8> = BTreeMap::new();

        for card in cards.clone() {
            let c = cards_by_type.entry(card).or_insert(0);
            *c += 1;
        }

        let cc = cards_by_type.values().collect::<Vec<_>>();

        let hand_type = match &cc[..] {
            [1, 1, 1, 1, 1] => HandType::HighCard,
            [1, 1, 1, 2] | [1, 1, 2, 1] | [1, 2, 1, 1] | [2, 1, 1, 1] => HandType::OnePair,
            [1, 2, 2] | [2, 1, 2] | [2, 2, 1] => HandType::TwoPairs,
            [1, 1, 3] | [1, 3, 1] | [3, 1, 1] => HandType::ThreeOfAKind,
            [2, 3] | [3, 2] => HandType::FullHouse,
            [1, 4] | [4, 1] => HandType::FourOfAKind,
            [5] => HandType::FiveOfAKind,
            _ => panic!("Unknown hand type"),
        };

        Self {
            cards,
            bid,
            hand_type,
        }
    }
}

#[derive(Debug)]
pub struct Input {
    hands: Vec<Hand>,
}

#[aoc_generator(day7)]
pub fn input_generator(input: &str) -> Input {
    let hands = input.lines().map(|line| line.into()).collect();

    Input { hands }
}

/* Part One
*
*/
// Your puzzle answer was
/*
/// ```
/// use advent_of_code_2023::day_07::*;
/// let data = include_str!("../input/2023/day7.txt");
/// assert_eq!(solve_part_01(&input_generator(data)), 861300);
/// ```
*/
#[aoc(day7, part1)]
pub fn solve_part_01(input: &Input) -> u64 {
    let mut ranked: Vec<Hand> = Vec::with_capacity(input.hands.len());
    let mut by_rank: BTreeMap<HandType, Vec<Hand>> = BTreeMap::new();
    let mut sum = 0;

    for hand in &input.hands {
        let c = by_rank.entry(hand.hand_type.clone()).or_default();
        c.push(hand.clone());
    }

    for (_, hands) in by_rank.iter() {
        let mut hands = hands.clone();

        hands.sort_by(|a, b| match (a.cards.clone(), b.cards.clone()) {
            (a, b) if a == b => a.cmp(&b),
            (a, b) => a.cmp(&b),
        });

        ranked.append(&mut hands);
    }

    for (i, hand) in ranked.iter().enumerate() {
        sum += hand.bid * (i + 1) as u64;
    }

    sum
}

/* Part Two
*
* Here we combine all the race times and distance from the previous data into
* two large numbers. This means we only have one race, but since the time
* is longer, we have a lot more ways to beat the record.
*
*/
/*
/// ```
/// use advent_of_code_2023::day_07::*;
/// let data = include_str!("../input/2023/day7.txt");
/// assert_eq!(solve_part_02(&input_generator(data)), 28101347);
/// ```
*/
#[aoc(day7, part2)]
pub fn solve_part_02(_input: &Input) -> u64 {
    0
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
}
