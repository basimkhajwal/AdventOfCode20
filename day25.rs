use std::fs;

const MOD: i64 = 20201227;

fn find_subject(x: i64) -> i64 {
  for y in 0..MOD {
    if mod_pow(7, y, MOD) == x {
      return y
    }
  }
  println!("Not found");
  -1
}

fn mod_pow(mut a: i64, mut b: i64, m: i64) -> i64 {
  let mut res = 1;
  while b > 0 {
    if b % 2 == 1 { res = (res * a) % m; }
    b >>= 1;
    a = (a * a) % m;
  }
  res
}

fn part1(a: i64, b: i64) -> i64 {
  mod_pow(7, find_subject(a) * find_subject(b), MOD)
}

fn main() {
  let xs: Vec<i64> = fs::read_to_string("input/input25.txt").unwrap()
    .lines().map(|x| x.parse().unwrap()).collect();
  println!("Part 1: {}", part1(xs[0], xs[1]));
}