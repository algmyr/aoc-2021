use std::fmt::Display;
use std::iter::repeat;

use itertools::Itertools;

use crate::error::AocResult;

fn parse_input(fname: &str) -> AocResult<Vec<Vec<u8>>> {
  let grid = std::fs::read_to_string(fname)?
    .lines()
    .map(|s| s.bytes().map(|b| b - ('0' as u8)).collect_vec())
    .collect_vec();
  Ok(grid)
}

struct State {
  x: i32,
  y: i32,
}

struct CyclicPQ {
  distance: usize,
  buckets: [Vec<State>; 16],
}

impl CyclicPQ {
  fn new() -> CyclicPQ {
    CyclicPQ {
      distance: 0,
      buckets: [
        vec![State { x: 0, y: 0 }],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
      ],
    }
  }

  fn push(&mut self, s: State, dist: usize) {
    let n = self.buckets.len();
    self.buckets[dist % n].push(s);
  }

  fn pop(&mut self) -> Option<State> {
    while self.buckets[self.distance % self.buckets.len()].is_empty() {
      self.distance += 1;
    }
    self.buckets[self.distance % self.buckets.len()].pop()
  }
}

const INFTY: usize = 1000000000;

fn shortest_path(grid: Vec<Vec<u8>>) -> AocResult<i32> {
  let height = grid.len() as i32;
  let width = grid[0].len() as i32;

  let mut dist_to = (0..height)
    .map(|_| repeat(INFTY).take(width as usize).collect_vec())
    .collect_vec();

  // Silly optimization, makes use of 1-9 costs.
  let mut pq = CyclicPQ::new();

  while let Some(cur) = pq.pop() {
    if cur.x == width - 1 && cur.y == height - 1 {
      break;
    }

    for (dx, dy) in [(1, 0), (-1, 0), (0, 1), (0, -1)] {
      let x = cur.x + dx;
      let y = cur.y + dy;
      if x < 0 || x >= width || y < 0 || y >= height {
        continue;
      }

      let new_dist = pq.distance + (grid[y as usize][x as usize] as usize);
      if new_dist < dist_to[y as usize][x as usize] {
        dist_to[y as usize][x as usize] = new_dist;
        pq.push(State { x, y }, new_dist);
      }
    }
  }
  Ok(pq.distance as i32)
}

fn part1(fname: &str) -> AocResult<i32> {
  let grid = parse_input(fname)?;
  shortest_path(grid)
}

fn part2(fname: &str) -> AocResult<i32> {
  let grid = parse_input(fname)?;
  let n = 5;

  // lmao
  let expanded = (0..n)
    .flat_map(|i| {
      grid
        .iter()
        .map(|row| {
          (0..n)
            .flat_map(|j| row.iter().map(move |val| (val + j + i - 1) % 9 + 1))
            .collect_vec()
        })
        .collect_vec()
    })
    .collect_vec();

  //let mut expanded = vec![];
  //for i in 0..n {
  //  for row in grid.iter() {
  //    let mut new_row = vec![];
  //    for j in 0..n {
  //      new_row.extend(row.iter().map(|val| (val + j + i - 1) % 9 + 1));
  //    }
  //    expanded.push(new_row);
  //  }
  //}

  shortest_path(expanded)
}

pub fn run(fname: &str) -> AocResult<(impl Display, impl Display)> {
  Ok((part1(fname)?, part2(fname)?))
}
