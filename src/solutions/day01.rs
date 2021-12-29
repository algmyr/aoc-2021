use std::fmt::Display;

use crate::error::AocResult;

fn solve(fname: &str) -> AocResult<(i32, i32)> {
  let s = crate::utils::read_or_die(fname);
  let ints: Vec<i32> = s
    .trim()
    .lines()
    .map(|x| x.parse::<i32>().unwrap())
    .collect();
  let res1 = ints
    .windows(2)
    .map(|w| if w[1] > w[0] { 1 } else { 0 })
    .sum::<i32>();
  let three_ints: Vec<i32> = ints.windows(3).map(|x| x.iter().sum()).collect();
  let res2 = three_ints
    .windows(2)
    .map(|w| if w[1] > w[0] { 1 } else { 0 })
    .sum::<i32>();
  Ok((res1, res2))
}

pub fn run(fname: &str) -> AocResult<(impl Display, impl Display)> { solve(fname) }
