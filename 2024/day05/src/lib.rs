use std::collections::VecDeque;

use helper::{parse_unwrap, Day, IteratorExt, Variants};

pub fn main() {
    helper::main::<Day05>(include_str!("../input.txt"));
}

struct Day05;

helper::define_variants! {
    day => crate::Day05;
    part1 {
        basic => crate::part1, sample_count=1000;
    }
    part2 {
        basic => crate::part2, sample_count=1000;
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
    struct Update {
        values: Vec<u64>,
        set: [bool; 100],
    }

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
        let values = line.split(",").map(parse_unwrap).collect::<Vec<_>>();
        let mut set = [false; 100];
        for value in &values {
            set[*value as usize] = true;
        }
        updates.push(Update { values, set });
    }

    fn build_nodes(edges: &mut Vec<[bool; 100]>, rules: impl Iterator<Item = (u64, u64)>) {
        edges.clear();

        edges.resize(100, [false; 100]);

        for (first, then) in rules {
            edges[first as usize][then as usize] = true;
        }
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

    let mut edges = Vec::new();

    let mut result = 0;
    for update in updates {
        build_nodes(
            &mut edges,
            rules
                .iter()
                .filter(|(a, b)| update.set[*a as usize] && update.set[*b as usize])
                .copied(),
        );

        let mut is_bad = false;
        for ab in update.values.windows(2) {
            let (a, b) = (ab[0], ab[1]);
            if edges[b as usize][a as usize] {
                is_bad = true;
                break;
            }
        }
        if !is_bad {
            result += update.values[update.values.len() / 2];
        }
    }

    result
}

fn part2(input: &str) -> u64 {
    struct Update {
        values: Vec<u64>,
        set: [bool; 100],
    }

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
        let values = line.split(",").map(parse_unwrap).collect::<Vec<_>>();
        let mut set = [false; 100];
        for value in &values {
            set[*value as usize] = true;
        }
        updates.push(Update { values, set });
    }

    fn build_nodes(edges: &mut Vec<[bool; 100]>, rules: impl Iterator<Item = (u64, u64)>) {
        edges.clear();

        edges.resize(100, [false; 100]);

        for (first, then) in rules {
            edges[first as usize][then as usize] = true;
        }
    }

    let mut edges = Vec::new();

    let mut result = 0;
    for update in updates {
        build_nodes(
            &mut edges,
            rules
                .iter()
                .filter(|(a, b)| update.set[*a as usize] && update.set[*b as usize])
                .copied(),
        );

        let mut is_bad = false;
        for ab in update.values.windows(2) {
            let (a, b) = (ab[0], ab[1]);
            if edges[b as usize][a as usize] {
                is_bad = true;
                break;
            }
        }
        if is_bad {
            // do Topological sort:
            // https://en.wikipedia.org/wiki/Topological_sorting#Depth-first_search
            let mut all_sorted = VecDeque::new();
            let mut marked = [false; 100];

            fn visit(
                all_sorted: &mut VecDeque<u64>,
                marked: &mut [bool; 100],
                edges: &[[bool; 100]],
                node: usize,
            ) {
                if marked[node] {
                    return;
                }

                for (child, _) in edges[node].iter().enumerate().filter(|(_, edge)| **edge) {
                    visit(all_sorted, marked, edges, child);
                }

                marked[node] = true;
                all_sorted.push_front(node as u64);
            }

            while let Some(next) = update.values.iter().find(|value| !marked[**value as usize]) {
                visit(&mut all_sorted, &mut marked, &edges, *next as usize);
            }

            result += all_sorted[all_sorted.len() / 2];
        }
    }

    result
}

helper::tests! {
    day05 Day05;
    part1 {
        small => 143;
        default => 4959;
    }
    part2 {
        small => 123;
        default => 4655;
    }
}
helper::benchmarks! {}
