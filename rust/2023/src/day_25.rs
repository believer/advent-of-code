//! Day 25: Snowverload
//!
//! I thought this was going to be easier while looking at the description, but
//! it turned out to be a bit more complicated. I looked for different solutions
//! using petgraph, but I couldn't find anything that worked for me. I then found
//! the Louvain algorithm that detects communities in a graph. This was also
//! available in the small graphrs crate, so I decided to use that.

use graphrs::{algorithms::community::louvain, Edge, Graph, GraphSpecs};

pub struct Input {
    graph: Graph<String, ()>,
}

#[aoc_generator(day25)]
pub fn input_generator(input: &str) -> Input {
    let mut graph = Graph::<String, ()>::new(GraphSpecs::undirected_create_missing());

    for line in input.lines() {
        let (name, right) = line.split_once(": ").unwrap();

        for node in right.split_whitespace() {
            graph
                .add_edge(Edge::new(name.to_string(), node.to_string()))
                .unwrap();
        }
    }

    Input { graph }
}

/* Part One
*
*/
// Your puzzle answer was
#[doc = r#"```
use advent_of_code_2023::day_25::*;
let data = include_str!("../input/2023/day25.txt");
assert_eq!(solve_part_01(&input_generator(data)), 548960);
```"#]
#[aoc(day25, part1)]
pub fn solve_part_01(input: &Input) -> usize {
    let partitions =
        louvain::louvain_partitions(&input.graph, false, Some(0.0), Some(4.0), None).unwrap();

    // We have one result with two partitions
    assert_eq!(partitions.len(), 1);
    assert_eq!(partitions[0].len(), 2);

    partitions[0].iter().map(|n| n.len()).product()
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(
        "jqt: rhn xhk nvd
rsh: frs pzl lsr
xhk: hfx
cmg: qnr nvd lhk bvb
rhn: xhk bvb hfx
bvb: xhk hfx
pzl: lsr hfx nvd
qnr: nvd
ntq: jqt hfx bvb xhk
nvd: lhk
lsr: lhk
rzs: qnr cmg lsr rsh
frs: qnr lhk lsr",
        54
    )]
    fn sample_01(#[case] input: &str, #[case] expected: usize) {
        assert_eq!(solve_part_01(&input_generator(input)), expected);
    }
}
