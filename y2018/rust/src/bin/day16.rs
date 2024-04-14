use itertools::Itertools;
use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::fs;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

const P1_THRESHOLD: u32 = 3;

#[derive(Debug, EnumIter, Eq, PartialEq, Hash, Copy, Clone)]
enum Op {
    Addr,
    Addi,
    Mulr,
    Muli,
    Banr,
    Bani,
    Borr,
    Bori,
    Setr,
    Seti,
    Gtir,
    Gtri,
    Gtrr,
    Eqir,
    Eqri,
    Erqq,
}

impl Op {
    fn apply(&self, reg: &Vec<i32>, a: i32, b: i32, c: i32) -> Vec<i32> {
        let mut reg = reg.clone();
        reg[c as usize] = match self {
            Op::Addr => reg[a as usize] + reg[b as usize],
            Op::Addi => reg[a as usize] + b,
            Op::Mulr => reg[a as usize] * reg[b as usize],
            Op::Muli => reg[a as usize] * b,
            Op::Banr => reg[a as usize] & reg[b as usize],
            Op::Bani => reg[a as usize] & b,
            Op::Borr => reg[a as usize] | reg[b as usize],
            Op::Bori => reg[a as usize] | b,
            Op::Setr => reg[a as usize],
            Op::Seti => a,
            Op::Gtir => i32::from(a > reg[b as usize]),
            Op::Gtri => i32::from(reg[a as usize] > b),
            Op::Gtrr => i32::from(reg[a as usize] > reg[b as usize]),
            Op::Eqir => i32::from(a == reg[b as usize]),
            Op::Eqri => i32::from(reg[a as usize] == b),
            Op::Erqq => i32::from(reg[a as usize] == reg[b as usize]),
        };
        reg
    }
}

fn main() {
    let mut read = fs::read_to_string("../input/day16p2.txt").unwrap();
    // this might be the nastiest solution to ever exist
    while read.lines().count() % 4 != 0 {
        read += "\n";
    }

    let before_fmt = Regex::new(r"Before:\s+\[(\d+),\s+(\d+),\s+(\d+),\s+(\d+)]").unwrap();
    let after_fmt = Regex::new(r"After:\s+\[(\d+),\s+(\d+),\s+(\d+),\s+(\d+)]").unwrap();
    let mut samples = Vec::new();
    for (bef, op, aft, _) in read.lines().tuples() {
        let bef_match = before_fmt.captures(bef).unwrap();
        let aft_match = after_fmt.captures(aft).unwrap();
        let mut bef_reg = Vec::new();
        let mut aft_reg = Vec::new();
        for i in 1..=4 {
            bef_reg.push(bef_match[i].parse::<i32>().unwrap());
            aft_reg.push(aft_match[i].parse::<i32>().unwrap());
        }

        let op_split: Vec<_> = op.split_whitespace().collect();
        let parsed_op = (
            op_split[0].parse::<i32>().unwrap(),
            op_split[1].parse::<i32>().unwrap(),
            op_split[2].parse::<i32>().unwrap(),
            op_split[3].parse::<i32>().unwrap(),
        );
        samples.push((bef_reg, parsed_op, aft_reg));
    }

    let mut valid_ops = Vec::new();
    let mut total = 0;
    for (bef, op, aft) in &samples {
        let mut good = HashSet::new();
        for poss in Op::iter() {
            let res = poss.apply(&bef, op.1, op.2, op.3);
            if res.eq(aft) {
                good.insert(poss);
            }
        }
        total += (good.len() >= P1_THRESHOLD as usize) as u32;
        valid_ops.push(good);
    }
    println!("values that have at least {P1_THRESHOLD} valid ops: {total}");

    let mut defined = HashMap::new();
    while defined.len() < Op::iter().count() {
        let mut now_valid = Vec::new();
        for (valid, (_, op, _)) in valid_ops.iter().zip(samples.iter()) {
            if valid.len() != 1 {
                continue;
            }
            let new = valid.iter().next().unwrap().clone();
            defined.insert(op.0, new);
            now_valid.push(new);
        }
        valid_ops.iter_mut().for_each(|k| {
            now_valid.iter().for_each(|o| {
                k.remove(o);
            })
        });
    }

    let instructions: Vec<_> = fs::read_to_string("../input/day16p1.txt")
        .unwrap()
        .lines()
        .map(|i| {
            let i: Vec<_> = i
                .split_whitespace()
                .map(|s| s.parse::<i32>().unwrap())
                .collect();
            (defined[&i[0]], i[1], i[2], i[3])
        })
        .collect();

    let mut reg = vec![0; 4];
    for (op, a, b, c) in &instructions {
        reg = op.apply(&reg, *a, *b, *c);
    }
    println!("value of register 0: {}", reg[0]);
}
