use std::collections::HashMap;
use std::collections::HashSet;
use std::fs;

type TileId = i128;
type Tile = Vec<Vec<char>>;
type Tiles = HashMap<TileId, Tile>;

fn parse_tile(s: &str) -> (TileId, Tile) {
  let xs: Vec<&str> = s.lines().collect();

  let id = xs[0][5..xs[0].len()-1].parse().unwrap();
  let tile = xs[1..].iter().map(|x| x.chars().collect()).collect();

  (id, tile)
}

fn left_rot(t: &Tile) -> Tile {
  let m = t.len();
  let mut s = vec![vec!['#'; m]; m];

  for i in 0..m {
    for j in 0..m {
      s[i][j] = t[j][m-1-i];
    }
  }

  s
}

fn flip(t: &Tile) -> Tile {
  let m = t.len();
  let mut s = vec![vec!['#'; m]; m];

  for i in 0..m {
    for j in 0..m {
      s[i][j] = t[i][m-1-j];
    }
  }

  s
}

fn get_rots(t: &Tile) -> Vec<Tile> {
  let mut rs = vec![t.clone()];
  for i in 0..3 {
    rs.push(left_rot(&rs[i]));
  }
  rs
}

fn get_tiles(t: &Tile) -> Vec<Tile> {
  let mut vs = get_rots(t);
  vs.append(&mut get_rots(&flip(t)));
  vs
}

fn match_right(a: &Tile, b: &Tile) -> bool {
  let m = a.len();
  for i in 0..m {
    if a[i][m-1] != b[i][0] {
      return false;
    }
  }
  true
}

fn match_below(a: &Tile, b: &Tile) -> bool {
  let m = a.len();
  for i in 0..m {
    if a[m-1][i] != b[0][i] {
      return false;
    }
  }
  true
}

fn search(tiles: &Tiles, n: usize, acc: &mut Vec<Vec<(TileId, Tile)>>, av: &mut HashSet<TileId>) -> bool {

  if acc.len() == n && acc[n-1].len() == n {
    return true;
  }

  let add_row = acc.len() == 0 || acc[acc.len()-1].len() == n;
  if add_row { acc.push(vec![]); }

  let i = acc.len()-1;
  let j = acc[i].len();

  for t_id in av.iter().cloned().collect::<Vec<_>>() {
    for tile in get_tiles(&tiles[&t_id]) {
      if j > 0 && !match_right(&acc[i][j-1].1, &tile) { continue; }
      if i > 0 && !match_below(&acc[i-1][j].1, &tile) { continue; }
      acc[i].push((t_id, tile));
      av.remove(&t_id);
      if search(tiles, n, acc, av) {
        return true;
      }
      av.insert(t_id);
      acc[i].pop();
    }
  }

  if add_row { acc.pop(); }

  false
}

fn merge_tiles(acc: &Vec<Vec<(TileId, Tile)>>) -> Tile {
  let n = acc.len();
  let m = acc[0][0].1.len();
  let k = n * (m - 2);

  let mut t = vec![vec!['Q'; k]; k];

  for i in 0..n {
    for j in 0..n {
      for r in 0..m-2 {
        for c in 0..m-2 {
          t[i*(m-2)+r][j*(m-2)+c] = acc[i][j].1[r+1][c+1];
        }
      }
    }
  }

  t
}

fn count_monsters(t: &Tile) -> usize {

  let monster: Tile = [
    "                  # ",
    "#    ##    ##    ###",
    " #  #  #  #  #  #   ",
  ].iter().map(|s| s.chars().collect()).collect();
  let n = t.len();

  let mut is_hash = vec![vec![false; n]; n];

  for i in 0..n {
    for j in 0..n {
      if t[i][j] == '#' {
        is_hash[i][j] = true;
      }
    }
  }

  for i in 0..=n-monster.len() {
    for j in 0..n-monster[0].len() {
      let mut valid = true;
      for r in 0..monster.len() {
        for c in 0..monster[0].len() {
          if monster[r][c] == '#' && t[i+r][j+c] != '#' {
            valid = false;
          }
        }
      }
      if valid {
        for r in 0..monster.len() {
          for c in 0..monster[0].len() {
            if monster[r][c] == '#' {
              is_hash[i+r][j+c] = false;
            }
          }
        }
      }
    }
  }

  is_hash
    .iter()
    .map(|r| r.iter().filter(|c| **c).count())
    .sum()
}

fn main() {
  let data = fs::read_to_string("input/input20.txt").unwrap();
  let tiles: Tiles =
    data.split("\n\n")
      .map(parse_tile)
      .collect();
  let n = (tiles.len() as f32).sqrt().round() as usize;
  
  let mut acc = vec![];
  if !search(&tiles, n, &mut acc, &mut tiles.keys().cloned().collect()) {
    println!("Failed!");
    return;
  }
  let ans1 = acc[0][0].0 * acc[0][n-1].0 * acc[n-1][0].0 * acc[n-1][n-1].0;
  println!("Part 1: {}", ans1);

  let combined = merge_tiles(&acc);
  
  let ans2 = get_tiles(&combined).iter().map(count_monsters).min().unwrap();
  println!("Part 2: {}", ans2);
}