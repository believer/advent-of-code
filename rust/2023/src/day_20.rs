//! Day 20: Pulse Propagation
//!
//! Hard to follow and my data structures make it even harder haha. Seems
//! like a bunch of people had problems with this one. Might come back and
//! simplify it later. Again HyperNeutrino's solution made it clearer to
//! understand what should be done, but it seems like most people need
//! to make some assumptions about the input to get it to work.

use std::collections::{HashMap, VecDeque};

use crate::math;

#[derive(Debug)]
pub struct Input {
    modules: HashMap<String, Module>,
    broadcast_target: Vec<String>,
}

#[derive(Debug, Clone, PartialEq)]
enum ModuleType {
    FlipFlop,
    Conjuction,
}

#[derive(Debug, Clone, PartialEq)]
enum Memory {
    On,
    Off,
    Low,
    High,
    Map(HashMap<String, Memory>),
}

impl From<char> for ModuleType {
    fn from(c: char) -> Self {
        match c {
            '%' => Self::FlipFlop,
            '&' => Self::Conjuction,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Module {
    name: String,
    module_type: ModuleType,
    outputs: Vec<String>,
    memory: Memory,
}

impl Module {
    fn new(name: String, module_type: ModuleType, outputs: Vec<String>) -> Self {
        let memory = match module_type {
            ModuleType::FlipFlop => Memory::Off,
            ModuleType::Conjuction => Memory::Map(HashMap::new()),
        };

        Self {
            name,
            module_type,
            outputs,
            memory,
        }
    }
}

#[aoc_generator(day20)]
pub fn input_generator(input: &str) -> Input {
    let mut modules: HashMap<String, Module> = HashMap::new();
    let mut broadcast_target = vec![];

    for line in input.lines() {
        let (module, outputs) = line.split_once(" -> ").unwrap();
        let outputs = outputs
            .split(',')
            .map(|s| s.trim().to_string())
            .collect::<Vec<_>>();

        if module == "broadcaster" {
            broadcast_target = outputs;
        } else {
            let module_type = module.chars().next().unwrap();
            let name = module[1..].to_string();

            modules.insert(name.clone(), Module::new(name, module_type.into(), outputs));
        }
    }

    // Setup initial memory
    for (name, module) in modules.clone().iter() {
        for output in module.outputs.iter() {
            if let Some(m) = modules.get_mut(output) {
                if let ModuleType::Conjuction = m.module_type {
                    if let Memory::Map(ref mut map) = m.memory {
                        map.insert(name.clone(), Memory::Low);
                    }
                }
            }
        }
    }

    Input {
        modules,
        broadcast_target,
    }
}

/* Part One
*
*/
// Your puzzle answer was
#[doc = r#"```
use advent_of_code_2023::day_20::*;
let data = include_str!("../input/2023/day20.txt");
assert_eq!(solve_part_01(&input_generator(data)), 812609846);
```"#]
#[aoc(day20, part1)]
pub fn solve_part_01(input: &Input) -> u64 {
    let mut modules = input.modules.clone();
    let mut low_pulses = 0;
    let mut high_pulses = 0;

    for _ in 0..1000 {
        // A low pulse from the button press is sent each time
        low_pulses += 1;
        let mut queue = VecDeque::new();

        for target in input.broadcast_target.iter() {
            queue.push_back(("broadcaster".to_string(), target.clone(), Memory::Low));
        }

        while let Some((from, to, pulse)) = queue.pop_front() {
            match pulse {
                Memory::Low => low_pulses += 1,
                Memory::High => high_pulses += 1,
                _ => (),
            };

            // Handles unknown modules
            if !modules.contains_key(&to) {
                continue;
            };

            let module = modules.get_mut(&to).unwrap();

            match module.module_type {
                ModuleType::FlipFlop => {
                    if pulse == Memory::Low {
                        module.memory = if module.memory == Memory::Off {
                            Memory::On
                        } else {
                            Memory::Off
                        };
                        let next_pulse = if module.memory == Memory::On {
                            Memory::High
                        } else {
                            Memory::Low
                        };

                        for output in module.outputs.iter() {
                            queue.push_back((
                                module.name.clone(),
                                output.clone(),
                                next_pulse.clone(),
                            ));
                        }
                    }
                }

                ModuleType::Conjuction => {
                    if let Memory::Map(ref mut map) = module.memory {
                        map.insert(from, pulse.clone());

                        let next_pulse = if map.values().all(|x| *x == Memory::High) {
                            Memory::Low
                        } else {
                            Memory::High
                        };

                        for output in module.outputs.iter() {
                            queue.push_back((
                                module.name.clone(),
                                output.clone(),
                                next_pulse.clone(),
                            ));
                        }
                    }
                }
            }
        }
    }

    low_pulses * high_pulses
}

/* Part Two
*
* Some assumptions are made about the input to get this to work. We find
* that the module that feeds into "rx" is "hp" and that "hp" is fed by
* four other modules. "hp" is a conjuction module which means that it will
* only send a low pulse if all of its inputs are high. So, we can
* find the cycle length of each of the modules that feed into "hp" and
* then calculate the LCM of those cycle lengths.
*
*/
#[doc = r#"```
use advent_of_code_2023::day_20::*;
let data = include_str!("../input/2023/day20.txt");
assert_eq!(solve_part_02(&input_generator(data)), 245114020323037);
```"#]
#[aoc(day20, part2)]
pub fn solve_part_02(input: &Input) -> u64 {
    let mut modules = input.modules.clone();
    let mut button_presses = 0;

    // Find which module feeds into "rx". This should only be one.
    // This is "hp" in my input.
    let feed = input
        .modules
        .values()
        .find(|module| module.outputs.contains(&"rx".to_string()))
        .unwrap();

    // Find the modules that feed into the module, "hp", that feeds into "rx".
    // We'll use these to find the cycle length and then we can calculate the
    // LCM of the cycle lengths.
    let mut cycle_lengths: HashMap<String, u64> = HashMap::new();

    let mut seen = input
        .modules
        .values()
        .filter(|module| module.outputs.contains(&feed.name))
        .map(|module| (module.name.clone(), 0))
        .collect::<HashMap<String, u64>>();

    let fewest_button_presses = 'outer: loop {
        button_presses += 1;

        let mut queue = VecDeque::new();

        for target in input.broadcast_target.iter() {
            queue.push_back(("broadcaster".to_string(), target.clone(), Memory::Low));
        }

        while let Some((from, to, pulse)) = queue.pop_front() {
            // Handles unknown modules
            if !modules.contains_key(&to) {
                continue;
            };

            let module = modules.get_mut(&to).unwrap();

            // We only care about the module that feeds into "hp"
            // and we only care about high pulses
            if module.name == feed.name && pulse == Memory::High {
                seen.entry(from.clone()).and_modify(|x| *x += 1);

                // Update the cycle length
                if !cycle_lengths.contains_key(&from) {
                    cycle_lengths.insert(from.clone(), button_presses);
                }

                // We've seen all the modules that feed into "hp"
                // Calculate the LCM of the cycle lengths and break
                if seen.values().all(|x| *x == 1) {
                    break 'outer cycle_lengths
                        .values()
                        .fold(1, |acc, x| math::lcm(acc, *x as i64));
                }
            }

            match module.module_type {
                ModuleType::FlipFlop => {
                    if pulse == Memory::Low {
                        module.memory = if module.memory == Memory::Off {
                            Memory::On
                        } else {
                            Memory::Off
                        };
                        let next_pulse = if module.memory == Memory::On {
                            Memory::High
                        } else {
                            Memory::Low
                        };

                        for output in module.outputs.iter() {
                            queue.push_back((
                                module.name.clone(),
                                output.clone(),
                                next_pulse.clone(),
                            ));
                        }
                    }
                }

                ModuleType::Conjuction => {
                    if let Memory::Map(ref mut map) = module.memory {
                        map.insert(from, pulse.clone());

                        let next_pulse = if map.values().all(|x| *x == Memory::High) {
                            Memory::Low
                        } else {
                            Memory::High
                        };

                        for output in module.outputs.iter() {
                            queue.push_back((
                                module.name.clone(),
                                output.clone(),
                                next_pulse.clone(),
                            ));
                        }
                    }
                }
            }
        }
    };

    fewest_button_presses as u64
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(
        "broadcaster -> a, b, c
%a -> b
%b -> c
%c -> inv
&inv -> a",
        32_000_000
    )]
    #[case(
        "broadcaster -> a
%a -> inv, con
&inv -> b
%b -> con
&con -> output",
        11687500
    )]
    fn sample_01(#[case] input: &str, #[case] expected: u64) {
        assert_eq!(solve_part_01(&input_generator(input)), expected);
    }
}
