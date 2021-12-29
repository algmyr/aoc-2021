use std::fmt::{Debug, Display};

use itertools::Itertools;

use crate::error::{aoc_error, AocResult};

#[derive(Clone, Copy)]
struct Path {
  data: u32,
}

impl Path {
  fn new(len: i32, path: u32) -> Self {
    // Low 5 bits len, high bits path
    debug_assert!(len <= 32 - 5);
    let data = path << 5u32 | (len as u32);
    Self { data }
  }
  fn len(&self) -> u32 { self.data & 0b11111 }
  fn path(&self) -> u32 { self.data >> 5 }
  fn shorten(&self) -> Self { Self::new((self.len() as i32) - 1, self.path() >> 1) }
  fn lengthen(&self, dir: u32) -> Self {
    Self::new((self.len() as i32) + 1, self.path() << 1 | dir)
  }
  fn lengthen_below(&self, dir: u32) -> Self {
    Self::new((self.len() as i32) + 1, self.path() | dir << self.len())
  }
}

impl Debug for Path {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let len = self.len() as usize;
    let path = format!("{:027b}", self.path());
    f.write_str(&format!("{}_{}", len, &path[path.len() - len..]))
  }
}

fn parse_input(fname: &str) -> AocResult<Vec<Vec<(Path, i32)>>> {
  fn parse_line(line: &str) -> Vec<(Path, i32)> {
    let mut path: u32 = 0;
    let mut depth = 0;
    let mut values = vec![];
    for b in line.bytes() {
      match b {
        b'[' => {
          depth += 1;
          path = path << 1 | 0;
        }
        b']' => {
          depth -= 1;
          path >>= 1;
        }
        b',' => {
          path ^= 1;
        }
        b'0'..=b'9' => {
          let value = (b - b'0') as i32;
          values.push((Path::new(depth, path), value));
          ()
        }
        b' ' => {}
        t => panic!("Unexpected token '{}'", t as char),
      };
    }
    values
  }
  let input = std::fs::read_to_string(fname)?
    .lines()
    .map(parse_line)
    .collect_vec();
  Ok(input)
}

fn reduce(vec: Vec<(Path, i32)>) -> Vec<(Path, i32)> {
  fn explode_index(vec: &mut Vec<(Path, i32)>, i: usize) {
    let (path, lval) = vec.remove(i);
    let (_, rval) = vec.remove(i);
    vec.insert(i, (path.shorten(), 0));
    if i >= 1 {
      vec[i - 1].1 += lval;
    }
    if i + 1 < vec.len() {
      vec[i + 1].1 += rval;
    }
  }

  #[allow(dead_code)]
  fn explode(vec: &mut Vec<(Path, i32)>) {
    let mut i = 0;
    while i < vec.len() {
      let (path, _) = &vec[i];
      if path.len() > 4 {
        explode_index(vec, i);
      }
      i += 1;
    }
  }

  fn split_and_trim(vec: &mut Vec<(Path, i32)>) {
    let mut i = 0;
    while i < vec.len() {
      let (_, value) = &vec[i];
      if *value <= 9 {
        i += 1;
        continue;
      }

      // Split (and trim if needed)
      // If split we might need to look at out split elements again.
      // If trim we might also need to look at the element before.
      let (path, value) = vec.remove(i);
      let half = value / 2;
      vec.insert(i, (path.lengthen(1), value - half));
      vec.insert(i, (path.lengthen(0), half));
      if path.len() >= 4 {
        explode_index(vec, i);
        if i > 0 {
          i -= 1;
        }
      }
    }
  }

  // This earned mostly nothing
  let mut res: Vec<(Path, i32)> = vec![];
  let mut vec_iter = vec.into_iter();
  let mut add_next = 0;
  while let Some((path, value)) = vec_iter.next() {
    let value = value + add_next;
    add_next = 0;
    if path.len() > 4 {
      let (_, value2) = vec_iter.next().unwrap();
      if let Some((_, last_value)) = res.last_mut() {
        *last_value += value;
      }
      add_next = value2;
      res.push((path.shorten(), 0));
    } else {
      res.push((path, value));
    }
  }

  //let mut res = vec;
  //explode(&mut res);

  // This might be higher importance?
  split_and_trim(&mut res);
  res
}

fn join(a: &[(Path, i32)], b: &[(Path, i32)]) -> Vec<(Path, i32)> {
  let mut res = vec![];
  res.extend(
    a.into_iter()
      .map(|&(path, value)| (path.lengthen_below(0), value)),
  );
  res.extend(
    b.into_iter()
      .map(|&(path, value)| (path.lengthen_below(1), value)),
  );
  res
}

fn magnitude(vec: &Vec<(Path, i32)>) -> i32 {
  vec
    .iter()
    .map(|&(path, value)| {
      let seq = path.path();
      value
        * (0..path.len())
          .map(|i| if (seq >> i & 1) == 1 { 2 } else { 3 })
          .product::<i32>()
    })
    .sum::<i32>()
}

fn part1(fname: &str) -> AocResult<i32> {
  let expressions = parse_input(fname)?;

  let result = expressions
    .iter()
    .cloned()
    .reduce(|accu, el| reduce(join(&accu, &el)))
    .ok_or(aoc_error("Empty fold"))?;

  Ok(magnitude(&result))
}

fn part2(fname: &str) -> AocResult<i32> {
  let expressions = parse_input(fname)?;

  (0..expressions.len())
    .flat_map(|i| (0..expressions.len()).map(move |j| (i, j)))
    .filter(|(i, j)| i != j)
    .map(|(i, j)| magnitude(&reduce(join(&expressions[i], &expressions[j])))).max()
    .ok_or(aoc_error("Empty input"))
}

pub fn run(fname: &str) -> AocResult<(impl Display, impl Display)> {
  Ok((part1(fname)?, part2(fname)?))
}
