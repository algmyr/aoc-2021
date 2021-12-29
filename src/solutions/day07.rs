use std::fmt::Display;
use std::io::BufRead;

use crate::error::{aoc_error, AocResult};

fn parse_input(fname: &str) -> AocResult<Vec<i32>> {
  let to_int = |x: &str| -> i32 { x.parse().expect("Integer parse failed") };

  let f = std::fs::File::open(fname)?;
  let mut lines = std::io::BufReader::new(f).lines();
  Ok(
    lines
      .next()
      .ok_or(aoc_error("No first line"))??
      .split(',')
      .map(to_int)
      .collect(),
  )
}

fn part1(fname: &str) -> AocResult<i32> {
  let mut vec = parse_input(fname).expect("Input reading failed.");
  vec.sort();
  let median = vec[vec.len() / 2 as usize];
  let res: i32 = vec.iter().map(|x| (median - x).abs()).sum();
  Ok(res)
}

fn part2(fname: &str) -> AocResult<i32> {
  let mut vec = parse_input(fname).expect("Input reading failed.");
  vec.sort();
  let res = (vec[0]..=vec[vec.len() - 1])
    .map(|i| {
      vec
        .iter()
        .map(|x| {
          let y = (i - x).abs();
          y * (y + 1) / 2
        })
        .sum::<i32>()
    })
    .min();
  Ok(res.unwrap())
}

pub fn run(fname: &str) -> AocResult<(impl Display, impl Display)> {
  Ok((part1(fname)?, part2(fname)?))
}
