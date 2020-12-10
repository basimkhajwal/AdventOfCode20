use std::fs;

fn part1(xs: &Vec<i32>) -> i32 {
  let mut counts = [0; 4];
  for i in 1..xs.len() {
    counts[(xs[i] - xs[i-1]) as usize] += 1;
  }
  counts[1] * counts[3]
}

fn part2(xs: &Vec<i32>) -> i64 {
  let n = xs.len();
  let mut dp = vec![0; n];

  dp[0] = 1;
  for i in 1..n {
    let mut j: i32 = (i-1) as i32;
    while j >= 0 && xs[j as usize] >= xs[i]-3 {
      dp[i] += dp[j as usize];
      j -= 1
    }
  }

  dp[n-1]
}

fn main() {
  let mut adaptors: Vec<i32> =
    fs::read_to_string("input/input10.txt")
      .unwrap()
      .lines()
      .map(|x| x.parse().unwrap())
      .collect();
  adaptors.push(0);
  adaptors.push(*adaptors.iter().max().unwrap()+3);
  adaptors.sort();

  println!("Part 1: {}", part1(&adaptors));
  println!("Part 2: {}", part2(&adaptors));
}