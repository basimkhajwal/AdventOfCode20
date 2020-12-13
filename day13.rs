use std::fs;

fn wait_time(t: i32, b: i32) -> i32 { (b - (t % b)) % b }

fn part1(t: i32, buses: &Vec<(usize, i32)>) -> i32 {
  let (_, b) = *buses.iter().min_by_key(|&&(_, b)| wait_time(t, b)).unwrap();
  b * wait_time(t, b)
}

fn solve(xs: &[(i128, i128)]) -> (i128, i128) {
  if xs.len() == 0 {
    (0, 1)
  } else {
    let (y, q) = solve(&xs[1..]);
    let (k, p) = xs[0];
    let mut ans = y;
    while (ans + k) % p != 0 { ans += q }
    (ans, q * p)
  }
}

fn part2(buses: &Vec<(usize, i32)>) -> i128 {
  let xs: Vec<_> = buses.iter().map(|&(i,b)| (i as i128, b as i128)).collect();
  solve(&xs).0
}

fn main() {
  let data = fs::read_to_string("input/input13.txt").unwrap();
  let xs: Vec<&str> = data.lines().collect();

  let t = xs[0].parse().unwrap();
  let buses =
    xs[1].split(",").enumerate().filter_map(|(i,x)| x.parse().ok().map(|v| (i,v))).collect();

  println!("Part 1: {}", part1(t, &buses));
  println!("Part 2: {}", part2(&buses));
}