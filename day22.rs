use std::fs;
use std::collections::HashSet;
use std::collections::VecDeque;

fn parse_player(s: &str) -> VecDeque<i32> {
  s.lines()
    .skip(1)
    .map(|l| l.parse().unwrap())
    .collect()
}

fn combined(p1: &VecDeque<i32>, p2: &VecDeque<i32>) -> (Vec<i32>,Vec<i32>) {
  (p1.iter().cloned().collect(), p2.iter().cloned().collect())
}

fn solve(mut p1: VecDeque<i32>, mut p2: VecDeque<i32>, recurse: bool) -> (bool, i32) {
  let mut seen = HashSet::new();

  while !p1.is_empty() && !p2.is_empty() && seen.insert(combined(&p1, &p2)) {
    let a = p1.pop_front().unwrap();
    let b = p2.pop_front().unwrap();
    let p1_round =
      if recurse && a <= p1.len() as i32 && b <= p2.len() as i32 {
        solve(
          p1.iter().take(a as usize).cloned().collect(),
          p2.iter().take(b as usize).cloned().collect(),
          recurse,
        ).0
      } else {
        a > b
      };

    if p1_round {
      p1.push_back(a);
      p1.push_back(b);
    } else {
      p2.push_back(b);
      p2.push_back(a);
    }
  }

  let p1_won = !p1.is_empty();
  let win_score = if p1_won { p1 } else { p2 }
    .iter().rev().enumerate()
    .map(|(i,c)| c * (i + 1) as i32)
    .sum();
  (p1_won, win_score)
}

fn main() {
  let data = fs::read_to_string("input/input22.txt").unwrap();
  let xs = data.split("\n\n").map(parse_player).collect::<Vec<_>>();

  println!("Part 1: {}", solve(xs[0].clone(), xs[1].clone(), false).1);
  println!("Part 2: {}", solve(xs[0].clone(), xs[1].clone(), true).1);
}