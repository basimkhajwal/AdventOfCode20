use std::fs;

#[derive(Clone, Copy)]
enum Dir { N, E, S, W }

enum Move {
  D(Dir, i32), // direction
  T(i32), // turn
  F(i32), // forward
}

fn parse_move(s: &str) -> Move {
  let v = (&s[1..]).parse().unwrap();
  match s.chars().next().unwrap() {
    'N' => Move::D(Dir::N, v),
    'E' => Move::D(Dir::E, v),
    'S' => Move::D(Dir::S, v),
    'W' => Move::D(Dir::W, v),
    'F' => Move::F(v),
    'L' => Move::T(v),
    _   => Move::T(-v),
  }
}

type Pos = (i32, i32);

fn add(p1: Pos, p2: Pos) -> Pos { (p1.0 + p2.0, p1.1 + p2.1) }

fn dir_delta(d: Dir) -> Pos {
  match d {
    Dir::N => (0, 1),
    Dir::S => (0, -1),
    Dir::E => (1, 0),
    Dir::W => (-1, 0),
  }
}

fn left(d: Dir) -> Dir {
  match d {
    Dir::E => Dir::N,
    Dir::S => Dir::E,
    Dir::W => Dir::S,
    Dir::N => Dir::W,
  }
}

fn repeat<A, F : Fn(A) -> A>(f: F, n: i32, mut x: A) -> A {
  for _ in 0..n { x = f(x) }
  x
}

fn move_dir(p: Pos, d: Pos, v: i32) -> Pos { repeat(|p| add(p, d), v, p) }

fn part1(xs: &Vec<Move>) -> i32 {
  let mut pos = (0, 0);
  let mut d = Dir::E;

  for m in xs {
    match m {
      Move::D(d, v) => pos = move_dir(pos, dir_delta(*d), *v),
      Move::T(a) => d = repeat(left, (4 + (a / 90)) % 4, d),
      Move::F(v) => pos = move_dir(pos, dir_delta(d), *v),
    }
  }

  i32::abs(pos.0) + i32::abs(pos.1)
}

fn part2(xs: &Vec<Move>) -> i32 {
  let mut pos = (0, 0);
  let mut marker = (10, 1);

  for m in xs {
    match m {
      Move::D(d, v) => marker = move_dir(marker, dir_delta(*d), *v),
      Move::T(a) => marker = repeat(|(x,y)| (-y,x), (4 + (a / 90)) % 4, marker),
      Move::F(v) => pos = move_dir(pos, marker, *v),
    }
  }

  i32::abs(pos.0) + i32::abs(pos.1)
}

fn main() {
  let xs: Vec<Move> =
    fs::read_to_string("input/input12.txt")
      .unwrap().lines()
      .map(parse_move).collect();
  println!("Part 1: {}", part1(&xs));
  println!("Part 2: {}", part2(&xs));
}