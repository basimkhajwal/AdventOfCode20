use std::fs;
use std::collections::HashMap;
use std::collections::HashSet;

type Bag<'a> = &'a str;
type Rule<'a> = (Bag<'a>, usize);
type Rules<'a> = HashMap<Bag<'a>, Vec<Rule<'a>>>;

fn parse_bag(s: &str) -> Bag {
  s.trim().rsplitn(2, ' ').skip(1).next().unwrap()
}

fn parse_rule(s: &str) -> Rule {
  let xs: Vec<&str> = s.trim().splitn(2, ' ').collect();
  (parse_bag(xs[1]), xs[0].parse().unwrap())
}

fn parse_rules(s: &str) -> (Bag, Vec<Rule>) {
  let mut xs = s.split("contain");
  let name: Bag = parse_bag(xs.next().unwrap());
  let inner = xs.next().unwrap().trim().trim_end_matches('.');

  let rules =
    if inner == "no other bags" {
      Vec::new()
    } else {
      inner.split(",").map(parse_rule).collect()
    };

  (name, rules)
}

fn part1(rules: &Rules) -> usize {
  let mut marked = HashSet::new();
  marked.insert("shiny gold");

  loop {
    let mut changed = false;

    for c in rules.keys() {
      if !marked.contains(c) && rules[c].iter().any(|(name,_)| marked.contains(name)) {
        marked.insert(c);
        changed = true;
      }
    }

    if !changed {
      break
    }
  }

  marked.len() - 1
}

fn count_bags<'a, 'b>(rules: &Rules<'a>, cache: &'b mut HashMap<Bag<'a>, usize>, bag: Bag<'a>) -> usize {
  if cache.contains_key(bag) {
    cache[bag]
  } else {
    let res = 
      1 + rules[bag]
        .iter()
        .map(|(inner_bag, cnt)| cnt * count_bags(rules, cache, inner_bag))
        .sum::<usize>();
    cache.insert(bag, res);
    res
  } 
}

fn part2(rules: &Rules) -> usize {
  count_bags(rules, &mut HashMap::new(), "shiny gold") - 1
}

fn main() {
  let data = fs::read_to_string("input/input7.txt").unwrap();
  let rules: Rules = data.lines().map(parse_rules).collect();

  println!("Part 1: {}", part1(&rules));
  println!("Part 2: {}", part2(&rules));
}