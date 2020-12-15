use std::collections::HashMap;

fn nth_spoken(start: &[i32], n: usize) -> i32 {
  let mut last_seen = HashMap::new();

  for (i,x) in start[0..start.len()-1].iter().enumerate() {
    last_seen.insert(*x, i);
  }

  let mut i = start.len()-1;
  let mut v = start[i];

  while i < n-1 {
    let w = (i as i32) - (*last_seen.get(&v).unwrap_or(&i) as i32);
    last_seen.insert(v, i);
    v = w;
    i += 1;
  }

  v
}

fn main() {
  let xs = vec![20,0,1,11,6,3];
  println!("Part 1: {}", nth_spoken(&xs, 2020));
  println!("Part 2: {}", nth_spoken(&xs, 30000000));
}