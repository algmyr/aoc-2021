use std::fmt;
use std::fmt::Display;
use std::str::FromStr;

use crate::error::AocResult;

fn parse_input<T>(fname: &str) -> Result<Vec<((T, T), (T, T))>, Box<dyn std::error::Error>>
where
  T: FromStr,
  T: Copy,
  <T as FromStr>::Err: fmt::Debug,
{
  let to_int = |x: &str| -> T { x.parse().expect("Integer parse failed") };
  let parse_pair = |s: &str| -> Vec<T> { s.split(',').map(to_int).collect() };
  let parse_line = |s: &str| -> ((T, T), (T, T)) {
    let v: Vec<T> = s.split(" -> ").map(parse_pair).flatten().collect();
    match v[..] {
      [a, b, c, d] => ((a, b), (c, d)),
      _ => panic!("T_T"),
    }
  };

  Ok(
    std::fs::read_to_string(fname)?
      .trim()
      .lines()
      .map(parse_line)
      .collect(),
  )
}

type IntType = i32;

#[allow(dead_code)]
struct Board {
  n_rows: usize,
  n_cols: usize,
  data: Vec<IntType>,
}

impl Board {
  fn new(n_rows: usize, n_cols: usize) -> Self {
    Self {
      n_rows,
      n_cols,
      data: vec![0; n_cols * n_rows],
    }
  }
  fn inc(&mut self, i: usize, j: usize) { self.data[i * self.n_cols + j] += 1; }
  fn draw_line(&mut self, x1: i32, y1: i32, x2: i32, y2: i32) {
    let dx = (x2 - x1).signum();
    let dy = (y2 - y1).signum();
    let dist = (x1 - x2).abs().max((y1 - y2).abs());
    for i in 0..=dist {
      let x = x1 + dx * i;
      let y = y1 + dy * i;
      self.inc(y as usize, x as usize);
    }
  }
}

fn solve(fname: &str) -> AocResult<(usize, usize)> {
  let coords = parse_input::<i32>(fname).expect("Input read failed");

  const N: usize = 1000;
  let mut board1 = Board::new(N, N);
  let mut board2 = Board::new(N, N);

  for ((x1, y1), (x2, y2)) in coords {
    if x1 != x2 && y1 != y2 {
      board1.draw_line(x1, y1, x2, y2);
    };
    board2.draw_line(x1, y1, x2, y2);
  }

  Ok((
    board1.data.iter().filter(|&&x| x > 1).count(),
    board2.data.iter().filter(|&&x| x > 1).count(),
  ))
}

pub fn run(fname: &str) -> AocResult<(impl Display, impl Display)> { solve(fname) }
