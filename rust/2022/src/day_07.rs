use std::collections::HashMap;
use std::path::Path;
use std::process::Command;
use std::{env, fs};

// Day 7 - No Space Left On Device

// I didn't have a good idea of how to build a tree structure, but the input is
// basically shell commands. To get the answers, I modified the input manually
// to create the file tree.
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
//
// Took me way too long to remember that the tests are running in parallel
// and both are trying to create/remove the same directory.

type Input = HashMap<String, u64>;

fn find_folder_sizes(root: &Path, current_root: &Path, sizes: &mut HashMap<String, u64>) {
    // Update the current root
    env::set_current_dir(current_root).unwrap();
    let current_dir = env::current_dir().unwrap();

    for entry in fs::read_dir(current_dir).unwrap() {
        match entry {
            Ok(dir) => {
                let metadata = fs::metadata(dir.path()).unwrap();

                // If it's a directory, recurse
                if metadata.is_dir() {
                    find_folder_sizes(root, &dir.path(), sizes);
                }

                // If it's a file, add the size to the folder.
                // Also add the size to parent folders
                if metadata.is_file() {
                    let mut parent = dir.path().parent().unwrap().to_str().unwrap().to_string();

                    // Add size to parent folders until we reach the root
                    // Remove trailing slash from temp dir
                    while !parent.ends_with(env::temp_dir().to_str().unwrap().trim_end_matches('/'))
                    {
                        let size = sizes
                            .entry(
                                parent
                                    .replace("/private", "")
                                    .replace(root.to_str().unwrap(), "")
                                    .clone(),
                            )
                            .or_insert(0);

                        *size += metadata.len();
                        parent = Path::new(&parent)
                            .parent()
                            .unwrap()
                            .to_str()
                            .unwrap()
                            .to_string();
                    }
                }
            }
            Err(err) => panic!("{err:?}"),
        }
    }
}

#[aoc_generator(day7)]
pub fn input_generator(input: &str) -> Input {
    let mut sizes: HashMap<String, u64> = HashMap::new();

    env::set_current_dir(env::temp_dir()).unwrap();

    // No need to create the tree structure again for
    // the second part
    match Path::new("aoc").try_exists() {
        Ok(true) => {
            env::set_current_dir("aoc").unwrap();
        }
        Ok(false) => {
            fs::create_dir("aoc").unwrap();

            // Modify the input to valid shell commands
            let sh = input
                .lines()
                .map(|l| l.to_string())
                .map(|l| {
                    if l == "$ cd /" {
                        "cd aoc".to_string()
                    } else if l.starts_with("$ ") {
                        l.replace("$ ", "")
                    } else if l.starts_with("dir ") {
                        l.replace("dir ", "mkdir ")
                    } else {
                        let parts: Vec<&str> = l.split_whitespace().collect();
                        format!("mkfile -n {} {}", parts[0], parts[1])
                    }
                })
                .collect::<Vec<_>>()
                .join("\n");

            let file_name = "day_07.sh";

            // Create shell file
            fs::write(file_name, sh).unwrap();

            // Run the shell script
            Command::new("sh").arg(file_name).output().unwrap();

            env::set_current_dir("aoc").unwrap();
        }
        Err(err) => panic!("{err:?}"),
    }

    find_folder_sizes(&env::temp_dir(), &env::current_dir().unwrap(), &mut sizes);

    // Return the temporary directory
    sizes
}

// Part One
//
// Here we need to find the sum of all folders smaller than 100,000 bytes
#[aoc(day7, part1)]
pub fn solve_part_01(sizes: &Input) -> u64 {
    // Sum folders that are smaller that 100000 bytes
    sizes.values().filter(|size| **size < 100000).sum()
}

// Part Two
//
// Here we need to find the smallest folder we can delete to get
// at least 30,000,000 bytes of free space
#[aoc(day7, part2)]
pub fn solve_part_02(sizes: &Input) -> u64 {
    let total = 70_000_000;
    let least_unused = 30_000_000;
    let mut solution = 0;

    // Get size of the root folder, i.e. the total amount of space used
    let root_size = sizes.get("aoc").unwrap();

    // Calculate unused space
    let unused = total - root_size;

    // Find the smallest folder that, when deleted, will leave at
    // least 30,000,000 bytes of space
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
    use serial_test::serial;

    const SAMPLE: &str = "$ cd /
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

    #[test]
    #[ignore]
    #[serial]
    fn sample_01() {
        assert_eq!(solve_part_01(&input_generator(SAMPLE)), 95437);
    }

    #[test]
    #[ignore]
    #[serial]
    fn sample_02() {
        assert_eq!(solve_part_02(&input_generator(SAMPLE)), 24933642);
    }
}
