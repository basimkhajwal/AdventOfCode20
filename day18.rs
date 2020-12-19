use std::fs;

type Int = i128;

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
enum Op {
  Add,
  Mul,
}

#[derive(Copy, Clone, Debug)]
enum Token {
  Num(Int),
  Op(Op),
  BOpen,
  BClose,
}

fn tokens(s: &str) -> Vec<Token> {
  let mut tokens = vec![];

  for w in s.split_whitespace() {

    let w_start= w.trim_start_matches('(');
    tokens.append(&mut vec![Token::BOpen; w.len()-w_start.len()]);

    let w_end = w_start.trim_end_matches(')');

    if w_end == "*" {
      tokens.push(Token::Op(Op::Mul));
    } else if w_end == "+" {
      tokens.push(Token::Op(Op::Add));
    } else if let Ok(v) = w_end.parse() {
      tokens.push(Token::Num(v))
    }

    tokens.append(&mut vec![Token::BClose; w_start.len()-w_end.len()]);
  }

  tokens
}

fn apply_op(op: Op, a: Int, b: Int) -> Int {
  match op {
    Op::Add => a + b,
    Op::Mul => a * b,
  }
}

fn part1(tokens: &Vec<Token>) -> Int {
  let mut acc = vec![0];
  let mut ops = vec![Op::Add];

  for t in tokens.iter() {
    match t {
      Token::Num(v) => {
        let i = acc.len()-1;
        acc[i] = apply_op(ops.pop().unwrap(), acc[i], *v);
      },
      Token::Op(o) => {
        ops.push(*o);
      },
      Token::BOpen => {
        acc.push(0);
        ops.push(Op::Add);
      },
      Token::BClose => {
        let i = acc.len()-2;
        acc[i] = apply_op(ops.pop().unwrap(), acc[i], acc.pop().unwrap());
      },
    }
  }

  acc.pop().unwrap()
}

fn part2(tokens: &Vec<Token>) -> Int {

  // Neat trick:
  //  Represent depth of stack as pair of numbers (a,m)
  //  where value is a * m
  let mut stack = vec![0, 1];
  let mut i = 0;

  for t in tokens {
    match t {
      Token::Num(v) => { stack[i] += *v; },
      Token::Op(Op::Add) => { },
      Token::Op(Op::Mul) => {
        stack[i+1] *= stack[i];
        stack[i] = 0;
      },
      Token::BOpen => {
        stack.push(0);
        stack.push(1);
        i += 2;
      },
      Token::BClose => {
        let m = stack.pop().unwrap();
        let a = stack.pop().unwrap();
        i -= 2;
        stack[i] += a * m;
      },
    }
  }

  stack[0] * stack[1]
}

fn main() {
  let data = fs::read_to_string("input/input18.txt").unwrap();
  let ts: Vec<Vec<Token>> = data.lines().map(tokens).collect();

  println!("Part 1: {}", ts.iter().map(part1).sum::<Int>());
  println!("Part 2: {}", ts.iter().map(part2).sum::<Int>());
}