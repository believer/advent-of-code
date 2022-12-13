use std::{cmp::Ordering, iter::Peekable, str::Bytes};

// Day 13 - Distress Signal
//
// I had no idea how to parse the input, but found some help that suggested
// using a peekable iterator and it worked perfectly. They also suggested looking
// into implementing Ord for the data to make sorting easier. Made for a clean
// solution.
//
// I think I've seen something similar to the lists before? I should remember this
// solution for the future.

type Packets = Vec<Packet>;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Packet {
    Integer(u8),
    List(Vec<Packet>),
}

impl Packet {
    fn parse(input: &str) -> Self {
        parse_packet_parts(&mut input[1..].bytes().peekable())
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

// Parse the string to a Packet. Use a peekable iterator to look ahead
// to the next character. This is used to determine if the current
// character is the start of a list or an integer.
fn parse_packet_parts(parts: &mut Peekable<Bytes>) -> Packet {
    let mut packets: Vec<Packet> = Vec::new();

    while let Some(p) = parts.next() {
        match p {
            // Found a number
            b'0'..=b'9' => {
                let mut number = String::new();
                number.push(p as char);

                // Include numbers that are more than one digit
                while let Some(b'0'..=b'9') = parts.peek() {
                    number.push(parts.next().unwrap() as char);
                }

                packets.push(Packet::Integer(number.parse().unwrap()));
            }

            // Found a nested list, recurse
            b'[' => packets.push(parse_packet_parts(parts)),

            // Found the end of the list, return our list packet
            b']' => return Packet::List(packets),

            // Comma is the list separator, so we can just ignore it
            b',' => continue,

            // Something was missed, panic!
            b => panic!("Unknown byte: {b}"),
        }
    }

    unreachable!("Unfinished packet");
}

#[aoc_generator(day13)]
pub fn input_generator(input: &str) -> Packets {
    input.split_whitespace().map(Packet::parse).collect()
}

/* Part One
*/
/// Your puzzle answer was
/// ```
/// use advent_of_code_2022::day_13::*;
/// let data = include_str!("../input/2022/day13.txt");
/// assert_eq!(solve_part_01(&input_generator(data)), 5390);
/// ```
#[aoc(day13, part1)]
pub fn solve_part_01(packets: &Packets) -> usize {
    packets
        .iter()
        // Split the packets into pairs
        .array_chunks::<2>()
        .enumerate()
        // Compare the pairs using our Ord implementation
        // Return the index + 1 when packets are in the correct order
        .filter_map(|(i, [left, right])| match left.cmp(right) {
            Ordering::Less => Some(i + 1),
            _ => None,
        })
        .sum()
}

/* Part Two
*/
/// Your puzzle answer was
/// ```
/// use advent_of_code_2022::day_13::*;
/// let data = include_str!("../input/2022/day13.txt");
/// assert_eq!(solve_part_02(&input_generator(data)), 19261);
/// ```
#[aoc(day13, part2)]
pub fn solve_part_02(packets: &Packets) -> usize {
    // Clone to solve mutability issues
    let mut packets = packets.clone();

    packets.sort_unstable();

    // Find where to put the divider packets, [[2]] and [[6]]
    // The new packets won't be found, but using unwrap_err we get the index
    // where the packets would be inserted. Add 1 for the first divider and
    // 2 for the second divider
    let i = packets.binary_search(&Packet::parse("[[2]]")).unwrap_err() + 1;
    let j = packets.binary_search(&Packet::parse("[[6]]")).unwrap_err() + 2;

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
