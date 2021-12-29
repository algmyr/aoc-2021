use std::fmt::Display;

use itertools::Itertools;

use crate::error::{aoc_error, AocResult};

fn parse_input(fname: &str) -> AocResult<(Vec<(i32, i32)>, Vec<(char, i32)>)> {
  let content = std::fs::read_to_string(fname)?;
  let sections = content.split("\n\n").collect_vec();

  if sections.len() != 2 {
    return Err(aoc_error("Wrong number of sections"));
  }

  let points: Vec<(i32, i32)> = sections[0]
    .lines()
    .map(|s| {
      let parts = s
        .split(",")
        .map(|x| x.parse::<i32>().unwrap())
        .collect_vec();
      (parts[0], parts[1])
    })
    .collect_vec();

  let fold: Vec<(char, i32)> = sections[1]
    .lines()
    .map(|s| {
      let parts = s.split(" ").last().unwrap().split("=").collect_vec();
      (
        parts[0].chars().next().unwrap(),
        parts[1].parse::<i32>().unwrap(),
      )
    })
    .collect_vec();

  Ok((points, fold))
}

fn solve(fname: &str) -> AocResult<(usize, String)> {
  let (mut points, instructions) = parse_input(fname)?;

  let mut res1 = None;

  let mut first = true;
  for inst in instructions {
    match inst {
      ('x', val) => {
        points = points
          .into_iter()
          .map(|(x, y)| (if x <= val { x } else { val - (x - val) }, y))
          .unique()
          .collect_vec()
      }
      ('y', val) => {
        points = points
          .into_iter()
          .map(|(x, y)| (x, if y <= val { y } else { val - (y - val) }))
          .unique()
          .collect_vec()
      }
      _ => panic!("AAA"),
    }

    if first {
      res1 = Some(points.len());
      first = false;
    }
  }

  let res2 = "\n".to_owned()
    + &(0..7)
      .map(|y| {
        (0..40)
          .map(|x| if points.contains(&(x, y)) { '#' } else { ' ' })
          .join("")
      })
      .join("\n");

  Ok((res1.unwrap(), res2))
}

pub fn run(fname: &str) -> AocResult<(impl Display, impl Display)> { solve(fname) }
