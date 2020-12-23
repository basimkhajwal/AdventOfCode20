fn step(curr: usize, next: &mut Vec<usize>) -> usize {
  let n = next.len() - 1;
  let a = next[curr];
  let b  = next[a];
  let c = next[b];
  let xs = [curr, a, b, c];

  let mut dest = curr;
  while xs.contains(&dest) {
    dest = if dest == 1 { n } else { dest - 1 };
  }

  next[curr] = next[c];
  next[c] = next[dest];
  next[dest] = a;
  next[curr]
}

fn data_ll(data: &Vec<usize>) -> (usize, Vec<usize>) {
  let mut next = vec![0; data.len()+1];
  for i in 0..data.len() {
    next[data[i]] = data[(i+1)%data.len()];
  }
  (data[0], next)
}

fn part1(data: &Vec<usize>) -> String {
  let (mut curr, mut next) = data_ll(data);
  for _ in 0..100 { curr = step(curr, &mut next); }

  let mut res = "".to_owned();
  let mut i = next[1];
  while i != 1 {
    res.push_str(&i.to_string());
    i = next[i];
  }
  res
}

fn part2(data: &Vec<usize>) -> usize {
  let data: Vec<usize> = data.iter().cloned().chain(10..=1000000).collect();
  let (mut curr, mut next) = data_ll(&data);
  for _ in 0..10000000 { curr = step(curr, &mut next); }
  next[1] * next[next[1]]
}

fn main() {
  let data =
    "871369452".chars()
      .map(|c| c.to_digit(10).unwrap() as usize)
      .collect();
  println!("Part 1: {}", part1(&data));
  println!("Part 2: {}", part2(&data));
}