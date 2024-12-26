use std::collections::{HashMap, HashSet};

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

fn part2(input: &str) -> u64 {
    /*
    Adding two binary numbers

    LSB:
       X   Y
    ---|---|---
       |   |
       +---|--+
      (^)--+  |
       |  (&)-+
       |   |
    ---|---|---
       Z  Carry


    other bits:
       X   Y  Carry
    ---|---|---|---
       |   |   |
       |   +---|-----+
       +---|---|--+  |
       |   |   |  |  |
      (^)--+   |  +-(&)
       |       |     |
       +---(&)-+     |
       |    |  |     |
      (^)------+     |
       |    |        |
       |   (|)-------+
       |   ++
    ---|---|---
       Z  Carry
    */

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
    enum Wire<'a> {
        X(u8),
        Y(u8),
        Z(u8),
        Intermediate(&'a str),
    }
    #[derive(Debug, Clone, Copy)]
    enum Value {
        Unknown,
        Op(Op, usize, usize),
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    enum Op {
        And,
        Or,
        Xor,
    }

    let mut wires = Vec::new();
    let mut wire_names = Vec::new();
    let mut wire_by_name = HashMap::new();

    fn intern<'i>(
        wires: &mut Vec<Value>,
        wire_names: &mut Vec<Wire<'i>>,
        wire_by_name: &mut HashMap<Wire<'i>, usize>,
        name: &'i str,
    ) -> (usize, Wire<'i>) {
        let name = if let Some(n) = name.strip_prefix("x") {
            Wire::X(n.parse().unwrap())
        } else if let Some(n) = name.strip_prefix("y") {
            Wire::Y(n.parse().unwrap())
        } else if let Some(n) = name.strip_prefix("z") {
            Wire::Z(n.parse().unwrap())
        } else {
            Wire::Intermediate(name)
        };
        let idx = *wire_by_name.entry(name).or_insert_with(|| {
            let idx = wires.len();
            wires.push(Value::Unknown);
            wire_names.push(name);
            idx
        });
        (idx, name)
    }

    let (_, gates) = input.split_once("\n\n").unwrap();

    let mut zs = Vec::new();

    for computed in gates.lines() {
        let (expr, wire) = computed.split_once(" -> ").unwrap();
        let [lhs, op, rhs] = expr.split(' ').collect_array().unwrap();

        let (wire, wire_name) = intern(&mut wires, &mut wire_names, &mut wire_by_name, wire);
        let (lhs, _) = intern(&mut wires, &mut wire_names, &mut wire_by_name, lhs);
        let (rhs, _) = intern(&mut wires, &mut wire_names, &mut wire_by_name, rhs);

        if let Wire::Z(_) = wire_name {
            zs.push(wire)
        }

        let op = match op {
            "AND" => Op::And,
            "OR" => Op::Or,
            "XOR" => Op::Xor,
            _ => panic!("Invalid op: {op}"),
        };

        wires[wire] = Value::Op(op, lhs, rhs);
    }

    zs.sort_by_key(|wire| wire_names[*wire]);

    #[derive(Debug, Clone, PartialEq, Eq, Hash)]
    enum Pattern {
        Op(Op, Box<Pattern>, Box<Pattern>),
        X(u8),
        Y(u8),
    }

    fn check_match(
        wire_names: &[Wire<'_>],
        wires: &[Value],
        check_cache: &mut HashMap<(usize, Pattern), Result<(), (usize, Pattern)>>,
        wire: usize,
        pattern: &Pattern,
    ) -> Result<(), (usize, Pattern)> {
        let key = (wire, pattern.clone());
        if let Some(cache_value) = check_cache.get(&key) {
            return cache_value.clone();
        }
        let value = check_match_uncached(wire_names, wires, check_cache, wire, pattern);
        let prev_cache_entry = check_cache.insert(key, value.clone());
        assert!(prev_cache_entry.is_none());
        value
    }
    fn check_match_uncached(
        wire_names: &[Wire<'_>],
        wires: &[Value],
        check_cache: &mut HashMap<(usize, Pattern), Result<(), (usize, Pattern)>>,
        wire: usize,
        pattern: &Pattern,
    ) -> Result<(), (usize, Pattern)> {
        //eprintln!("{:?} matches {:?}", wire_names[wire], pattern);

        match (wire_names[wire], pattern) {
            (_, Pattern::Op(_, _, _)) => {}
            (Wire::X(x_value), Pattern::X(x_pattern)) if x_value == *x_pattern => return Ok(()),
            (Wire::Y(y_value), Pattern::Y(y_pattern)) if y_value == *y_pattern => return Ok(()),
            _ => {
                return Err((wire, pattern.clone()));
            }
        }

        match (wires[wire], pattern) {
            (
                Value::Op(op_value, lhs_wire, rhs_wire),
                Pattern::Op(op_pattern, pattern1, pattern2),
            ) if op_value == *op_pattern => {
                let lhs_1 = check_match(wire_names, wires, check_cache, lhs_wire, &pattern1);
                let rhs_2 = check_match(wire_names, wires, check_cache, rhs_wire, &pattern2);
                if lhs_1.is_ok() && rhs_2.is_ok() {
                    return Ok(());
                }

                let lhs_2 = check_match(wire_names, wires, check_cache, lhs_wire, &pattern2);
                let rhs_1 = check_match(wire_names, wires, check_cache, rhs_wire, &pattern1);
                if lhs_2.is_ok() && rhs_1.is_ok() {
                    return Ok(());
                }

                // If both inputs are wrong, this node is likely the culprit.
                // If only one input is wrong, this input is the culprit.
                if lhs_1.is_ok() {
                    rhs_2
                } else if rhs_2.is_ok() {
                    lhs_1
                } else if lhs_2.is_ok() {
                    rhs_1
                } else if rhs_1.is_ok() {
                    lhs_2
                } else {
                    Err((wire, pattern.clone()))
                }
            }
            _ => Err((wire, pattern.clone())),
        }
    }

    let mut incorrect_gates = HashSet::new();

    let mut prev_carry_pat = Pattern::X(255); // dummy

    let mut check_cache = HashMap::new();

    for z_wire in zs {
        let Wire::Z(pos) = wire_names[z_wire] else {
            unreachable!()
        };

        eprintln!("------------ checking z{pos:0>2}");

        let input_xor_pat = Pattern::Op(
            Op::Xor,
            Box::new(Pattern::X(pos)),
            Box::new(Pattern::Y(pos)),
        );

        let (z_pat, carry_pat) = match pos {
            0 => {
                let carry_pat = Pattern::Op(
                    Op::And,
                    Box::new(Pattern::X(pos)),
                    Box::new(Pattern::Y(pos)),
                );
                (input_xor_pat, carry_pat)
            }
            _ => {
                let carry_pat = Pattern::Op(
                    Op::Or,
                    Box::new(Pattern::Op(
                        Op::And,
                        Box::new(Pattern::X(pos)),
                        Box::new(Pattern::Y(pos)),
                    )),
                    Box::new(Pattern::Op(
                        Op::And,
                        Box::new(prev_carry_pat.clone()),
                        Box::new(input_xor_pat.clone()),
                    )),
                );
                (
                    Pattern::Op(Op::Xor, Box::new(input_xor_pat), Box::new(prev_carry_pat)),
                    carry_pat,
                )
            }
        };
        prev_carry_pat = carry_pat;

        if let Err((incorrect_wire, pat)) =
            check_match(&wire_names, &wires, &mut check_cache, z_wire, &z_pat)
        {
            eprintln!("error: {:?}", wire_names[incorrect_wire]);
            incorrect_gates.insert((incorrect_wire, pat));
        }
    }

    dbg!(incorrect_gates.len());

    for (wire, pat) in &incorrect_gates {
        eprintln!("{:?}", wire_names[*wire]);

        let pair = incorrect_gates.iter().find(|(other_wire, other_pat)| {
            check_match(&wire_names, &wires, &mut check_cache, *wire, other_pat).is_ok()
        });
        if let Some(pair) = pair {
            eprintln!(" pairs with {:?}", wire_names[pair.0]);
        }
    }

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
