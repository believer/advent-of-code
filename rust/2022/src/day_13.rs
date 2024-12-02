use std::cmp::Ordering;

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::newline,
    multi::{separated_list0, separated_list1},
    sequence::{delimited, separated_pair},
    IResult, Parser,
};

// Day 13 - Distress Signal
//
// I had no idea how to parse the input, but found some help that suggested
// using a peekable iterator and it worked perfectly. They also suggested looking
// into implementing Ord for the data to make sorting easier. Made for a clean
// solution.
//
// I think I've seen something similar to the lists before? I should remember this
// solution for the future.
//
// I've been wanting to try nom to parse inputs, but it's quite hard to get started.
// After I've completed today I decided to try it out as a refactor. Turned out quite nicely.
// Powerful, but not easy. I decided to keep it as it also gave a faster solution for part 2
// AND a faster generator.

type Packets = Vec<Pair>;

#[derive(Debug)]
pub struct Pair {
    left: Packet,
    right: Packet,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Packet {
    Integer(u8),
    List(Vec<Packet>),
}

impl Packet {
    fn parse(input: &str) -> Self {
        packet(input).unwrap().1
    }
}

// Needed to implement Ord for Packet
impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

// Compare the different packets and return ordering
impl Ord for Packet {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            // If both values are integers, the lower integer should come first
            (Packet::Integer(l), Packet::Integer(r)) => l.cmp(r),

            // If both values are lists, compare them number by number. If the left
            // list runs out of number, then the numbers are in the correct order.
            (Packet::List(l), Packet::List(r)) => l.cmp(r),

            // If one value is an integer, convert it to a list and compare
            (Packet::Integer(l), Packet::List(r)) => vec![Packet::Integer(*l)].cmp(r),
            (Packet::List(l), Packet::Integer(r)) => l.cmp(&vec![Packet::Integer(*r)]),
        }
    }
}

fn packet(input: &str) -> IResult<&str, Packet> {
    // Parse a packet, which is either an integer or a list
    alt((
        // Lists can be recursive, so we need to call packet again to find
        // out what we need to do with it
        delimited(tag("["), separated_list0(tag(","), packet), tag("]")).map(Packet::List),
        // Parse integers
        nom::character::complete::u8.map(Packet::Integer),
    ))(input)
}

fn parse_pairs(input: &str) -> IResult<&str, Vec<Pair>> {
    // Split the list by double newlines, i.e., each pair.
    // Use separated_pair to parse both packets which are separated by a newline.
    // Map the left and right packet to our Pair.
    separated_list1(
        tag("\n\n"),
        separated_pair(packet, newline, packet).map(|(left, right)| Pair { left, right }),
    )(input)
}

#[aoc_generator(day13)]
pub fn input_generator(input: &str) -> Packets {
    let (_, pairs) = parse_pairs(input).unwrap();

    pairs
}

/* Part One
*/
#[aoc(day13, part1)]
pub fn solve_part_01(packets: &Packets) -> usize {
    packets
        .iter()
        .enumerate()
        // Compare the pairs using our Ord implementation
        // Return the index + 1 when packets are in the correct order
        .filter_map(|(i, Pair { left, right })| match left.cmp(right) {
            Ordering::Less => Some(i + 1),
            _ => None,
        })
        .sum()
}

/* Part Two
*/
#[aoc(day13, part2)]
pub fn solve_part_02(packets: &Packets) -> usize {
    let mut packets: Vec<&Packet> = packets
        .iter()
        .flat_map(|Pair { left, right }| [left, right])
        .collect();

    packets.sort_unstable();

    // Find where to put the divider packets, [[2]] and [[6]]
    // The new packets won't be found, but using unwrap_err we get the index
    // where the packets would be inserted. Add 1 for the first divider and
    // 2 for the second divider
    let i = packets.binary_search(&&Packet::parse("[[2]]")).unwrap_err() + 1;
    let j = packets.binary_search(&&Packet::parse("[[6]]")).unwrap_err() + 2;

    i * j
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]";

    #[test]
    fn sample_01() {
        assert_eq!(solve_part_01(&input_generator(SAMPLE)), 13)
    }

    #[test]
    fn sample_02() {
        assert_eq!(solve_part_02(&input_generator(SAMPLE)), 140)
    }
}
