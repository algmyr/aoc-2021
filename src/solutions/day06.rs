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

fn part1(fname: &str) -> AocResult<usize> {
  let mut vec = parse_input(fname)?;
  for _ in 0..80 {
    let zeroes = vec.iter().filter(|&&x| x == 0).count();
    vec = vec
      .into_iter()
      .map(|x| if x > 0 { x - 1 } else { 6 })
      .collect();
    vec.extend(std::iter::repeat(8).take(zeroes));
  }
  Ok(vec.len())
}

fn part2(fname: &str) -> AocResult<i64> {
  let mut counts = [0, 0, 0, 0, 0, 0, 0, 0, 0];
  let init = parse_input(fname)?;
  for e in init {
    counts[e as usize] += 1;
  }
  for _ in 0..256 {
    let zeroes = counts[0];
    counts[0] = counts[1];
    counts[1] = counts[2];
    counts[2] = counts[3];
    counts[3] = counts[4];
    counts[4] = counts[5];
    counts[5] = counts[6];
    counts[6] = counts[7] + zeroes;
    counts[7] = counts[8];
    counts[8] = zeroes;
  }
  Ok(counts.iter().sum::<i64>())
}

pub fn run(fname: &str) -> AocResult<(impl Display, impl Display)> {
  Ok((part1(fname)?, part2(fname)?))
}
