use std::{fs, fmt::Display};

use crate::error::AocResult;

fn parse_input(fname: &str) -> AocResult<(Vec<(usize, usize)>,Vec<(usize, usize)>,Vec<Vec<bool>>)> {
  let contents = fs::read_to_string(fname)?;

  let mut east = vec![];
  let mut south = vec![];
  let mut occupied = vec![vec![false; contents.lines().next().unwrap().len()]; contents.lines().count()];
  
  for (y, line) in contents.lines().enumerate() {
    for (x, b) in line.bytes().enumerate() {
      match b {
        b'>' => {
          east.push((x, y));
          occupied[y][x] = true;
        },
        b'v' => {
          south.push((x, y));
          occupied[y][x] = true;
        },
        _ => (),
      }
    }
  }

  Ok((east, south, occupied))
}

fn part1(fname: &str) -> AocResult<i32> {
  let (mut east, mut south, mut occupied) = parse_input(fname)?;
  let height = occupied.len();
  let width = occupied[0].len();

  let mut iter = 0;
  loop {
    iter += 1;
    let mut n_moves = 0;

    let mut moves = vec![];
    for (x, y) in &mut east {
      let nx = (*x + 1)%width;
      if !occupied[*y][nx] {
        moves.push(((*x, *y), (nx, *y)));
        *x = nx;
      }
    }
    n_moves += moves.len();
    for ((x1, y1), (x2, y2)) in moves {
      assert_eq!(occupied[y1][x1], true);
      assert_eq!(occupied[y2][x2], false);
      occupied[y1][x1] = false;
      occupied[y2][x2] = true;
    }

    let mut moves = vec![];
    for (x, y) in &mut south {
      let ny = (*y + 1)%height;
      if !occupied[(*y + 1)%height][*x] {
        moves.push(((*x, *y), (*x, ny)));
        *y = ny;
      }
    }
    n_moves += moves.len();
    for ((x1, y1), (x2, y2)) in moves {
      assert_eq!(occupied[y1][x1], true);
      assert_eq!(occupied[y2][x2], false);
      occupied[y1][x1] = false;
      occupied[y2][x2] = true;
    }

    if n_moves == 0 { break; }
  };

  Ok(iter)
}

pub fn run(fname: &str) -> AocResult<(impl Display, impl Display)> {
  Ok((part1(fname)?, -1))
}
