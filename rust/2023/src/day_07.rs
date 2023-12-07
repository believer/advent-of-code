use std::{
    collections::{BTreeMap, HashMap},
    ops::Add,
};

// Day 7: Camel Cards
//
// The first parts was pretty straight forward, not too many cases to handle.
// The second part was a bit more tricky, mostly due to convert the jokers to
// the best possible hand.
//
// I had bunch of match statements, but I refactored them using Add and From
// traits (gotta love traits). This made the code a lot more readable and
// easier to handle.

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

// Used to convert a vector of card counts to a hand type
// It's important that the vector is sorted, otherwise we
// have to handle more cases.
impl From<Vec<&u8>> for HandType {
    fn from(value: Vec<&u8>) -> Self {
        match &value[..] {
            [1, 1, 1, 1, 1] => Self::HighCard,
            [1, 1, 1, 2] => Self::OnePair,
            [1, 2, 2] => Self::TwoPairs,
            [1, 1, 3] => Self::ThreeOfAKind,
            [2, 3] => Self::FullHouse,
            [1, 4] => Self::FourOfAKind,
            [5] => Self::FiveOfAKind,
            _ => panic!("Unknown hand type"),
        }
    }
}

// Used to convert a hand with jokers to the best possible hand
// If jokers is 0, return the initial hand type
impl Add<&u8> for HandType {
    type Output = Self;

    fn add(self, number: &u8) -> Self::Output {
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

fn parse_hand(s: &str, j_variant: Card) -> Hand {
    // Split the string into the hand and the bid
    let mut parts = s.split_whitespace();
    let hand = parts.next().unwrap().to_string();
    let bid = parts.next().unwrap().parse::<u64>().unwrap();

    // Parse hand into a vector of cards
    let cards = hand
        .chars()
        .map(|c| parse_card(c, j_variant))
        .collect::<Vec<Card>>();

    // Count the number of each card
    let mut cards_by_type: HashMap<Card, u8> = HashMap::new();

    for card in cards.clone() {
        *cards_by_type.entry(card).or_insert(0) += 1;
    }

    // Get the values, i.e. how many of each card we have, and sort them
    let mut card_values = cards_by_type.values().collect::<Vec<_>>();
    card_values.sort();

    // Get the number of jokers
    let jokers = cards_by_type.get(&Card::Joker).unwrap_or(&0);

    // Add the jokers to the hand type
    let hand_type = HandType::from(card_values) + jokers;

    Hand {
        cards,
        bid,
        hand_type,
    }
}

#[aoc_generator(day7, part1)]
pub fn input_generator(input: &str) -> Input {
    let hands = input
        .lines()
        .map(|line| parse_hand(line, Card::Jack))
        .collect();

    Input { hands }
}

#[aoc_generator(day7, part2)]
pub fn input_generator_part2(input: &str) -> Input {
    let hands = input
        .lines()
        .map(|line| parse_hand(line, Card::Joker))
        .collect();

    Input { hands }
}

fn play(input: &Input) -> u64 {
    // Create a vector for sorting the hands by rank
    let mut ranked: Vec<Hand> = Vec::with_capacity(input.hands.len());

    // Create a map for grouping hands by rank
    let mut by_rank: BTreeMap<HandType, Vec<Hand>> = BTreeMap::new();

    // Group hands by rank
    for hand in &input.hands {
        by_rank
            .entry(hand.hand_type.clone())
            .or_default()
            .push(hand.clone());
    }

    // Sort hands by rank and then by cards
    for hands in by_rank.values() {
        let mut hands = hands.clone();

        hands.sort_by(|a, b| match (a.cards.clone(), b.cards.clone()) {
            (a, b) if a == b => a.cmp(&b),
            (a, b) => a.cmp(&b),
        });

        ranked.append(&mut hands);
    }

    // Sum the bid * rank
    ranked
        .iter()
        .enumerate()
        .map(|(i, hand)| hand.bid * (i + 1) as u64)
        .sum()
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
