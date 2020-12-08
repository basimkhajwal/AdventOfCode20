use std::fs;

enum Op {
  Nop(i32),
  Acc(i32),
  Jmp(i32),
}

enum Res {
  Loop(i32),
  Finished(i32),
}

fn parse_op(s: &str) -> Op {
  let words: Vec<&str> = s.split(" ").collect();
  let v = words[1].parse().unwrap();

  match words[0] {
    "acc" => Op::Acc(v),
    "jmp" => Op::Jmp(v),
    _     => Op::Nop(v),
  }
}

fn check_loop(ops: &Vec<Op>) -> Res {
  let mut pc: usize = 0;
  let mut acc: i32 = 0;
  let mut seen = vec![false; ops.len()];

  while pc < ops.len() && !seen[pc] {
    seen[pc] = true;
    match ops[pc] {
      Op::Nop(_)    => pc += 1,
      Op::Acc(v)    => { pc += 1; acc += v },
      Op::Jmp(v)    => pc = ((pc as i32) + v) as usize,
    }
  }

  if pc < ops.len() { Res::Loop(acc) }
  else { Res::Finished(acc) }
}

fn part1(ops: &Vec<Op>) -> i32 {
  match check_loop(ops) {
    Res::Loop(acc) => acc,
    Res::Finished(_) => -1,
  }
}

fn part2(ops: &mut Vec<Op>) -> i32 {

  for i in 0..ops.len() {
    match ops[i] {
      Op::Acc(_) => continue,
      Op::Jmp(v) => {
        ops[i] = Op::Nop(v);
        if let Res::Finished(acc) = check_loop(ops) {
          return acc
        }
        ops[i] = Op::Jmp(v)
      },
      Op::Nop(v) => {
        ops[i] = Op::Jmp(v);
        if let Res::Finished(acc) = check_loop(ops) {
          return acc
        }
        ops[i] = Op::Nop(v)
      }
    }
  }

  -1
}

fn main() {
  let input = fs::read_to_string("input/input8.txt").unwrap();
  let mut ops: Vec<Op> = input.lines().map(parse_op).collect();

  println!("Part 1: {}", part1(&ops));
  println!("Part 2: {}", part2(&mut ops));
}