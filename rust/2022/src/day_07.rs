use std::collections::HashMap;
use std::path::Path;
use std::{env, fs};

// Day 7 - No Space Left On Device
//
// I didn't have a good idea of how to build a tree structure, but the input is
// basically shell commands. I modified the input manually to have it create
// the file tree.
//
// Convert commands to shell commands
// $ cd .. -> cd ..
// $ ls -> ls
//
// dir lines creates a folder
// dir abc -> mkdir abc
//
// file lines creates empty files of the specified size
// using mkfile
// 123456 test.txt -> mkfile -n 123456 test.txt
//
// To make this more generic, parse through the input and create a shell file automatically.
// Run the file to create the tree and calculate the solutions.
//
// Remove the temporary directory at the end.

type Input = Vec<String>;

#[aoc_generator(day7)]
pub fn input_generator(input: &str) -> Input {
    input.lines().map(|l| l.to_string()).collect()
}

fn recurse(root: &Path, current_root: &Path, sizes: &mut HashMap<String, u64>) {
    env::set_current_dir(current_root).unwrap();
    let current_dir = env::current_dir().unwrap();

    for entry in fs::read_dir(current_dir).unwrap() {
        match entry {
            Ok(dir) => {
                let metadata = fs::metadata(dir.path()).unwrap();

                if metadata.is_dir() {
                    recurse(root, &dir.path(), sizes);
                }

                if metadata.is_file() {
                    let mut parent = dir.path().parent().unwrap().to_str().unwrap().to_string();

                    while root.display().to_string() != parent {
                        let size = sizes.entry(parent.clone()).or_insert(0);
                        *size += metadata.len() as u64;
                        parent = Path::new(&parent)
                            .parent()
                            .unwrap()
                            .to_str()
                            .unwrap()
                            .to_string();
                    }
                }
            }
            Err(_) => (),
        }
    }
}

/* Part One
*/
/// Your puzzle answer was
/// ```
/// use advent_of_code_2022::day_07::*;
/// let data = include_str!("../input/2022/day7.txt");
/// assert_eq!(solve_part_01(&input_generator(data)), 0);
/// ```
#[aoc(day7, part1)]
pub fn solve_part_01(_input: &Input) -> u64 {
    let mut solution = 0;
    let mut sizes: HashMap<String, u64> = HashMap::new();

    let start_dir = env::current_dir().unwrap();
    env::set_current_dir("test_data").unwrap();

    recurse(&start_dir, &env::current_dir().unwrap(), &mut sizes);

    for &size in sizes.values() {
        if size < 100000 {
            solution += size;
        }
    }

    solution
}

/* Part Two
*/
/// Your puzzle answer was
/// ```
/// use advent_of_code_2022::day_07::*;
/// let data = include_str!("../input/2022/day7.txt");
/// assert_eq!(solve_part_02(&input_generator(data)), 0);
/// ```
#[aoc(day7, part2)]
pub fn solve_part_02(_input: &Input) -> u64 {
    let total = 70000000;
    let least_unused = 30000000;

    let mut solution = 0;
    let mut sizes: HashMap<String, u64> = HashMap::new();

    let start_dir = env::current_dir().unwrap();
    env::set_current_dir("test").unwrap();

    // Save the root dir
    let root_dir = env::current_dir().unwrap();

    recurse(&start_dir, &env::current_dir().unwrap(), &mut sizes);
    let root_size = sizes.get(&root_dir.display().to_string()).unwrap();

    let unused = total - root_size;

    let mut all_sizes = sizes.values().collect::<Vec<&u64>>();
    all_sizes.sort();

    for size in all_sizes {
        if size + unused > least_unused {
            solution = *size;
            break;
        }
    }

    solution
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_01() {
        let data = "$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k";

        assert_eq!(solve_part_02(&input_generator(data)), 95437)
    }
}
