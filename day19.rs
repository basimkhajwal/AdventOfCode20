use std::fs;
use std::str::FromStr;
use std::collections::HashMap;

type RuleId = usize;
enum Rule {
  Char(char),
  SubRules(Vec<Vec<RuleId>>),
}

impl FromStr for Rule {

  type Err = &'static str;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let s = s.trim();
    
    if s.starts_with('"') {
      return s.chars().nth(1).map(Rule::Char).ok_or("Expected character");
    }

    let res: Result<_, _> = s.split("|").map(|r|
        r.trim().split_whitespace().map(|v|
          v.parse()).collect()
      ).collect();
    
    res.map(Rule::SubRules).map_err(|_| "Integer expected")
  }
}

struct RuleEntry(RuleId, Rule);

impl FromStr for RuleEntry {

  type Err = &'static str;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let xs: Vec<&str> = s.split(":").collect();
    if xs.len() != 2 { return Err("Expected key:value") }

    xs[0].parse()
      .map_err(|_| "Expected rule ID")
      .and_then(|v: RuleId|
        xs[1].parse().map(|r: Rule| RuleEntry(v,r))
      )
  }
}

struct Rules(HashMap<RuleId, Rule>);

impl FromStr for Rules {

  type Err = &'static str;

  fn from_str(s: &str) -> Result<Self, Self::Err> {

    let entries: Result<Vec<_>, Self::Err> =
      s.lines().map(str::parse::<RuleEntry>).collect();
    
    entries.map(|v| {
      let rules = v.into_iter().map(|RuleEntry(r,v)| (r, v)).collect();
      Rules(rules)
    })
  }
}

fn match_rule<'a>(rules: &HashMap<RuleId, Rule>, id: RuleId, s: &'a str) -> Vec<&'a str> {

  match rules.get(&id) {
    Some(&Rule::Char(c)) =>
      match s.strip_prefix(c) {
        Some(t) => vec![t],
        None    => vec![],
      },
    Some(Rule::SubRules(rs)) => {
      rs.iter().flat_map(|r|
        r.iter().fold(
          vec![s],
          |acc, &r_id| acc.into_iter().flat_map(|t| match_rule(rules, r_id, t)).collect()
        )
      ).collect()
    },
    None => vec![],
  }
}

fn part1(rules: &HashMap<RuleId, Rule>, xs: &Vec<&str>) -> usize {
  xs.iter().filter(|x| match_rule(rules, 0, x).contains(&"")).count()
}

fn part2(mut rules: HashMap<RuleId, Rule>, xs: &Vec<&str>) -> usize {
  rules.insert(8, "42 | 42 8".parse().unwrap());
  rules.insert(11, "42 31 | 42 11 31".parse().unwrap());
  part1(&rules, xs)
}

fn main() {
  let data = fs::read_to_string("input/input19.txt").unwrap();
  let sections: Vec<&str> = data.split("\n\n").collect();
  let Rules(rules) = sections[0].parse().unwrap();
  let xs: Vec<&str> = sections[1].lines().collect();

  println!("Part 1: {}", part1(&rules, &xs));
  println!("Part 2: {}", part2(rules, &xs));
}