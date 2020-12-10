use std::fs;

fn find_sum(ys: &[i64], v: i64) -> bool {
  let mut xs: Vec<i64> = ys.iter().cloned().collect();
  xs.sort();

  let mut i = 0;
  let mut j = xs.len()-1;

  while i < j && xs[i]+xs[j] != v {
    if xs[i]+xs[j] < v { i += 1 }
    else { j -= 1 }
  }

  i<j && xs[i]+xs[j]==v
}

fn part1(xs: &Vec<i64>) -> i64 {
  let n = xs.len();

  for i in 25..n {
    if ! find_sum(&xs[i-25..i], xs[i]) {
      return xs[i]
    }
  }

  -1
}

fn part2(xs: &Vec<i64>, v: i64) -> i64 {
  // Two Pointers again

  let n = xs.len();
  let mut i = 0;
  let mut j = 2;
  let mut acc = xs[0]+xs[1];

  while acc != v && (j < n || i+2 < j) {
    if (acc < v && j < n) || i+2 >= j {
      acc += xs[j];
      j += 1;
    } else {
      acc -= xs[i];
      i += 1;
    }
  }

  let vs = &xs[i..j];
  *vs.iter().max().unwrap()+*vs.iter().min().unwrap()
}

fn main() {
  let xs: Vec<i64> =
    fs::read_to_string("input/input09.txt")
      .unwrap()
      .lines()
      .map(|x| x.parse().unwrap())
      .collect();
    
  let v = part1(&xs);
  println!("Part 1: {}", v);
  println!("Part 2: {}", part2(&xs, v));
}