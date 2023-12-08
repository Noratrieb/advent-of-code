use std::collections::HashMap;

#[derive(Debug)]
struct Map {
    instructions: Vec<Instruction>,
    nodes: Vec<Node>,
    aaa: usize,
    zzz: usize,
}

#[derive(Debug)]
struct Node {
    left_right: [usize; 2],
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
    let mut aaa = 0;
    let mut zzz = 0;
    let nodes = lines
        .enumerate()
        .map(|(i, line)| {
            let (name, next) = line.split_once(" = ").unwrap();
            names.insert(name, i);
            if name == "AAA" {
                aaa = i;
            } else if name == "ZZZ" {
                zzz = i;
            }
            let (left, right) = next
                .strip_prefix("(")
                .unwrap()
                .strip_suffix(")")
                .unwrap()
                .split_once(", ")
                .unwrap();
            (left, right)
        })
        .collect::<Vec<_>>();

    let nodes = nodes
        .into_iter()
        .map(|(left, right)| Node {
            left_right: [*names.get(left).unwrap(), *names.get(right).unwrap()],
        })
        .collect();

    Map {
        instructions,
        nodes,
        aaa,
        zzz,
    }
}

pub fn part1(input: &str) -> u64 {
    let map = parse(input);

    let mut next = map.instructions.iter().cycle();
    let mut node = map.aaa;
    let mut i = 0;
    while node != map.zzz {
        node = map.nodes[node].left_right[*next.next().unwrap() as usize];
        i += 1;
    }

    i
}
