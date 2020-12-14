use std::fs;
use std::collections::HashMap;

enum Instruction {
  Mask(String),
  Mem(i64, i64),
}

fn parse_instruction(s: &str) -> Instruction {
  let xs: Vec<&str> = s.split(" = ").collect();
  if xs[0] == "mask" {
    Instruction::Mask(xs[1].to_owned())
  } else {
    let addr = xs[0][4..xs[0].len()-1].parse().unwrap();
    let v = xs[1].parse().unwrap();
    Instruction::Mem(addr, v)
  }
}

fn part1(instructions: &Vec<Instruction>) -> i64 {
  let mut mem = HashMap::new();
  let mut mask = "";

  for i in instructions {
    match i {
      Instruction::Mask(m) => { mask = m; },
      Instruction::Mem(a, mut b) => {
        for (i,c) in mask.chars().rev().enumerate() {
          let v = 1 << i;
          if c == '0' { b = b & (!v) }
          if c == '1' { b = b | v }
        }
        mem.insert(*a, b);
      },
    }
  }

  mem.values().sum()
}

fn gen_addrs(ans: &mut Vec<i64>, addr: i64, acc: i64, mask: &Vec<char>, idx: usize) -> () {
  if idx == mask.len() {
    ans.push(acc);
    return;
  }

  let new_accs = match mask[idx] {
    '0' => vec![ acc * 2 + ((addr >> (35 - idx)) & 1) ],
    '1' => vec![ acc * 2 + 1 ],
    _   => vec![ acc * 2, acc * 2 + 1],
  };
  
  for new_acc in new_accs {
    gen_addrs(ans, addr, new_acc, mask, idx+1);
  }
}

fn part2(instructions: &Vec<Instruction>) -> i64 {
  let mut mem = HashMap::new();
  let mut mask = vec![];

  for i in instructions {
    match i {
      Instruction::Mask(m) => { mask = m.chars().collect(); },
      Instruction::Mem(a, b) => {
        let mut addrs = vec![];
        gen_addrs(&mut addrs, *a, 0, &mask, 0);
        for addr in addrs {
          mem.insert(addr, *b);
        }
      },
    }
  }

  mem.values().sum()
}

fn main() {
  let xs: Vec<_> =
    fs::read_to_string("input/input14.txt")
      .unwrap().lines()
      .map(parse_instruction)
      .collect();
  
  println!("Part 1: {}", part1(&xs));
  println!("Part 2: {}", part2(&xs));
}