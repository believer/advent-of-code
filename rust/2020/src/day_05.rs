use crate::common;
use itertools::Itertools;
use std::ops::RangeInclusive;

// Day 5 - Binary Boarding
//
// Later in the day I learned that the boarding pass ID can be expressed
// as a binary number. It was a much cleaner solution, BUT it was a whole lot
// slower in the benchmarks (~1.7 ms vs. ~100 µs). So I'll use my original solution.

#[aoc_generator(day5)]
pub fn input_generator(input: &str) -> Vec<String> {
    common::input_vec(input)
}

fn half_range(range: &RangeInclusive<u32>) -> u32 {
    ((*range.end() as f32 - *range.start() as f32) / 2_f32).round() as u32
}

fn upper_half(range: &RangeInclusive<u32>) -> RangeInclusive<u32> {
    *range.start() + half_range(range)..=*range.end()
}

fn lower_half(range: &RangeInclusive<u32>) -> RangeInclusive<u32> {
    *range.start()..=*range.end() - half_range(range)
}

fn find_my_seat(seat: &str) -> u32 {
    let mut row = 0..=127;
    let mut col = 0..=7;

    for c in seat.chars() {
        match c {
            'F' => row = lower_half(&row),
            'B' => row = upper_half(&row),
            'L' => col = lower_half(&col),
            'R' => col = upper_half(&col),
            _ => (),
        }
    }

    *row.end() * 8 + *col.end()
}

fn find_airplane_seats(input: &[String]) -> Vec<u32> {
    input.iter().map(|seat| find_my_seat(seat)).collect()
}

/* Part One
 *
 * You board your plane only to discover a new problem: you dropped your boarding pass!
 * You aren't sure which seat is yours, and all of the flight attendants are busy with the
 * flood of people that suddenly made it through passport control.
 *
 * You write a quick program to use your phone's camera to scan all of the nearby
 * boarding passes (your puzzle input); perhaps you can find your seat through process of elimination.
 *
 * Instead of zones or groups, this airline uses binary space partitioning to seat people.
 * A seat might be specified like FBFBBFFRLR, where F means "front", B means "back", L means "left", and R means "right".
 *
 * The first 7 characters will either be F or B; these specify exactly one of the 128 rows on the
 * plane (numbered 0 through 127). Each letter tells you which half of a region the given seat is in.
 * Start with the whole list of rows; the first letter indicates whether the seat is in the
 * front (0 through 63) or the back (64 through 127). The next letter indicates which half of that region the seat is in,
 * and so on until you're left with exactly one row.
 *
 * For example, consider just the first seven characters of FBFBBFFRLR:
 *
 * Start by considering the whole range, rows 0 through 127.
 * F means to take the lower half, keeping rows 0 through 63.
 * B means to take the upper half, keeping rows 32 through 63.
 * F means to take the lower half, keeping rows 32 through 47.
 * B means to take the upper half, keeping rows 40 through 47.
 * B keeps rows 44 through 47.
 * F keeps rows 44 through 45.
 * The final F keeps the lower of the two, row 44.
 *
 * The last three characters will be either L or R; these specify exactly one of the
 * 8 columns of seats on the plane (numbered 0 through 7). The same process as above proceeds again,
 * this time with only three steps. L means to keep the lower half, while R means to keep the upper half.
 *
 * For example, consider just the last 3 characters of FBFBBFFRLR:
 *
 * Start by considering the whole range, columns 0 through 7.
 * R means to take the upper half, keeping columns 4 through 7.
 * L means to take the lower half, keeping columns 4 through 5.
 * The final R keeps the upper of the two, column 5.
 * So, decoding FBFBBFFRLR reveals that it is the seat at row 44, column 5.
 *
 * Every seat also has a unique seat ID: multiply the row by 8, then add the column.
 * In this example, the seat has ID 44 * 8 + 5 = 357.
 *
 * Here are some other boarding passes:
 *
 * BFFFBBFRRR: row 70, column 7, seat ID 567.
 * FFFBBBFRRR: row 14, column 7, seat ID 119.
 * BBFFBBFRLL: row 102, column 4, seat ID 820.
 * As a sanity check, look through your list of boarding passes. What is the highest seat ID on a boarding pass?
 */
///your puzzle answer was.
/// ```
/// use advent_of_code_2020::day_05::*;
/// let input = include_str!("../input/2020/day5.txt");
/// assert_eq!(solve_part_01(&input_generator(input)), 866);
/// ```
#[aoc(day5, part1)]
pub fn solve_part_01(input: &[String]) -> u32 {
    find_airplane_seats(input).into_iter().max().unwrap()
}

/* Part Two
 *
 * Ding! The "fasten seat belt" signs have turned on. Time to find your seat.
 *
 * It's a completely full flight, so your seat should be the only missing boarding pass in your list.
 * However, there's a catch: some of the seats at the very front and back of the plane don't exist on this aircraft,
 * so they'll be missing from your list as well.
 *
 * Your seat wasn't at the very front or back, though; the seats with IDs +1 and -1 from yours will be in your list.
 *
 * What is the ID of your seat?
*/
///your puzzle answer was.
/// ```
/// use advent_of_code_2020::day_05::*;
/// let input = include_str!("../input/2020/day5.txt");
/// assert_eq!(solve_part_02(&input_generator(input)), 583);
/// ```
#[aoc(day5, part2)]
pub fn solve_part_02(input: &[String]) -> u32 {
    let mut my_seat = 0;
    let seats: Vec<u32> = find_airplane_seats(input).into_iter().sorted().collect();

    for (i, seat_id) in seats.iter().enumerate() {
        if seat_id + 1 != seats[i + 1] {
            my_seat = seat_id + 1;
            break;
        }
    }

    my_seat
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Test example data on part 1
    #[test]
    fn sample_01() {
        let data = "FBFBBFFRLR";

        assert_eq!(solve_part_01(&input_generator(data)), 357)
    }
}
