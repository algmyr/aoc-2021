use std::fmt::Display;
use std::hash::Hash;
use std::iter;

use hashbrown::HashMap;
use itertools::Itertools;

use crate::error::{aoc_error, AocResult};

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Ord, Eq)]
struct PointType {
  x: i32,
  y: i32,
  z: i32,
}

impl PointType {
  fn new(x: i32, y: i32, z: i32) -> Self { Self { x, y, z } }
  fn l1_dist(&self, other: &Self) -> i32 {
    let diff = *self - *other;
    diff.x.abs() + diff.y.abs() + diff.z.abs()
  }
  fn sq_dist(&self, other: &Self) -> i32 {
    let diff = *self - *other;
    diff.x.pow(2) + diff.y.pow(2) + diff.z.pow(2)
  }
}

impl std::ops::Add for PointType {
  type Output = PointType;

  fn add(self, rhs: Self) -> Self::Output {
    PointType::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
  }
}

impl std::ops::Sub for PointType {
  type Output = PointType;

  fn sub(self, rhs: Self) -> Self::Output {
    PointType::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
  }
}

impl Hash for PointType {
  fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
    (self.x as i16).hash(state);
    (self.y as i16).hash(state);
    (self.z as i16).hash(state);
  }
}

fn parse_input(fname: &str) -> AocResult<Vec<Vec<PointType>>> {
  fn parse_triple(triple: &str) -> AocResult<PointType> {
    if let Some((x, y, z)) = triple.split(",").map(|x| x.parse::<i32>()).collect_tuple() {
      Ok(PointType::new(x?, y?, z?))
    } else {
      Err(aoc_error("Wrong number of elements in triple."))
    }
  }
  fn parse_section(section: &str) -> AocResult<Vec<PointType>> {
    let mut lines = section.lines();
    lines.next();
    let res: AocResult<Vec<PointType>> = lines.map(parse_triple).collect();
    res.map(|mut v| {
      v.sort();
      v
    })
  }
  let s = std::fs::read_to_string(fname)?;
  s.split("\n\n").map(parse_section).collect()
}

fn closeness(truth: &[PointType], other: &[PointType]) -> Option<(PointType, Vec<PointType>)> {
  let mappings: [fn(PointType) -> PointType; 24] = [
    |PointType { x, y, z }| PointType::new(x, y, z),
    |PointType { x, y, z }| PointType::new(-x, -y, z),
    |PointType { x, y, z }| PointType::new(-x, y, -z),
    |PointType { x, y, z }| PointType::new(x, -y, -z),
    |PointType { x, y, z }| PointType::new(y, z, x),
    |PointType { x, y, z }| PointType::new(-y, -z, x),
    |PointType { x, y, z }| PointType::new(-y, z, -x),
    |PointType { x, y, z }| PointType::new(y, -z, -x),
    |PointType { x, y, z }| PointType::new(z, x, y),
    |PointType { x, y, z }| PointType::new(-z, -x, y),
    |PointType { x, y, z }| PointType::new(-z, x, -y),
    |PointType { x, y, z }| PointType::new(z, -x, -y),
    |PointType { x, y, z }| PointType::new(y, x, -z),
    |PointType { x, y, z }| PointType::new(-y, -x, -z),
    |PointType { x, y, z }| PointType::new(-y, x, z),
    |PointType { x, y, z }| PointType::new(y, -x, z),
    |PointType { x, y, z }| PointType::new(z, y, -x),
    |PointType { x, y, z }| PointType::new(-z, -y, -x),
    |PointType { x, y, z }| PointType::new(-z, y, x),
    |PointType { x, y, z }| PointType::new(z, -y, x),
    |PointType { x, y, z }| PointType::new(x, z, -y),
    |PointType { x, y, z }| PointType::new(-x, -z, -y),
    |PointType { x, y, z }| PointType::new(-x, z, y),
    |PointType { x, y, z }| PointType::new(x, -z, y),
  ];

  for i in 0..mappings.len() {
    // The first set is assumed as ground truth.
    // Try all rotations of the other set.
    let cand = other.iter().map(|&x| mappings[i](x)).collect_vec();

    // Compute all differences of points and see if get enough equal ones.
    // (This could technically produce false positives,
    //  though not likely for large varied data.)
    let mut counts = HashMap::with_capacity(1 << 11);
    truth
      .iter()
      .cartesian_product(cand.iter())
      .map(|(&p1, &p2)| p1 - p2)
      .for_each(|pt| *counts.entry(pt).or_insert(0) += 1);
    if let Some((off, maxi)) = counts.into_iter().max_by(|(_, c1), (_, c2)| c1.cmp(c2)) {
      if maxi >= 12 {
        return Some((off, cand));
      }
    }
  }
  None
}

fn distances(points: &[PointType]) -> Vec<i32> {
  let mut dists = vec![];
  for i in 0..points.len() {
    for j in i + 1..points.len() {
      dists.push(points[i].sq_dist(&points[j]));
    }
  }
  dists.sort();
  dists
}

fn solve(fname: &str) -> AocResult<(i32, i32)> {
  let mut input = parse_input(fname)?;

  let input_distances = input.iter().map(|x| distances(x)).collect_vec();

  fn quick_check(distances1: &[i32], distances2: &[i32]) -> i32 {
    let mut common = 0;
    let mut it = distances2.iter();
    let mut d2 = {
      if let Some(value) = it.next() {
        value
      } else {
        return 0;
      }
    };
    for d1 in distances1 {
      'outer: loop {
        if d2 == d1 {
          common += 1;
          break;
        }
        if d2 > d1 {
          break;
        }
        match it.next() {
          Some(value) => d2 = value,
          None => break 'outer,
        }
      }
    }
    common
  }

  let mut locations = HashMap::new();
  locations.insert(0, PointType::new(0, 0, 0));

  let mut stack = vec![0];
  let mut seen = iter::repeat(false).take(input.len()).collect_vec();
  while let Some(cur) = stack.pop() {
    seen[cur] = true;
    for cand_neigh in 0..input.len() {
      if seen[cand_neigh]
        || quick_check(&input_distances[cur], &input_distances[cand_neigh]) < 12 * 11 / 2
      {
        continue;
      }
      if let Some((off, rotated_cand_set)) = closeness(&input[cur], &input[cand_neigh]) {
        input[cand_neigh] = rotated_cand_set;
        stack.push(cand_neigh);
        locations.insert(cand_neigh, locations[&cur] + off);
      }
    }
  }

  let points = input
    .into_iter()
    .enumerate()
    .map(|(i, points)| {
      points
        .into_iter()
        .map(|pt| pt + locations[&i] - locations[&0])
        .collect_vec()
    })
    .flatten()
    .collect_vec();

  let res1 = points.iter().unique().count();

  let res2 = locations
    .values()
    .cartesian_product(locations.values())
    .map(|(a, b)| a.l1_dist(b))
    .max()
    .unwrap_or(-1);

  Ok((res1 as i32, res2))
}
pub fn run(fname: &str) -> AocResult<(impl Display, impl Display)> { solve(fname) }
