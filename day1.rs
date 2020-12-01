use std::fs;
use std::io;

fn main() -> io::Result<()> {

  let mut xs: Vec<i32> =
    fs::read_to_string("input/input1.txt")?
      .split_whitespace()
      .map(|s: &str| s.parse().unwrap())
      .collect();
  xs.sort();

  // Solve both parts using two-pointers trick
  // in O(n) and O(n^2) respectively.

  let target = 2020;
  let mut i = 0;
  let mut j = xs.len()-1;

  while i < j && xs[i]+xs[j] != target {
    if xs[i]+xs[j] < target { i += 1; }
    else { j -= 1; }
  }
  println!("Part One: {}", xs[i]*xs[j]);

  i = 0;
  j = 1;
  let mut k = xs.len()-1;
  while j < k && xs[i]+xs[j]+xs[k] != target {
    if xs[i]+xs[j]+xs[k] < target {
      j += 1;
      if j == k {
        i += 1;
        j = i+1;
        k = xs.len()-1;
      }
    } else {
      k -= 1;
    }
  }
  println!("Part Two: {}", xs[i]*xs[j]*xs[k]);

  Ok(())
}