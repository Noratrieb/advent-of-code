use std::collections::HashMap;

use helper::{parse_unwrap, Day, IteratorExt, Variants};

pub fn main() {
    helper::main::<Day05>(include_str!("../input.txt"));
}

struct Day05;

helper::define_variants! {
    day => crate::Day05;
    part1 {
        basic => crate::part1;
    }
    part2 {
        basic => crate::part2;
    }
}

impl Day for Day05 {
    fn part1() -> Variants {
        part1_variants!(construct_variants)
    }

    fn part2() -> Variants {
        part2_variants!(construct_variants)
    }
}

fn part1(input: &str) -> u64 {
    let mut rules = Vec::new();
    let mut updates = Vec::new();
    let mut lines = input.lines();
    while let Some(line) = lines.next() {
        if line.is_empty() {
            break;
        }
        let values = line
            .split('|')
            .collect_array::<2>()
            .unwrap()
            .map(parse_unwrap);
        rules.push((values[0], values[1]));
    }
    while let Some(line) = lines.next() {
        let numbers = line.split(",").map(parse_unwrap).collect::<Vec<_>>();
        updates.push(numbers);
    }

    fn build_nodes(
        rules: impl Iterator<Item = (u64, u64)>,
    ) -> (HashMap<u64, usize>, Vec<u64>, Vec<Vec<usize>>) {
        let mut nodes_lookup = HashMap::new();
        let mut nodes = Vec::new();
        let mut edges = Vec::<Vec<_>>::new();
        for (first, then) in rules {
            let first = *nodes_lookup.entry(first).or_insert_with(|| {
                nodes.push(first);
                edges.push(Vec::new());
                nodes.len() - 1
            });
            let then = *nodes_lookup.entry(then).or_insert_with(|| {
                nodes.push(then);
                edges.push(Vec::new());
                nodes.len() - 1
            });
            if !edges[first].contains(&then) {
                edges[first].push(then);
            }
        }

        assert_eq!(nodes_lookup.len(), nodes.len());
        assert_eq!(nodes.len(), edges.len());
        (nodes_lookup, nodes, edges)
    }

    /*
    let mut all_sorted = Vec::new();
    let mut marked = vec![false; nodes.len()];
    let mut worklist = VecDeque::new();

    while let Some((i, _)) = marked
        .iter()
        .enumerate()
        .find(|(_, is_marked)| !**is_marked)
    {
        let mut temporary_ordering = Vec::new();
        worklist.clear();
        worklist.push_back(i);

        while let Some(item) = worklist.pop_front() {
            if marked[item] {
                continue;
            }
            temporary_ordering.push(nodes[item]);
            for child in edges[item].iter().rev() {
                worklist.push_front(*child);
            }
            marked[item] = true;
        }

        all_sorted.extend_from_slice(&temporary_ordering);
    }

    dbg!(all_sorted);
    */

    fn must_be_before(edges: &[Vec<usize>], nodes: &[u64], before: usize, after: usize) -> bool {
        if edges[before].contains(&after) {
            return true;
        }
        for &child in &edges[before] {
            if must_be_before(edges, nodes, child, after) {
                return true;
            }
        }
        false
    }

    let mut result = 0;
    for update in updates {
        let (nodes_lookup, nodes, edges) = build_nodes(
            rules
                .iter()
                .filter(|(a, b)| update.contains(&a) && update.contains(&b))
                .copied(),
        );

        let mut is_bad = false;
        for ab in update.windows(2) {
            let (a, b) = (ab[0], ab[1]);
            if must_be_before(&edges, &nodes, nodes_lookup[&b], nodes_lookup[&a]) {
                is_bad = true;
                break;
            }
        }
        if !is_bad {
            result += update[update.len() / 2];
        }
    }

    result
}

fn part2(_input: &str) -> u64 {
    0
}

helper::tests! {
    day05 Day05;
    part1 {
        small => 143;
        default => 4959;
    }
    part2 {
        small => 0;
        default => 0;
    }
}
helper::benchmarks! {}
