use std::collections::HashSet;
use std::collections::HashMap;
use std::fs;

type Entry = i32;
type Ticket = Vec<Entry>;

struct Notes<'a> {
  fields: HashMap<&'a str, HashSet<Entry>>,
  ticket: Ticket,
  nearby: Vec<Ticket>,
}

fn parse_field<'a>(s: &'a str) -> (&'a str, HashSet<Entry>) {
  let xs: Vec<&str> = s.split(": ").collect();

  let name = xs[0];
  let mut values = HashSet::new();
  for r in xs[1].split(" or ") {
    let ys: Vec<Entry> = r.split("-").map(|x| x.parse().unwrap()).collect();
    values.extend(ys[0]..=ys[1]);
  }

  (name, values)
}

fn parse_ticket(s: &str) -> Ticket {
  s.split(",").map(|x| x.parse().unwrap()).collect()
}

fn parse_notes(data: &str) -> Notes {
  let xs: Vec<&str> = data.split("\n\n").collect();

  let fields = xs[0].split("\n").map(parse_field).collect();
  let ticket = parse_ticket(xs[1].lines().skip(1).next().unwrap());
  let nearby = xs[2].lines().skip(1).map(parse_ticket).collect();

  Notes { fields, ticket, nearby }
}

fn valid_entries(notes: &Notes) -> HashSet<i32> {
  notes.fields
    .values()
    .fold(
      HashSet::new(),
      |mut acc, vs| { acc.extend(vs); acc }
    )
}

fn part1(notes: &Notes) -> i32 {
  let v = valid_entries(notes);
  let mut tot = 0;

  for f in notes.nearby.iter().flat_map(|t| t.iter()) {
    if !v.contains(f) {
      tot += f;
    }
  }

  tot
}

fn find_assignment<'a>(notes: &Notes, valid_fields: &Vec<HashSet<&'a str>>, acc: &mut Vec<(&'a str, usize)>) -> bool {
  let num_fields = notes.fields.len();
  if acc.len() == num_fields {
    return true
  }

  let used_indices: HashSet<usize> = acc.iter().map(|(_,i)| *i).collect();
  let used_fields: HashSet<&'a str> = acc.iter().map(|(s,_)| *s).collect();

  let mut missing: Vec<(usize, HashSet<&'a str>)> =
    (0..num_fields)
      .filter(|x| !used_indices.contains(x))
      .map(|x| (x, valid_fields[x].difference(&used_fields).cloned().collect()))
      .collect();
  missing.sort_by_key(|(_,vs)| vs.len());

  for (i,vs) in missing.iter() {
    for &f in vs {
      acc.push((f,*i));
      if find_assignment(notes, valid_fields, acc) {
        return true;
      }
      acc.pop();
    }
  }


  false
}

fn part2(notes: &Notes) -> i128 {
  let v = valid_entries(notes);
  let valid_nearby: Vec<&Ticket> = notes.nearby.iter().filter(|t| t.iter().all(|f| v.contains(f))).collect();

  let num_fields = valid_nearby[0].len();
  assert_eq!(num_fields, notes.fields.len());

  let mut valid_fields = vec![notes.fields.keys().cloned().collect::<HashSet<&str>>(); num_fields];
  for t in valid_nearby.iter() {
    for i in 0..num_fields {
      for f in notes.fields.keys() {
        if !notes.fields[f].contains(&t[i]) {
          valid_fields[i].remove(f);
        }
      }
    }
  }

  let mut acc = vec![];
  if !find_assignment(notes, &valid_fields, &mut acc) {
    println!("ERROR: No valid assignment found!");
    return -1;
  }

  let mut prod: i128 = 1;
  for (f, i) in acc.iter() {
    if f.starts_with("departure") {
      prod *= notes.ticket[*i] as i128;
    }
  }

  prod
}

fn main() {
  let data = fs::read_to_string("input/input16.txt").unwrap();
  let notes = parse_notes(&data);

  println!("Part 1: {}", part1(&notes));
  println!("Part 2: {}", part2(&notes));
}