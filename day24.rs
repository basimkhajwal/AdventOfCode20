use std::fs;
use std::collections::HashSet;

enum Step {
  E,
  W,
  SE,
  SW,
  NE,
  NW,
}

fn parse_steps(s: &str) -> Vec<Step> {
  let mut steps = vec![];
  let mut it = s.chars();
  while let Some(c) = it.next() {
    match c {
      'e' => steps.push(Step::E),
      'w' => steps.push(Step::W),
      _ => {
        match (c,it.next()) {
          ('s',Some('e')) => steps.push(Step::SE),
          ('s',Some('w')) => steps.push(Step::SW),
          ('n',Some('e')) => steps.push(Step::NE),
          ('n',Some('w')) => steps.push(Step::NW),
          _ => {
            println!("Error: Unrecognised sequence");
          }
        }
      }
    }
  }
  steps
}

type Pos = (i32, i32);

fn step_dir(&(x, y): &Pos, dir: &Step) -> Pos {
  match dir {
    Step::E  => (x+1, y),
    Step::W  => (x-1, y),
    Step::NE => (x, y+1),
    Step::SW => (x, y-1),
    Step::NW => (x-1, y+1),
    Step::SE => (x+1, y-1),
  }
}

fn neighbours(pos: &Pos) -> Vec<Pos> {
  let dirs = [Step::E, Step::W, Step::NE, Step::NW, Step::SE, Step::SW];

  dirs.iter().map(|d| step_dir(pos, d)).collect()
}

fn get_blacks(xs: &Vec<Vec<Step>>) -> HashSet<Pos> {
  let mut blacks = HashSet::new();
  for steps in xs {
    let mut pos = (0,0);
    for s in steps { pos = step_dir(&pos, s) }
    if !blacks.insert(pos) {
      blacks.remove(&pos);
    }
  }
  blacks
}

fn iter(blacks: &HashSet<Pos>) -> HashSet<Pos> {
  blacks.iter()
    .flat_map(neighbours)
    .collect::<HashSet<_>>()
    .into_iter()
    .filter(|pos| {
      let n = neighbours(&pos).into_iter()
        .filter(|p| blacks.contains(p)).count();
      if blacks.contains(&pos) { n == 1 || n == 2 }
      else { n == 2 }
    })
    .collect()
}

fn part2(init: &HashSet<Pos>) -> usize {
  let mut blacks = init.clone();
  for _ in 0..100 { blacks = iter(&blacks); }
  blacks.len()
}

fn main() {
  let xs: Vec<Vec<Step>> =
    fs::read_to_string("input/input24.txt").unwrap()
      .lines().map(parse_steps).collect();
  let blacks = get_blacks(&xs);
  println!("Part 1: {}", blacks.len());
  println!("Part 2: {}", part2(&blacks));
}