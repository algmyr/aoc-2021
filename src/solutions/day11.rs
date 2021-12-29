use std::fmt::Display;

use itertools::Itertools;

use crate::error::AocResult;

fn parse_input(fname: &str) -> AocResult<Vec<Vec<i8>>> {
  let content = std::fs::read_to_string(fname)?;
  Ok(
    content
      .lines()
      .map(|x| x.chars().map(|e| e.to_digit(10).unwrap() as i8).collect())
      .collect(),
  )
}

fn sim1(grid: &mut Vec<Vec<i8>>) -> i32 {
  let h = grid.len() as i32;
  let w = grid[0].len() as i32;
  let mut n_flashes = 0;

  let mut burst = vec![];
  for y in 0..h {
    for x in 0..w {
      let v = &mut grid[y as usize][x as usize];
      *v += 1;
      if *v > 9 {
        burst.push((y, x));
        *v = -100;
      }
    }
  }

  while !burst.is_empty() {
    let (y, x) = burst.pop().unwrap();
    n_flashes += 1;

    let neighbors = (-1..=1)
      .map(|i| (-1..=1).map(|j| (i, j)).collect_vec())
      .flatten()
      .filter_map(|(i, j)| {
        if i == 0 && j == 0 {
          None
        } else {
          Some((y + i, x + j))
        }
      })
      .filter(|&(i, j)| 0 <= i && i < h && 0 <= j && j < w)
      .collect_vec();

    for (ny, nx) in neighbors {
      let v = &mut grid[ny as usize][nx as usize];
      *v += 1;
      if *v > 9 {
        burst.push((ny, nx));
        *v = -100;
      }
    }
  }

  for y in 0..h {
    for x in 0..w {
      let v = &mut grid[y as usize][x as usize];
      *v = (*v).max(0);
    }
  }

  n_flashes
}

fn part1(fname: &str) -> AocResult<i32> {
  let mut grid = parse_input(fname)?;
  Ok((0..100).map(|_| sim1(&mut grid)).sum())
}

fn part2(fname: &str) -> AocResult<i32> {
  let mut grid = parse_input(fname)?;
  let mut step = 0;
  loop {
    step += 1;
    if sim1(&mut grid) == 100 {
      break;
    }
  }
  Ok(step)
}

pub fn run(fname: &str) -> AocResult<(impl Display, impl Display)> {
  Ok((part1(fname)?, part2(fname)?))
}
