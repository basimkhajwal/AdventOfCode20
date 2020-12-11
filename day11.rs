use std::fs;

#[derive(Clone, Copy, PartialEq, Eq)]
enum Location {
  Floor,
  Empty,
  Occupied
}

type Seats = Vec<Vec<Location>>;

type NewFn = fn(&Seats, usize, usize) -> (Location, bool);

fn parse_loc(ch: char) -> Location {
  match ch {
    '.' => Location::Floor,
    'L' => Location::Empty,
    _   => Location::Occupied,
  }
}

fn parse_seats(data: &str) -> Seats {
  data
    .lines()
    .map(|l| l.chars().map(parse_loc).collect())
    .collect()
}

fn h(seats: &Seats) -> usize { seats.len() }
fn w(seats: &Seats) -> usize { seats[0].len() }

fn in_range<A : Ord>(x: A, l: A, u: A) -> bool {
  l <= x && x < u
}

fn step(seats: &Seats, new_fn: NewFn) -> (Seats, bool) {
  let mut new_seats = vec![];
  let mut changed = false;

  for j in 0..h(seats) {
    let mut new_row = vec![];
    for i in 0..w(seats) {
      let (nc, change) = new_fn(seats, j, i);
      changed = changed || change;
      new_row.push(nc);
    }
    new_seats.push(new_row);
  }

  (new_seats, changed)
}

fn count_occupied(seats: &Seats) -> usize {
  seats.iter()
    .map(|row| row.iter().filter(|l| **l == Location::Occupied).count())
    .sum()
}

fn count_equilibrium(mut seats: Seats, new_fn: NewFn) -> usize {
  loop {
    let (new_seats, changed) = step(&seats, new_fn);
    if !changed { break }
    seats = new_seats;
  }
  count_occupied(&seats)
}

fn new_loc_1(seats: &Seats, j: usize, i: usize) -> (Location, bool) {
  let h = h(seats) as i32;
  let w = w(seats) as i32;
  let dirs: [(i32,i32); 8] = [(-1,-1),(-1,0),(-1,1),(0,-1),(0,1),(1,-1),(1,0),(1,1)];
  let neighbours =
    dirs.iter()
      .map(|(dr,dc)| ((j as i32) + dr, (i as i32) + dc))
      .filter(|(r,c)|
        in_range(*r, 0, h) &&
        in_range(*c, 0, w) &&
        seats[(*r) as usize][(*c) as usize] == Location::Occupied
      )
      .count();

  match seats[j][i] {
    Location::Empty if neighbours == 0 => (Location::Occupied, true),
    Location::Occupied if neighbours >= 4 => (Location::Empty, true),
    v => (v, false),
  }
}

fn scan_dir(seats: &Seats, j: usize, i: usize, d: (i32, i32)) -> bool {
  let h = h(seats) as i32;
  let w = w(seats) as i32;
  let mut r = j as i32;
  let mut c = i as i32;

  loop {
    r += d.0;
    c += d.1;
    if !in_range(r, 0, h) || !in_range(c, 0, w) ||
       seats[r as usize][c as usize] != Location::Floor
       { break }
  }

  in_range(r, 0, h) && in_range(c, 0, w) &&
    seats[r as usize][c as usize] == Location::Occupied
}

fn new_loc_2(seats: &Seats, j: usize, i: usize) -> (Location, bool) {
  let dirs: [(i32,i32); 8] = [(-1,-1),(-1,0),(-1,1),(0,-1),(0,1),(1,-1),(1,0),(1,1)];
  let neighbours =
    dirs.iter().filter(|d| scan_dir(seats, j, i, **d)).count();

  match seats[j][i] {
    Location::Empty if neighbours == 0 => (Location::Occupied, true),
    Location::Occupied if neighbours >= 5 => (Location::Empty, true),
    v => (v, false),
  }
}

fn main() {
  let data = fs::read_to_string("input/input11.txt").unwrap();
  let seats = parse_seats(&data);

  println!("Part 1: {}", count_equilibrium(seats.clone(), new_loc_1));
  println!("Part 2: {}", count_equilibrium(seats.clone(), new_loc_2));
}