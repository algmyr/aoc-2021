use std::fmt::Display;

use crate::error::AocResult;

fn parse_input(fname: &str) -> AocResult<Vec<String>> {
  let content = std::fs::read_to_string(fname)?;
  Ok(content.lines().map(|x| x.to_owned()).collect())
}

fn is_closing(c: char) -> bool {
  match c {
    ')' | ']' | '}' | '>' => true,
    _ => false,
  }
}

fn matching_open(c: char) -> char {
  match c {
    ')' => '(',
    ']' => '[',
    '}' => '{',
    '>' => '<',
    _ => panic!("Not a closing brace"),
  }
}

//

fn error_score(c: char) -> i32 {
  match c {
    ')' => 3,
    ']' => 57,
    '}' => 1197,
    '>' => 25137,
    _ => panic!("Not a closing brace"),
  }
}

fn part1(fname: &str) -> AocResult<i32> {
  let lines = parse_input(fname)?;

  let res = lines
    .iter()
    .map(|v| {
      // Typical approach stack to track balanced parens
      let mut stack = vec![];
      for c in v.chars() {
        if is_closing(c) && !stack.is_empty() {
          if *stack.last().unwrap() == matching_open(c) {
            stack.pop();
          } else {
            return error_score(c);
          }
        } else {
          stack.push(c);
        }
      }
      0
    })
    .sum();
  Ok(res)
}

fn completion_score(c: char) -> i64 {
  match c {
    '(' => 1,
    '[' => 2,
    '{' => 3,
    '<' => 4,
    _ => panic!("Not an opening brace"),
  }
}

fn part2(fname: &str) -> AocResult<i64> {
  let lines = parse_input(fname)?;

  let mut scores: Vec<i64> = lines
    .iter()
    .filter_map(|v| {
      // Typical stack approach to track balanced parens
      let mut stack: Vec<char> = vec![];
      for c in v.chars() {
        if !is_closing(c) || stack.is_empty() {
          stack.push(c);
          continue;
        }
        if *stack.last().unwrap() == matching_open(c) {
          stack.pop();
        } else {
          return None;
        }
      }

      Some(
        stack
          .into_iter()
          .rev()
          .map(|x| completion_score(x))
          .fold(0, |accu, el| accu * 5 + el),
      )
    })
    .collect();

  let middle = scores.len() / 2;
  let (_, median, _) = scores.select_nth_unstable(middle);
  Ok(*median)
}

pub fn run(fname: &str) -> AocResult<(impl Display, impl Display)> {
  Ok((part1(fname)?, part2(fname)?))
}
