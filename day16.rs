use std::collections::HashSet;
use std::collections::HashMap;
use std::fs;

type Entry = i32;
type Ticket = Vec<Entry>;

struct Notes {
  fields: HashMap<String, HashSet<Entry>>,
  ticket: Ticket,
  nearby: Vec<Ticket>,
}

fn parse_field(s: &str) -> (String, HashSet<Entry>) {
  let xs: Vec<&str> = s.split(": ").collect();

  let name = xs[0].to_owned();
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

fn find_assignment(notes: &Notes, valid_fields: &Vec<HashSet<&String>>, &mut acc: Vec<&String>, idx: usize) -> bool {
  let num_fields = notes.fields().len();

  false
}

fn part2(notes: &Notes) -> i32 {
  let v = valid_entries(notes);
  let valid_nearby: Vec<&Ticket> = notes.nearby.iter().filter(|t| t.iter().all(|f| v.contains(f))).collect();

  let num_fields = valid_nearby[0].len();
  assert_eq!(num_fields, notes.fields.len());

  let mut valid_fields = vec![notes.fields.keys().collect::<HashSet<&String>>(); num_fields];
  for t in valid_nearby.iter() {
    for i in 0..num_fields {
      for f in notes.fields.keys() {
        if !notes.fields[f].contains(&t[i]) {
          valid_fields[i].remove(f);
        }
      }
    }
  }

  let field_names = find_assignment(notes, valid_fields);

  0
}

fn main() {
  let data = fs::read_to_string("input/input16.txt").unwrap();
  let notes = parse_notes(&data);

  println!("Part 1: {}", part1(&notes));
}