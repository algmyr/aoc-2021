use std::fmt::Display;
use std::io::BufRead;

use crate::error::{aoc_error, AocResult};

#[derive(Debug)]
struct Board {
  board: Vec<Vec<i32>>,
  width: usize,
  height: usize,
  checked: Vec<Vec<bool>>,
}
impl Board {
  fn tick(&mut self, n: i32) {
    for i in 0..self.height {
      for j in 0..self.width {
        if self.board[i][j] == n {
          self.checked[i][j] = true;
          return;
        }
      }
    }
  }

  fn is_done(&self) -> bool {
    (0..self.height).any(|i| (0..self.width).all(|j| self.checked[i][j]))
      || (0..self.width).any(|j| (0..self.height).all(|i| self.checked[i][j]))
  }

  fn sum_unchecked(&self) -> i32 {
    self
      .checked
      .iter()
      .flatten()
      .zip(self.board.iter().flatten())
      .filter(|(&checked, _)| !checked)
      .map(|(_, value)| value)
      .sum()
  }

  fn new(board: Vec<Vec<i32>>) -> Board {
    let height = board.len();
    let width = board[0].len();
    let checked = vec![vec![false; width]; height];
    Board {
      board,
      height,
      width,
      checked,
    }
  }
}

fn parse_input(fname: &str) -> AocResult<(Vec<i32>, Vec<Board>)> {
  let to_int = |x: &str| x.parse::<i32>().expect("Integer parse failed");

  let f = std::fs::File::open(fname)?;
  let mut lines = std::io::BufReader::new(f).lines();
  let numbers: Vec<i32> = lines
    .next()
    .ok_or(aoc_error("No first line"))??
    .split(',')
    .map(to_int)
    .collect();

  let mut boards: Vec<Board> = vec![];
  loop {
    if !lines.next().is_some() {
      break;
    }
    let v = (&mut lines)
      .take(5)
      .map(|x| {
        x.expect("Failed to read line")
          .split_whitespace()
          .map(to_int)
          .collect()
      })
      .collect();
    boards.push(Board::new(v));
  }
  Ok((numbers, boards))
}

fn part1(fname: &str) -> AocResult<i32> {
  let (numbers, mut boards) = parse_input(fname)?;

  for n in numbers {
    for board in &mut boards {
      board.tick(n);
      if board.is_done() {
        return Ok(n * board.sum_unchecked());
      }
    }
  }
  Err(aoc_error("No solution found"))
}

fn part2(fname: &str) -> AocResult<i32> {
  let (numbers, mut boards) = parse_input(fname)?;

  let mut n_boards_left = boards.len();
  for n in numbers {
    for board in &mut boards {
      board.tick(n);
      if board.is_done() {
        n_boards_left -= 1;
        if n_boards_left == 0 {
          return Ok(n * board.sum_unchecked());
        }
      }
    }
    boards = boards.into_iter().filter(|b| !b.is_done()).collect();
  }
  Err(aoc_error("No solution found"))
}

pub fn run(fname: &str) -> AocResult<(impl Display, impl Display)> {
  Ok((part1(fname)?, part2(fname)?))
}
