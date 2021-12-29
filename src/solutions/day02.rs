use std::fmt::Display;

use crate::error::AocResult;

fn read_or_die(fname: &str) -> String { std::fs::read_to_string(fname).unwrap() }

#[derive(Debug)]
struct Point {
  x: i32,
  y: i32,
}

impl std::ops::AddAssign<Point> for Point {
  fn add_assign(&mut self, rhs: Point) {
    self.x += rhs.x;
    self.y += rhs.y;
  }
}

macro_rules! pt {
  ($l: expr, $r: expr) => {
    Point { x: ($l), y: ($r) }
  };
}

fn do_parse(s: &str) -> (&str, i32) {
  match s.split(' ').collect::<Vec<&str>>()[..] {
    [cmd, i] => (cmd, i.parse::<i32>().unwrap()),
    _ => panic!("Parse failed"),
  }
}

fn part1(fname: &str) -> i32 {
  let mut r = pt!(0, 0);
  read_or_die(fname)
    .lines()
    .map(do_parse)
    .for_each(|p| match p {
      ("forward", value) => r += pt!(value, 0),
      ("down", value) => r += pt!(0, value),
      ("up", value) => r += pt!(0, -value),
      _ => panic!("Unknown command"),
    });
  r.x * r.y
}

fn part2(fname: &str) -> i32 {
  let mut pos = pt!(0, 0);
  let mut aim = 0;
  read_or_die(fname)
    .lines()
    .map(do_parse)
    .for_each(|p| match p {
      ("forward", value) => pos += pt!(value, aim * value),
      ("down", value) => aim += value,
      ("up", value) => aim -= value,
      _ => panic!("Invalid command"),
    });
  pos.x * pos.y
}

pub fn run(fname: &str) -> AocResult<(impl Display, impl Display)> {
  Ok((part1(fname), part2(fname)))
}
