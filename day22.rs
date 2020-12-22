use std::{collections::{HashMap, HashSet}, fs};
use std::collections::VecDeque;

fn parse_player(s: &str) -> VecDeque<i32> {
  s.lines()
    .skip(1)
    .map(|l| l.parse().unwrap())
    .collect()
}

fn part1(mut p1: VecDeque<i32>, mut p2: VecDeque<i32>) -> i32 {
  while !p1.is_empty() && !p2.is_empty() {
    let a = p1.pop_front().unwrap();
    let b = p2.pop_front().unwrap();
    if a > b {
      p1.push_back(a);
      p1.push_back(b);
    } else {
      p2.push_back(b);
      p2.push_back(a);
    }
  }

  (if p1.is_empty() { p2} else { p1 })
    .iter().rev().enumerate()
    .map(|(i,c)| c * (i + 1) as i32)
    .sum()
}

fn tot_vec(p1: &VecDeque<i32>, p2: &VecDeque<i32>) -> Vec<i32> {
  p1.iter().cloned().chain(p2.iter().cloned()).collect()
}

fn fact(n: i32) -> i128 {
  let mut f = 1;
  for i in 1..=n { f *= i as i128 }
  f
}

fn perm_idx(vs: &[i32]) -> i128 {
  let n = vs.len() as i32;
  let mut acc = 0;
  for i in 0..vs.len()-1 {
    let m = vs[i+1..].iter().filter(|v| **v < vs[i]).count() as i128;
    acc += m * fact(n-(i as i32)-1)
  }
  acc
}


fn part2(mut p1: VecDeque<i32>, mut p2: VecDeque<i32>, dp: &mut HashMap<i128,bool>) -> bool {
  let mut seen = HashSet::new();
  let n = p1.len() + p2.len();
  let mut i = 0;

  let vs = perm_idx(&tot_vec(&p1, &p2)) * 50 + n as i128;
  if dp.contains_key(&vs) {
    return dp[&vs];
  }

  while !p1.is_empty() && !p2.is_empty() && seen.insert(perm_idx(&tot_vec(&p1, &p2))) {
    let a = p1.pop_front().unwrap();
    let b = p2.pop_front().unwrap();
    if n >= 46 {
      i += 1;
      println!("Iter {} {}", n, i);
    }
    let p1_round =
      if a <= p1.len() as i32 && b <= p2.len() as i32 {
        part2(p1.clone(), p2.clone(), dp)
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
  /*
  let winner = if p1_won { p1 } else { p2 };
  let win_score = winner
    .iter().rev().enumerate()
    .map(|(i,c)| c * (i + 1) as i32)
    .sum();*/
  let res = p1_won;//(p1_won, win_score);
  dp.insert(vs, res);
  res
}

fn main() {
  let data = fs::read_to_string("input/input22.txt").unwrap();
  let xs = data.split("\n\n").map(parse_player).collect::<Vec<_>>();

  println!("Part 1: {}", part1(xs[0].clone(), xs[1].clone()));
  println!("Part 2: {}", part2(xs[0].clone(), xs[1].clone(), &mut HashMap::new()));
}