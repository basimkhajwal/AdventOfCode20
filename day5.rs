use std::fs;

fn get_seat_id(s: &str) -> i32 {
  s.chars().fold(0, |acc, c|
    match c {
      'F' | 'L' => acc * 2,
      _         => acc * 2 + 1
    }
  )
}

fn sum_to(n: i32) -> i32 {
  n * (n + 1) / 2
}

fn main() {
  let data = fs::read_to_string("input/input5.txt").unwrap();
  let seats: Vec<i32> = data.lines().map(get_seat_id).collect();

  let min_seat = seats.iter().min().unwrap();
  let max_seat = seats.iter().max().unwrap();
  let seat_sum: i32 = seats.iter().sum();
  let missing_seat = sum_to(*max_seat) - sum_to(*min_seat-1) - seat_sum;
  
  println!("Part 1: {}", max_seat);
  println!("Part 2: {}", missing_seat);
}