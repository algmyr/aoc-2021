use std::fmt::Display;

use itertools::Itertools;

use crate::error::AocResult;

#[allow(dead_code)]
const DEBUG: bool = true;

#[allow(unused_macros)]
macro_rules! debug {
  ($($rest:tt)*) => {
    if DEBUG {
      println!($($rest)*);
    }
  };
}

mod parsing {
  use nom::bytes::complete::{is_not, tag};
  use nom::character::complete::{multispace0, one_of, space0, space1};
  use nom::combinator::map;
  use nom::multi::separated_list1;
  use nom::sequence::{delimited, preceded, tuple};
  use nom::IResult;

  use super::Cuboid;

  fn on_off(s: &str) -> IResult<&str, bool> {
    map(delimited(space0, is_not(" "), space1), |s| {
      if s == "on" {
        true
      } else {
        false
      }
    })(s)
  }

  fn range(s: &str) -> IResult<&str, (i32, i32)> {
    let (s, (_coord, _, l, _, r)) = tuple((
      one_of("xyz"),
      tag("="),
      nom::character::complete::i32,
      tag(".."),
      nom::character::complete::i32,
    ))(s)?;
    Ok((s, (l, r)))
  }

  fn instruction(s: &str) -> IResult<&str, (bool, Option<Cuboid>)> {
    let (rem, (on_off, xrange, yrange, zrange)) = tuple((
      on_off,
      range,
      preceded(tag(","), range),
      preceded(tag(","), range),
    ))(s)?;
    Ok((
      rem,
      (
        on_off,
        Cuboid::new(
          xrange.0,
          xrange.1 + 1,
          yrange.0,
          yrange.1 + 1,
          zrange.0,
          zrange.1 + 1,
          1,
        ),
      ),
    ))
  }

  pub(crate) fn parse(s: &str) -> IResult<&str, Vec<(bool, Option<Cuboid>)>> {
    delimited(
      multispace0,
      separated_list1(tag("\n"), instruction),
      multispace0,
    )(s)
  }
}

fn parse_input(fname: &str) -> AocResult<Vec<(bool, Option<Cuboid>)>> {
  let s = std::fs::read_to_string(fname)?;
  let (rem, result) = parsing::parse(&s)?;
  assert!(rem.is_empty());
  Ok(result)
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct Cuboid {
  xmin: i32,
  xmax: i32,
  ymin: i32,
  ymax: i32,
  zmin: i32,
  zmax: i32,
  sign: i32,
}

impl Cuboid {
  fn new(
    xmin: i32,
    xmax: i32,
    ymin: i32,
    ymax: i32,
    zmin: i32,
    zmax: i32,
    sign: i32,
  ) -> Option<Self> {
    if xmax <= xmin || ymax <= ymin || zmax <= zmin {
      None
    } else {
      Some(Self { xmin, xmax, ymin, ymax, zmin, zmax, sign })
    }
  }
  fn volume(&self) -> i64 {
    ((self.xmax - self.xmin) as i64)
      * ((self.ymax - self.ymin) as i64)
      * ((self.zmax - self.zmin) as i64)
      * (self.sign as i64)
  }
  fn intersect(&self, other: &Cuboid) -> Option<Self> {
    let xmin = self.xmin.max(other.xmin);
    let xmax = self.xmax.min(other.xmax);
    let ymin = self.ymin.max(other.ymin);
    let ymax = self.ymax.min(other.ymax);
    let zmin = self.zmin.max(other.zmin);
    let zmax = self.zmax.min(other.zmax);
    Self::new(xmin, xmax, ymin, ymax, zmin, zmax, -self.sign * other.sign)
  }
}

struct CuboidCollection {
  cuboids: Vec<Cuboid>,
  volume: i64,
}

impl CuboidCollection {
  fn add(&mut self, cuboid: Cuboid, is_add: bool) {
    let mut intersecting_cuboids = vec![];
    for existing_cuboid in &self.cuboids {
      if let Some(intersection) = existing_cuboid.intersect(&cuboid) {
        self.volume += intersection.volume();
        intersecting_cuboids.push(intersection);
      }
    }
    self.cuboids.extend(intersecting_cuboids);
    if is_add {
      self.volume += cuboid.volume();
      self.cuboids.push(cuboid);
    }
  }
}

fn solve(instructions: Vec<(bool, Option<Cuboid>)>) -> i64 {
  let mut cc = CuboidCollection { cuboids: vec![], volume: 0 };
  for (value, cuboid) in instructions {
    if let Some(cuboid) = cuboid {
      if value {
        cc.add(cuboid, true);
      } else {
        cc.add(cuboid, false);
      }
    }
  }
  cc.volume
}

fn part1(fname: &str) -> AocResult<i64> {
  let bounding_box = Cuboid::new(-50, 51, -50, 51, -50, 51, -1).unwrap();
  let instructions = parse_input(fname)?
    .into_iter()
    .map(|(value, cuboid)| (value, cuboid.and_then(|x| x.intersect(&bounding_box))))
    .collect_vec();
  Ok(solve(instructions))
}

fn part2(fname: &str) -> AocResult<i64> {
  let instructions = parse_input(fname)?;
  Ok(solve(instructions))
}

pub fn run(fname: &str) -> AocResult<(impl Display, impl Display)> {
  Ok((part1(fname)?, part2(fname)?))
}
