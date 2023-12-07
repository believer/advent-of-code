use std::collections::BTreeMap;

// Day 7: Camel Cards
//
// There's probably something better we can do here, but I'll save that for later.

#[derive(Debug)]
pub struct Input {
    hands: Vec<Hand>,
}

#[derive(Debug, Eq, PartialEq, Clone, Hash, Ord, PartialOrd)]
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

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Hand {
    hand_type: HandType,
    cards: Vec<Card>,
    bid: u64,
}

fn parse_hand(s: &str) -> Hand {
    let mut parts = s.split_whitespace();
    let hand = parts.next().unwrap().to_string();
    let bid = parts.next().unwrap().parse::<u64>().unwrap();
    let cards = hand
        .chars()
        .map(|c| parse_card(c, Card::Jack))
        .collect::<Vec<Card>>();
    let mut cards_by_type: BTreeMap<Card, u8> = BTreeMap::new();

    for card in cards.clone() {
        let c = cards_by_type.entry(card).or_insert(0);
        *c += 1;
    }

    let cc = cards_by_type.values().cloned().collect::<Vec<_>>();

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

    Hand {
        cards,
        bid,
        hand_type,
    }
}

fn parse_hand2(s: &str) -> Hand {
    let mut parts = s.split_whitespace();
    let hand = parts.next().unwrap().to_string();
    let bid = parts.next().unwrap().parse::<u64>().unwrap();
    let cards = hand
        .chars()
        .map(|c| parse_card(c, Card::Joker))
        .collect::<Vec<Card>>();
    let mut cards_by_type: BTreeMap<Card, u8> = BTreeMap::new();

    for card in cards.clone() {
        let c = cards_by_type.entry(card).or_insert(0);
        *c += 1;
    }

    let jokers = cards_by_type.get(&Card::Joker).unwrap_or(&0);
    let cc = cards_by_type.values().cloned().collect::<Vec<_>>();

    // Convert cards and number of jokers to hand type
    // Upgrade to the strongest possible hand
    let hand_type = match (&cc[..], jokers) {
        ([5], _) => HandType::FiveOfAKind,

        // Handle no jokers
        (cards, 0) => match cards {
            [1, 1, 1, 1, 1] => HandType::HighCard,
            [2, 1, 1, 1] | [1, 2, 1, 1] | [1, 1, 2, 1] | [1, 1, 1, 2] => HandType::OnePair,
            [2, 1, 2] | [1, 2, 2] | [2, 2, 1] => HandType::TwoPairs,
            [3, 1, 1] | [1, 3, 1] | [1, 1, 3] => HandType::ThreeOfAKind,
            [3, 2] | [2, 3] => HandType::FullHouse,
            [4, 1] | [1, 4] => HandType::FourOfAKind,
            _ => unreachable!("Five of a kind should have been caught above"),
        },

        (cards, 1) => match cards {
            // 1234J -> 12344
            [1, 1, 1, 1, 1] => HandType::OnePair,

            // KKJ12 -> KKK99
            [2, 1, 1, 1] => HandType::ThreeOfAKind,
            [1, 2, 1, 1] => HandType::ThreeOfAKind,
            [1, 1, 2, 1] => HandType::ThreeOfAKind,
            [1, 1, 1, 2] => HandType::ThreeOfAKind,

            // KKJQQ -> KKKQQ
            [2, 1, 2] => HandType::FullHouse,
            [1, 2, 2] => HandType::FullHouse,
            [2, 2, 1] => HandType::FullHouse,

            // 333J1 -> 33331
            [3, 1, 1] => HandType::FourOfAKind,
            [1, 3, 1] => HandType::FourOfAKind,
            [1, 1, 3] => HandType::FourOfAKind,

            // 3333J -> 33333
            [4, 1] => HandType::FiveOfAKind,
            [1, 4] => HandType::FiveOfAKind,

            _ => unreachable!("Five of a kind should have been caught above"),
        },

        (cards, 2) => match cards {
            // 123JJ -> 12333
            [2, 1, 1, 1] => HandType::ThreeOfAKind,
            [1, 2, 1, 1] => HandType::ThreeOfAKind,
            [1, 1, 2, 1] => HandType::ThreeOfAKind,
            [1, 1, 1, 2] => HandType::ThreeOfAKind,

            // KKJJQ -> KKKKQ
            [2, 1, 2] => HandType::FourOfAKind,
            [1, 2, 2] => HandType::FourOfAKind,
            [2, 2, 1] => HandType::FourOfAKind,

            // 333JJ -> 33333
            [3, 1, 1] => HandType::FiveOfAKind,
            [1, 3, 1] => HandType::FiveOfAKind,
            [1, 1, 3] => HandType::FiveOfAKind,

            [2, 3] | [3, 2] => HandType::FiveOfAKind,

            _ => unreachable!("Five of a kind should have been caught above"),
        },

        (cards, 3) => match cards {
            [3, 1, 1] | [1, 3, 1] | [1, 1, 3] => HandType::FourOfAKind,
            [2, 3] | [3, 2] => HandType::FiveOfAKind,

            _ => unreachable!("Five of a kind should have been caught above"),
        },

        (cards, 4) => match cards {
            [4, 1] | [1, 4] => HandType::FiveOfAKind,

            _ => unreachable!("Five of a kind should have been caught above"),
        },

        h => todo!("{:?}", h),
    };

    Hand {
        cards,
        bid,
        hand_type,
    }
}

#[aoc_generator(day7, part1)]
pub fn input_generator(input: &str) -> Input {
    let hands = input.lines().map(parse_hand).collect();

    Input { hands }
}

#[aoc_generator(day7, part2)]
pub fn input_generator_part2(input: &str) -> Input {
    let hands = input.lines().map(parse_hand2).collect();

    Input { hands }
}

fn play(input: &Input) -> u64 {
    let mut ranked: Vec<Hand> = Vec::with_capacity(input.hands.len());
    let mut by_rank: BTreeMap<HandType, Vec<Hand>> = BTreeMap::new();

    // Group hands by rank
    for hand in &input.hands {
        let c = by_rank.entry(hand.hand_type.clone()).or_default();
        c.push(hand.clone());
    }

    // Sort hands by rank and then by cards
    for (_, hands) in by_rank.iter() {
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
*/
// Your puzzle answer was
/*
/// ```
/// use advent_of_code_2023::day_07::*;
/// let data = include_str!("../input/2023/day7.txt");
/// assert_eq!(solve_part_01(&input_generator(data)), 250474325);
/// ```
*/
#[aoc(day7, part1)]
pub fn solve_part_01(input: &Input) -> u64 {
    play(input)
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
/// assert_eq!(solve_part_02(&input_generator_part2(data)), 28101347);
/// ```
*/
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
