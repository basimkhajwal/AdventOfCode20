use std::fs;
use std::collections::HashSet;

fn group_or(s: &str) -> usize {
  s.split("\n")
   .flat_map(|l| l.chars())
   .collect::<HashSet<_>>()
   .len()
}

fn group_and(s: &str) -> usize {
  s.split("\n")
   .fold(
      ('a'..='z').collect::<HashSet<_>>(),
      |acc, l|
        acc.intersection(&l.chars().collect())
           .copied().collect()
    )
   .len()
}

fn main() {
  let data = fs::read_to_string("input/input6.txt").unwrap();
  let groups: Vec<&str> = data.split("\n\n").collect();

  println!("Part 1: {}", groups.iter().copied().map(group_or).sum::<usize>());
  println!("Part 2: {}", groups.iter().copied().map(group_and).sum::<usize>());
}