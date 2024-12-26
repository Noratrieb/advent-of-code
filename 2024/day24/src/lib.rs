use std::collections::HashMap;

use helper::{Day, IteratorExt, Variants};

pub fn main() {
    helper::main::<Day24>(include_str!("../input.txt"));
}

struct Day24;

helper::define_variants! {
    day => crate::Day24;
    part1 {
        basic => crate::part1;
    }
    part2 {
        basic => crate::part2;
    }
}

impl Day for Day24 {
    fn part1() -> Variants {
        part1_variants!(construct_variants)
    }

    fn part2() -> Variants {
        part2_variants!(construct_variants)
    }
}

fn part1(input: &str) -> u64 {
    #[derive(Debug, Clone, Copy)]
    enum Value {
        None,
        Known(bool),
        Op(Op, usize, usize),
    }
    #[derive(Debug, Clone, Copy)]
    enum Op {
        And,
        Or,
        Xor,
    }

    let mut wires = Vec::new();
    let mut wire_by_name = HashMap::new();

    fn intern<'i>(
        wires: &mut Vec<Value>,
        wire_by_name: &mut HashMap<&'i str, usize>,
        name: &'i str,
    ) -> usize {
        *wire_by_name.entry(name).or_insert_with(|| {
            let idx = wires.len();
            wires.push(Value::None);
            idx
        })
    }

    let (start, gates) = input.split_once("\n\n").unwrap();

    for initial in start.lines() {
        let (wire, value) = initial.split_once(": ").unwrap();
        let value = value.parse::<u8>().unwrap() == 1;

        let wire = intern(&mut wires, &mut wire_by_name, wire);
        wires[wire] = Value::Known(value);
    }

    for computed in gates.lines() {
        let (expr, wire) = computed.split_once(" -> ").unwrap();
        let [lhs, op, rhs] = expr.split(' ').collect_array().unwrap();

        let wire = intern(&mut wires, &mut wire_by_name, wire);
        let lhs = intern(&mut wires, &mut wire_by_name, lhs);
        let rhs = intern(&mut wires, &mut wire_by_name, rhs);

        let op = match op {
            "AND" => Op::And,
            "OR" => Op::Or,
            "XOR" => Op::Xor,
            _ => panic!("Invalid op: {op}"),
        };

        wires[wire] = Value::Op(op, lhs, rhs);
    }

    fn eval(wires: &mut Vec<Value>, wire: usize) -> Option<bool> {
        match wires[wire] {
            Value::None => None,
            Value::Known(v) => Some(v),
            Value::Op(op, lhs, rhs) => {
                let lhs = eval(wires, lhs)?;
                let rhs = eval(wires, rhs)?;
                let result = match op {
                    Op::And => lhs & rhs,
                    Op::Or => lhs | rhs,
                    Op::Xor => lhs ^ rhs,
                };

                wires[wire] = Value::Known(result);

                Some(result)
            }
        }
    }

    let mut result = 0;

    for (wire_name, wire) in wire_by_name {
        let Some(bit_number) = wire_name.strip_prefix("z") else {
            continue;
        };
        let bit_number = bit_number.parse::<u32>().unwrap();

        let value = eval(&mut wires, wire).unwrap();

        result |= (value as u64) << bit_number;
    }

    result
}

fn part2(_input: &str) -> u64 {
    0
}

helper::tests! {
    day24 Day24;
    part1 {
        small => 4;
        default => 41324968993486;
    }
    part2 {
        small => 0;
        default => 0;
    }
}
helper::benchmarks! {}
