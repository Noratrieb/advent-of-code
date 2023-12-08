use std::collections::{HashMap, HashSet};

#[derive(Debug)]
struct Map {
    instructions: Vec<Instruction>,
    nodes: Vec<Node>,
    a_nodes: Vec<usize>,
}

#[derive(Debug)]
struct Node {
    left_right: [usize; 2],
    z: bool,
}

#[derive(Clone, Copy, Debug)]
enum Instruction {
    Left,
    Right,
}

fn parse(input: &str) -> Map {
    let mut lines = input.lines();
    let instructions = lines
        .next()
        .unwrap()
        .bytes()
        .map(|b| match b {
            b'R' => Instruction::Right,
            b'L' => Instruction::Left,
            _ => panic!(),
        })
        .collect();
    let _ = lines.next().unwrap();

    let mut names = HashMap::new();
    let mut a_nodes = Vec::new();

    let nodes = lines
        .enumerate()
        .map(|(i, line)| {
            let (name, next) = line.split_once(" = ").unwrap();
            names.insert(name, i);
            if name.ends_with('A') {
                a_nodes.push(i);
            }
            let z = name.ends_with('Z');
            let (left, right) = next
                .strip_prefix("(")
                .unwrap()
                .strip_suffix(")")
                .unwrap()
                .split_once(", ")
                .unwrap();

            (left, right, z)
        })
        .collect::<Vec<_>>();

    let nodes = nodes
        .into_iter()
        .map(|(left, right, z)| Node {
            left_right: [*names.get(left).unwrap(), *names.get(right).unwrap()],
            z,
        })
        .collect();

    Map {
        instructions,
        nodes,
        a_nodes,
    }
}

fn optimize(nodes: &mut [Node]) {
    let mut optimized = true;
    while optimized {
        optimized = false;
        for node_idx in 0..nodes.len() {
            let node = &nodes[node_idx];
            if node.z {
                continue;
            }
            if node.left_right[0] == node.left_right[1] {
                let replace = node.left_right[0];

                for i in 0..nodes.len() {
                    let node = &mut nodes[i];
                    if node.left_right[0] == node_idx {
                        node.left_right[0] = replace;
                        optimized = true;
                    }
                    if node.left_right[1] == node_idx {
                        node.left_right[1] = replace;
                        optimized = true;
                    }
                }
            }
        }
    }
}

// example:
// idx 0 cycles every 2 instructions
// idx 1 cycles every 3 instructions

// lcm(2, 3) = 6

// we're conservatively overcounting a bit sometimes
// idx 0 cycles every 1 sequences (in reality, 1)
// idx 1 cycles every 3 sequences (in realtiy, 1.5)

// lcm(1, 3) = 3 (3 * 2 = 6)

fn find_the_cycles(map: &Map) -> Vec<usize> {
    let mut periods = Vec::new();
    for start in &map.a_nodes {
        println!("node {start}");
        let mut locations = HashMap::new();
        let mut node = *start;
        let mut period = 0_usize;
        loop {
            for next in &map.instructions {
                node = map.nodes[node].left_right[*next as usize];
            }
            let end_location = node;
            println!("{end_location}");
            if let Some(start) = locations.get(&end_location) {
                assert_eq!(*start, 0);
                periods.push(period - start);
                break;
            }
            locations.insert(end_location, period);
            period += 1;
        }
    }
    periods
}

pub fn part2(input: &str) -> u64 {
    let mut map = parse(input);

    //optimize(&mut map.nodes);
    let cycles = find_the_cycles(&map);
    dbg!(&cycles);
    //return 0;

    let count_to_z = cycles
        .iter()
        .zip(&map.a_nodes)
        .map(|(period, node)| {
            let mut node = *node;
            for next in &map.instructions {
                node = map.nodes[node].left_right[*next as usize];
            }

            for (i, next) in map
                .instructions
                .iter()
                .cycle()
                .take(map.instructions.len() * period)
                .enumerate()
            {
                node = map.nodes[node].left_right[*next as usize];
                if map.nodes[node].z {
                    return i;
                }
            }
            unreachable!()
        })
        .collect::<Vec<_>>();

    dbg!(&count_to_z);

    let mut next = map.instructions.iter().cycle();
    let mut nodes = map.a_nodes;
    let mut i = 0;

    loop {
        let mut count = 0;

        let next = next.next().unwrap();
        nodes.iter_mut().for_each(|node| {
            *node = map.nodes[*node].left_right[*next as usize];

            if map.nodes[*node].z {
                count += 1;
            }
        });
        i += 1;

        if count == nodes.len() {
            break;
        }
    }

    i
}
