use std::collections::{HashSet, VecDeque};

// Day 22 - Crab Combat

#[aoc_generator(day22)]
pub fn input_generator(input: &str) -> (VecDeque<u32>, VecDeque<u32>) {
    let players: VecDeque<VecDeque<u32>> = input
        .split("\n\n")
        .map(|p| {
            let player: Vec<&str> = p.lines().collect();

            player[1..]
                .iter()
                .map(|c| c.parse::<u32>().unwrap())
                .collect()
        })
        .collect();

    (players[0].clone(), players[1].clone())
}

fn calculate_score(player: VecDeque<u32>) -> u32 {
    player
        .iter()
        .enumerate()
        .fold(0, |acc, (i, v)| acc + (v * (player.len() - i) as u32))
}

fn combat(player_1: &VecDeque<u32>, player_2: &VecDeque<u32>) -> (u32, VecDeque<u32>) {
    let mut player_1 = player_1.to_owned();
    let mut player_2 = player_2.to_owned();
    let mut seen_cards: HashSet<(VecDeque<u32>, VecDeque<u32>)> = HashSet::new();

    loop {
        let keys = (player_1.to_owned(), player_2.to_owned());

        if seen_cards.contains(&keys) {
            break (1, player_1);
        }

        seen_cards.insert(keys);

        let player_1_card = player_1.pop_front().unwrap();
        let player_2_card = player_2.pop_front().unwrap();

        if player_1.len() >= player_1_card as usize && player_2.len() >= player_2_card as usize {
            let (winner_number, _) = combat(
                &VecDeque::from(
                    player_1.to_owned().as_slices().0[..player_1_card as usize].to_vec(),
                ),
                &VecDeque::from(
                    player_2.to_owned().as_slices().0[..player_2_card as usize].to_vec(),
                ),
            );

            match winner_number {
                1 => {
                    player_1.push_back(player_1_card);
                    player_1.push_back(player_2_card);
                }
                2 => {
                    player_2.push_back(player_2_card);
                    player_2.push_back(player_1_card);
                }
                _ => unreachable!(),
            };
        } else if player_1_card > player_2_card {
            player_1.push_back(player_1_card);
            player_1.push_back(player_2_card);
        } else if player_2_card > player_1_card {
            player_2.push_back(player_2_card);
            player_2.push_back(player_1_card);
        }

        if player_1.is_empty() {
            break (2, player_2);
        }

        if player_2.is_empty() {
            break (1, player_1);
        }
    }
}

/* Part One
 *
 * It only takes a few hours of sailing the ocean on a raft for boredom to sink in.
 * Fortunately, you brought a small deck of space cards!
 * You'd like to play a game of Combat, and there's even an opponent available:
 * a small crab that climbed aboard your raft before you left.
 *
 * Fortunately, it doesn't take long to teach the crab the rules.
 *
 * Before the game starts, split the cards so each player has their own deck
 * (your puzzle input). Then, the game consists of a series of rounds:
 * both players draw their top card, and the player with the higher-valued card wins the round.
 * The winner keeps both cards, placing them on the bottom of their own deck so
 * that the winner's card is above the other card.
 * If this causes a player to have all of the cards, they win, and the game ends.
 *
 * For example, consider the following starting decks:
 *
 * Player 1:
 * 9
 * 2
 * 6
 * 3
 * 1
 *
 * Player 2:
 * 5
 * 8
 * 4
 * 7
 * 10
 *
 * This arrangement means that player 1's deck contains 5 cards, with 9 on top and 1 on the bottom;
 * player 2's deck also contains 5 cards, with 5 on top and 10 on the bottom.
 *
 * The first round begins with both players drawing the top card of their decks: 9 and 5.
 * Player 1 has the higher card, so both cards move to the bottom of player 1's deck such that 9 is above 5.
 * In total, it takes 29 rounds before a player has all of the cards:
 *
 * -- Round 1 --
 * Player 1's deck: 9, 2, 6, 3, 1
 * Player 2's deck: 5, 8, 4, 7, 10
 * Player 1 plays: 9
 * Player 2 plays: 5
 * Player 1 wins the round!
 *
 * -- Round 2 --
 * Player 1's deck: 2, 6, 3, 1, 9, 5
 * Player 2's deck: 8, 4, 7, 10
 * Player 1 plays: 2
 * Player 2 plays: 8
 * Player 2 wins the round!
 *
 * -- Round 3 --
 * Player 1's deck: 6, 3, 1, 9, 5
 * Player 2's deck: 4, 7, 10, 8, 2
 * Player 1 plays: 6
 * Player 2 plays: 4
 * Player 1 wins the round!
 *
 * -- Round 4 --
 * Player 1's deck: 3, 1, 9, 5, 6, 4
 * Player 2's deck: 7, 10, 8, 2
 * Player 1 plays: 3
 * Player 2 plays: 7
 * Player 2 wins the round!
 *
 * -- Round 5 --
 * Player 1's deck: 1, 9, 5, 6, 4
 * Player 2's deck: 10, 8, 2, 7, 3
 * Player 1 plays: 1
 * Player 2 plays: 10
 * Player 2 wins the round!
 *
 * ...several more rounds pass...
 *
 * -- Round 27 --
 * Player 1's deck: 5, 4, 1
 * Player 2's deck: 8, 9, 7, 3, 2, 10, 6
 * Player 1 plays: 5
 * Player 2 plays: 8
 * Player 2 wins the round!
 *
 * -- Round 28 --
 * Player 1's deck: 4, 1
 * Player 2's deck: 9, 7, 3, 2, 10, 6, 8, 5
 * Player 1 plays: 4
 * Player 2 plays: 9
 * Player 2 wins the round!
 *
 * -- Round 29 --
 * Player 1's deck: 1
 * Player 2's deck: 7, 3, 2, 10, 6, 8, 5, 9, 4
 * Player 1 plays: 1
 * Player 2 plays: 7
 * Player 2 wins the round!
 *
 *
 * == Post-game results ==
 * Player 1's deck:
 * Player 2's deck: 3, 2, 10, 6, 8, 5, 9, 4, 7, 1
 *
 * Once the game ends, you can calculate the winning player's score.
 * The bottom card in their deck is worth the value of the card multiplied by 1,
 * the second-from-the-bottom card is worth the value of the card multiplied by 2, and so on.
 * With 10 cards, the top card is worth the value on the card multiplied by 10. In this example, the winning player's score is:
 *
 *    3 * 10
 * +  2 *  9
 * + 10 *  8
 * +  6 *  7
 * +  8 *  6
 * +  5 *  5
 * +  9 *  4
 * +  4 *  3
 * +  7 *  2
 * +  1 *  1
 * = 306
 *
 * So, once the game ends, the winning player's score is 306.
 *
 * Play the small crab in a game of Combat using the two decks you just dealt. What is the winning player's score?
 */
/// Your puzzle answer was
/// ```
/// use advent_of_code_2020::day_22::*;
/// let data = include_str!("../input/2020/day22.txt");
/// assert_eq!(solve_part_01(&input_generator(data)), 32472);
/// ```
#[aoc(day22, part1)]
pub fn solve_part_01((player_1, player_2): &(VecDeque<u32>, VecDeque<u32>)) -> u32 {
    let mut player_1 = player_1.to_owned();
    let mut player_2 = player_2.to_owned();

    loop {
        let player_1_card = player_1.pop_front().unwrap();
        let player_2_card = player_2.pop_front().unwrap();

        if player_1_card > player_2_card {
            player_1.push_back(player_1_card);
            player_1.push_back(player_2_card);
        } else {
            player_2.push_back(player_2_card);
            player_2.push_back(player_1_card);
        }

        if player_1.is_empty() || player_2.is_empty() {
            break;
        }
    }

    match (player_1.is_empty(), player_2.is_empty()) {
        (true, _) => calculate_score(player_2),
        (_, true) => calculate_score(player_1),
        _ => unreachable!(),
    }
}

/* Part Two
 *
 * You lost to the small crab! Fortunately, crabs aren't very good at recursion.
 * To defend your honor as a Raft Captain, you challenge the small crab to a game of Recursive Combat.
 *
 * Recursive Combat still starts by splitting the cards into two decks
 * (you offer to play with the same starting decks as before - it's only fair).
 * Then, the game consists of a series of rounds with a few changes:
 *
 * Before either player deals a card, if there was a previous round in this game that had exactly
 * the same cards in the same order in the same players' decks, the game instantly ends in a win for player 1.
 * Previous rounds from other games are not considered. (This prevents infinite games of Recursive Combat, which everyone agrees is a bad idea.)
 *
 * Otherwise, this round's cards must be in a new configuration; the players
 * begin the round by each drawing the top card of their deck as normal.
 *
 * If both players have at least as many cards remaining in their deck as the value of the card they just drew,
 * the winner of the round is determined by playing a new game of Recursive Combat (see below).
 *
 * Otherwise, at least one player must not have enough cards left in their deck to recurse;
 * the winner of the round is the player with the higher-value card.
 *
 * As in regular Combat, the winner of the round (even if they won the round by winning a sub-game)
 * takes the two cards dealt at the beginning of the round and places them on the bottom of their own deck
 * (again so that the winner's card is above the other card). Note that the winner's card might be the lower-valued
 * of the two cards if they won the round due to winning a sub-game.
 * If collecting cards by winning the round causes a player to have all of the cards, they win, and the game ends.
 *
 * Here is an example of a small game that would loop forever without the infinite game prevention rule:
 *
 * Player 1:
 * 43
 * 19
 *
 * Player 2:
 * 2
 * 29
 * 14
 *
 * During a round of Recursive Combat, if both players have at least as many cards in their
 * own decks as the number on the card they just dealt, the winner of the round is determined by
 * recursing into a sub-game of Recursive Combat. (For example, if player 1 draws the 3 card,
 * and player 2 draws the 7 card, this would occur if player 1 has at least 3 cards left and player
 * 2 has at least 7 cards left, not counting the 3 and 7 cards that were drawn.)
 *
 * To play a sub-game of Recursive Combat, each player creates a new deck by making a copy of the next
 * cards in their deck (the quantity of cards copied is equal to the number on the card they drew to trigger the sub-game).
 * During this sub-game, the game that triggered it is on hold and completely unaffected;
 * no cards are removed from players' decks to form the sub-game. (For example, if player 1 drew the 3 card,
 * their deck in the sub-game would be copies of the next three cards in their deck.)
 *
 * Here is a complete example of gameplay, where Game 1 is the primary game of Recursive Combat:
 *
 * REMOVED LONG LIST OF GAMES
 *
 * == Post-game results ==
 * Player 1's deck:
 * Player 2's deck: 7, 5, 6, 2, 4, 1, 10, 8, 9, 3
 *
 * After the game, the winning player's score is calculated from the cards they have in their original deck using the same rules as regular Combat. In the above game, the winning player's score is 291.
 *
 * Defend your honor as Raft Captain by playing the small crab in a game of Recursive Combat using the same two decks as before. What is the winning player's score?
 */
/// Your puzzle answer was
/// ```
/// use advent_of_code_2020::day_22::*;
/// let data = include_str!("../input/2020/day22.txt");
/// assert_eq!(solve_part_02(&input_generator(data)), 36463);
/// ```
#[aoc(day22, part2)]
pub fn solve_part_02((player_1, player_2): &(VecDeque<u32>, VecDeque<u32>)) -> u32 {
    let (_, winner) = combat(player_1, player_2);

    calculate_score(winner)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_01() {
        let data = "Player 1:
9
2
6
3
1

Player 2:
5
8
4
7
10";

        assert_eq!(solve_part_01(&input_generator(data)), 306);
    }

    #[test]
    fn sample_02() {
        let data = "Player 1:
9
2
6
3
1

Player 2:
5
8
4
7
10";

        assert_eq!(solve_part_02(&input_generator(data)), 291);
    }

    #[test]
    fn infinite_solution() {
        let data = "Player 1:
43
19

Player 2:
2
29
14";

        assert_eq!(solve_part_02(&input_generator(data)), 105);
    }
}
