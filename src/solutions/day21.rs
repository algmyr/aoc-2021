use std::fmt::Display;
use std::ops::{Index, IndexMut};

use itertools::{iproduct, Itertools};

use crate::error::AocResult;

fn parse_input(fname: &str) -> AocResult<(i32, i32)> {
  let s = std::fs::read_to_string(fname)?;
  fn parse(s: &str) -> i32 { s.split(":").last().unwrap().trim().parse().unwrap() }
  match s.lines().collect_vec()[..] {
    [a, b] => Ok((parse(a), parse(b))),
    _ => panic!("AAA"),
  }
}

fn part1(fname: &str) -> AocResult<i32> {
  let (start1, start2) = parse_input(fname)?;
  let mut throws = (1..=100).cycle();
  let mut player = 0;
  let mut pos = [start1, start2];
  let mut score = [0, 0];
  let mut n_throws = 0;
  loop {
    let a = throws.next().unwrap();
    let b = throws.next().unwrap();
    let c = throws.next().unwrap();
    n_throws += 3;
    let tot = a + b + c;
    pos[player] = (pos[player] + tot - 1) % 10 + 1;
    score[player] += pos[player];
    if score[player] >= 1000 {
      break;
    }
    player = 1 - player;
  }
  let res = score[1 - player] * n_throws;
  Ok(res)
}

const WIN_LIMIT: usize = 21;
const MAX_POS: usize = 10;

type IntType = i64;

#[derive(Clone)]
struct Space {
  data: Vec<IntType>,
}

impl Space {
  const N_PARAMS: usize = 5;
  fn new() -> Self {
    let data = vec![0; 2 * WIN_LIMIT.pow(2) * (MAX_POS + 1).pow(2)];
    Self { data }
  }
  fn index(indices: [usize; Self::N_PARAMS]) -> usize {
    let [s1, s2, player, p1, p2] = indices;
    let ix = s1;
    let ix = s2 + ix * WIN_LIMIT;
    let ix = player + ix * 2;
    let ix = p1 + ix * (MAX_POS + 1);
    let ix = p2 + ix * (MAX_POS + 1);
    ix
  }
}

impl Index<[usize; Space::N_PARAMS]> for Space {
  type Output = IntType;
  fn index<'a>(&'a self, indices: [usize; Space::N_PARAMS]) -> &'a IntType {
    &self.data[Space::index(indices)]
  }
}

impl IndexMut<[usize; Space::N_PARAMS]> for Space {
  fn index_mut<'a>(&'a mut self, indices: [usize; Space::N_PARAMS]) -> &'a mut IntType {
    &mut self.data[Space::index(indices)]
  }
}

fn part2(fname: &str) -> AocResult<IntType> {
  let (start1, start2) = parse_input(fname)?;
  let mut space = Space::new();
  space[[0, 0, 1, start1 as usize, start2 as usize]] = 1;

  fn pair_to_array((x, y): (usize, usize)) -> [usize; 2] { [x, y] }

  // 3d3 distribution
  let rolls = [(3, 1), (4, 3), (5, 6), (6, 7), (7, 6), (8, 3), (9, 1)];
  let order = iproduct!(0..WIN_LIMIT, 0..WIN_LIMIT)
    .sorted_by_key(|&(i, j)| (i + j, i, j))
    .map(pair_to_array)
    .collect_vec();

  let mut win = [0, 0];
  for scores in order {
    for player in [0, 1] {
      let my_s = scores[player];
      let your_s = scores[1 - player];
      for positions in iproduct!(1..=10, 1..=10).map(pair_to_array) {
        let my_p = positions[player];
        let your_p = positions[1 - player];

        let ways = space[[my_s, your_s, 1 - player, my_p, your_p]];
        for (roll, count) in rolls {
          let new_pos = (my_p + roll - 1) % 10 + 1;
          let new_score = my_s + new_pos;
          if new_score < WIN_LIMIT {
            space[[your_s, new_score, player, your_p, new_pos]] += count * ways;
          } else {
            win[player] += count * ways;
          }
        }
      }
    }
  }
  Ok(win[0].max(win[1]))
}

pub fn run(fname: &str) -> AocResult<(impl Display, impl Display)> {
  Ok((part1(fname)?, part2(fname)?))
}
