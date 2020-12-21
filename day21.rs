use std::collections::HashSet;
use std::collections::HashMap;
use std::fs;

struct Food<'a> {
  ingredients: HashSet<&'a str>,
  allergens: Vec<&'a str>,
}

fn parse_food(s: &str) -> Food {
  let bidx = s.find('(').unwrap();
  let ingredients = (&s[..bidx-1]).split_whitespace().collect();
  let allergens = (&s[bidx+10..s.len()-1]).split(", ").collect();
  Food { ingredients, allergens }
}

fn solve<'a>(foods: &Vec<Food<'a>>) -> Option<HashMap<&'a str, &'a str>> {

  let mut possible: HashMap<&str, HashSet<&str>> = HashMap::new();

  for Food { ingredients, allergens } in foods {
    for allergen in allergens {
      possible.insert(
        allergen,
        possible.get(allergen)
          .map(|p| p.intersection(ingredients).cloned().collect())
          .unwrap_or(ingredients.clone())
      );
    }
  }

  let mut ambiguous: HashSet<&str> = possible.keys().cloned().collect();

  while let Some(&allergen) = ambiguous.iter().find(|&&t| possible[t].len()==1) {
    ambiguous.remove(allergen);
    let ingredient = *possible[allergen].iter().next().unwrap();
    for (k,v) in possible.iter_mut() {
      if *k != allergen {
        v.remove(ingredient);
      }
    }
  }

  if !ambiguous.is_empty() {
    None
  } else {
    Some(
      possible.iter()
        .map(|(k,v)| (*k, *v.iter().next().unwrap()))
        .collect()
    )
  }
}

fn part1(foods: &Vec<Food>, soln: &HashMap<&str,&str>) -> usize {
  let bad_ingredients: HashSet<&str> = soln.values().cloned().collect();
  foods.iter()
    .map(|f| f.ingredients.difference(&bad_ingredients).count())
    .sum()
}

fn part2(soln: &HashMap<&str,&str>) -> String {
  let mut items = soln.iter().collect::<Vec<_>>();
  items.sort_by_key(|(k,_)| *k);

  items.iter().map(|(_,v)| **v).collect::<Vec<_>>().join(",")
}

fn main() {
  let data = fs::read_to_string("input/input21.txt").unwrap();
  let foods: Vec<Food> = data.lines().map(parse_food).collect();

  if let Some(soln) = solve(&foods) {
    println!("Part 1: {}", part1(&foods, &soln));
    println!("Part 2: {}", part2(&soln));
  } else {
    println!("Could not solve!");
  }
}