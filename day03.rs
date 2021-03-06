use std::io;
use std::fs;

fn count_slope(xs: &Vec<Vec<char>>, r: usize, d: usize) -> usize {
  xs.iter()
    .step_by(d)
    .enumerate()
    .filter(|(i,x)| x[(r*i) % x.len()]=='#')
    .count()
}

fn main() -> io::Result<()> {
  let data = fs::read_to_string("input/input03.txt")?;
  let xs = data.split("\n").map(|x| x.chars().collect()).collect();

  println!("Part 1: {}", count_slope(&xs, 3, 1));

  let slopes = [(1,1), (3,1), (5,1), (7,1), (1,2)];
  let ans2 = slopes.iter().map(|&(r,d)| count_slope(&xs, r, d)).fold(1, |acc,x| acc * x);
  println!("Part 2: {}", ans2);

  Ok(())
}