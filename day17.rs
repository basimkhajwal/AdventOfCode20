use std::collections::HashSet;
use std::fs;

type Loc = (i32, i32, i32, i32);
type Cubes = HashSet<Loc>;

fn parse_cubes(data: &str) -> Cubes {
  let mut cubes = HashSet::new();

  for (i,l) in data.lines().enumerate() {
    for (j,c) in l.chars().enumerate() {
      if c == '#' {
        cubes.insert((i as i32, j as i32, 0, 0));
      }
    }
  }

  cubes
}

fn is_neighbour(a: Loc, b: Loc) -> bool {
  [a.0 - b.0, a.1 - b.1, a.2 - b.2, a.3 - b.3]
    .iter().all(|x| x.abs() <= 1)
}

fn is_active(cubes: &Cubes, loc: Loc) -> bool {
  let num_neighbours = cubes.iter().filter(|c| is_neighbour(**c, loc)).count();

  if cubes.contains(&loc) { num_neighbours == 3 || num_neighbours == 4 }
  else { num_neighbours == 3 }
}

// A bit inefficient but does the job (~30s)
fn cycle(cubes: &Cubes, update_w: bool) -> Cubes {
  let mut new_cubes = HashSet::new();

  let inf = 100000000;
  let (a_x, a_y, a_z, a_w) =
    cubes.iter().fold(
      (inf,inf,inf,inf),
      |(x,y,z,w),&(a,b,c,d)| (x.min(a), y.min(b), z.min(c), w.min(d))
    );
  let (b_x, b_y, b_z, b_w) =
    cubes.iter().fold(
      (-inf,-inf,-inf,-inf),
      |(x,y,z,w),&(a,b,c,d)| (x.max(a), y.max(b), z.max(c), w.max(d))
    );

  let ws = if update_w { (a_w-1)..=(b_w+1) } else { 0..=0 };
  for w in ws {
    for x in (a_x-1)..=(b_x+1) {
      for y in (a_y-1)..=(b_y+1) {
        for z in (a_z-1)..=(b_z+1) {
          let l = (x, y, z, w);
          if is_active(&cubes, l) {
            new_cubes.insert(l);
          }
        }
      }
    }
  }

  new_cubes
}

fn count_n(mut cubes: Cubes, n: usize, update_w: bool) -> usize {
  for _ in 0..n { cubes = cycle(&cubes, update_w); }
  cubes.len()
}

fn main() {
  let data = fs::read_to_string("input/input17.txt").unwrap();
  let cubes = parse_cubes(&data);

  println!("Part 1: {}", count_n(cubes.clone(), 6, false));
  println!("Part 2: {}", count_n(cubes.clone(), 6, true));
}