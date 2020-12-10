use std::io;
use std::fs;

fn valid_p1(pwd: &str) -> bool {
  let xs = pwd.split(":").collect::<Vec<_>>();
  let ys = xs[0].split(" ").collect::<Vec<_>>();
  let zs = ys[0].split("-").map(|y| y.parse().unwrap()).collect::<Vec<_>>();

  let s = xs[1];
  let c = ys[1].chars().next().unwrap();
  let x = s.matches(c).count();
  zs[0] <= x && x <= zs[1]
}

fn valid_p2(pwd: &str) -> bool {
  let xs = pwd.split(":").collect::<Vec<_>>();
  let ys = xs[0].split(" ").collect::<Vec<_>>();
  let zs = ys[0].split("-").map(|y| y.parse::<usize>().unwrap()).collect::<Vec<_>>();

  let s = xs[1].chars().collect::<Vec<_>>();
  let c = ys[1].chars().next().unwrap();
  (s[zs[0]]==c) ^ (s[zs[1]]==c)
}

fn main() -> io::Result<()> {
  let data = fs::read_to_string("input/input02.txt")?;
  let xs = data.split("\n").collect::<Vec<_>>();
  let ans1 = xs.iter().filter(|&s| valid_p1(s)).count();
  let ans2 = xs.iter().filter(|&s| valid_p2(s)).count();
  println!("{} {}", ans1, ans2);
  Ok(())
}