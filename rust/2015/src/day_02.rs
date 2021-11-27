// Day 2: I Was Told There Would Be No Math

#[aoc_generator(day2)]
pub fn input_generator(input: &str) -> Vec<(u32, u32, u32)> {
    input
        .lines()
        .map(|l| l.trim())
        .filter(|l| !l.is_empty())
        .map(|l| {
            let nums = l.split('x').collect::<Vec<_>>();
            let numbers = nums
                .iter()
                .map(|v| v.parse::<u32>().unwrap())
                .collect::<Vec<_>>();

            (numbers[0], numbers[1], numbers[2])
        })
        .collect()
}

/* Part One

The elves are running low on wrapping paper, and so they need to submit an order for more. They have a list of the dimensions (length l, width w, and height h) of each present, and only want to order exactly as much as they need.

Fortunately, every present is a box (a perfect right rectangular prism), which makes calculating the required wrapping paper for each gift a little easier: find the surface area of the box, which is 2*l*w + 2*w*h + 2*h*l. The elves also need a little extra paper for each present: the area of the smallest side.

For example:

A present with dimensions 2x3x4 requires 2*6 + 2*12 + 2*8 = 52 square feet of wrapping paper plus 6 square feet of slack, for a total of 58 square feet.
A present with dimensions 1x1x10 requires 2*1 + 2*10 + 2*10 = 42 square feet of wrapping paper plus 1 square foot of slack, for a total of 43 square feet.
All numbers in the elves' list are in feet. How many total square feet of wrapping paper should they order?

*/
// Your puzzle answer was
/// ```
/// use advent_of_code_2015::day_02::*;
/// let data = include_str!("../input/2015/day2.txt");
/// assert_eq!(solve_part_01(&input_generator(data)), 1586300);
/// ```
#[aoc(day2, part1)]
pub fn solve_part_01(input: &Vec<(u32, u32, u32)>) -> u32 {
    // 2*l*w + 2*w*h + 2*h*l
    let mut total_paper = 0;

    for (length, width, height) in input {
        let first_side = length * width;
        let second_side = width * height;
        let third_side = height * length;
        let mut all_sides = [first_side, second_side, third_side];
        all_sides.sort();

        total_paper += 2 * first_side + 2 * second_side + 2 * third_side + all_sides[0];
    }

    total_paper
}
/* Part Two
The elves are also running low on ribbon. Ribbon is all the same width, so they only have to worry about the length they need to order, which they would again like to be exact.

The ribbon required to wrap a present is the shortest distance around its sides, or the smallest perimeter of any one face. Each present also requires a bow made out of ribbon as well; the feet of ribbon required for the perfect bow is equal to the cubic feet of volume of the present. Don't ask how they tie the bow, though; they'll never tell.

For example:

A present with dimensions 2x3x4 requires 2+2+3+3 = 10 feet of ribbon to wrap the present plus 2*3*4 = 24 feet of ribbon for the bow, for a total of 34 feet.
A present with dimensions 1x1x10 requires 1+1+1+1 = 4 feet of ribbon to wrap the present plus 1*1*10 = 10 feet of ribbon for the bow, for a total of 14 feet.
How many total feet of ribbon should they order?
 */
// Your puzzle answer was
/// ```
/// use advent_of_code_2015::day_02::*;
/// let data = include_str!("../input/2015/day2.txt");
/// assert_eq!(solve_part_02(&input_generator(data)), 3737498);
/// ```
#[aoc(day2, part2)]
pub fn solve_part_02(input: &Vec<(u32, u32, u32)>) -> u32 {
    let mut total_ribbon = 0;

    for (length, width, height) in input {
        let mut all_lengths = [length, width, height];
        all_lengths.sort();

        total_ribbon += 2 * all_lengths[0] + 2 * all_lengths[1] + (length * width * height)
    }

    total_ribbon
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_01() {
        let data = "2x3x4";

        assert_eq!(solve_part_01(&input_generator(data)), 58)
    }

    #[test]
    fn sample_01_example_2() {
        let data = "1x1x10";

        assert_eq!(solve_part_01(&input_generator(data)), 43)
    }

    #[test]
    fn sample_02() {
        let data = "2x3x4";

        assert_eq!(solve_part_02(&input_generator(data)), 34)
    }

    #[test]
    fn sample_02_example_2() {
        let data = "1x1x10";

        assert_eq!(solve_part_02(&input_generator(data)), 14)
    }
}
