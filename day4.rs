use std::fs;
use std::collections::HashMap;
use std::collections::HashSet;

type Passport<'a> = HashMap<&'a str, &'a str>;

fn parse_passport(pp: &str) -> Passport  {
  let mut v = HashMap::new();

  for entry in pp.split_whitespace() {
    let xs: Vec<_> = entry.split(":").collect();
    v.insert(xs[0], xs[1]);
  }

  v
}

fn check_fields(pp: &Passport) -> bool {
  let fields = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];
  fields.iter().all(|f| pp.contains_key(f))
}

fn int_in_range(s: &str, l: i32, u: i32) -> bool {
  s.parse().map(|v| l <= v && v <= u).unwrap_or(false)
}


fn check_data(pp: &Passport) -> bool {
  let fields = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];

  let conditions: [fn(&str) -> bool; 7] = [
    |x| int_in_range(x, 1920, 2002),
    |x| int_in_range(x, 2010, 2020),
    |x| int_in_range(x, 2020, 2030),
    |x|
      x.strip_suffix("cm").map(|v| int_in_range(v, 150, 193)).unwrap_or(false) ||
      x.strip_suffix("in").map(|v| int_in_range(v, 59, 76)).unwrap_or(false),
    |x| x.strip_prefix("#").map(|v|
      v.chars()
       .collect::<HashSet<_>>()
       .is_subset(&"0123456789abcdef".chars().collect())
      ).unwrap_or(false),
    |x| ["amb","blu","brn","gry","grn","hzl","oth"].contains(&x),
    |x| x.len() == 9 && x.parse::<i32>().is_ok(),
  ];

  fields.iter().zip(conditions.iter()).all(|(f,c)| c(pp[f]))
}

fn main() {
  let data = fs::read_to_string("input/input4.txt").unwrap();
  //let xs: Vec<_> = data.split("\n\n").map(|x| parse_passport(x)).collect();
  let xs: Vec<_> = data.split("\n\n").map(parse_passport).collect();

  let ans1 = xs.iter().filter(|p| check_fields(p)).count();
  println!("{}", ans1);

  let ans2 = xs.iter().filter(|p| check_fields(p) && check_data(p)).count();
  println!("{}", ans2);
}