use std::collections::{HashMap, HashSet};
use std::fmt::Display;
use std::str::FromStr;

use itertools::Itertools;

use crate::error::{aoc_error, AocError, AocResult};

struct Data {
  patterns: Vec<String>,
  digits: Vec<String>,
}

impl FromStr for Data {
  type Err = AocError;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let parts: Vec<String> = s.split(" | ").map(String::from).collect();
    match &parts[..] {
      [a, b] => Ok(Data {
        patterns: a.split(" ").map(String::from).collect(),
        digits: b.split(" ").map(String::from).collect(),
      }),
      _ => Err(aoc_error("Parse error")),
    }
  }
}

fn parse_input(fname: &str) -> AocResult<Vec<Data>> {
  let content = std::fs::read_to_string(fname)?;
  content.lines().map(|s| s.parse()).collect()
}

fn part1(fname: &str) -> AocResult<usize> {
  let vec = parse_input(fname)?;
  Ok(
    vec
      .iter()
      .map(|x| {
        x.digits
          .iter()
          .filter(|&x| x.len() == 2 || x.len() == 4 || x.len() == 3 || x.len() == 7)
          .count()
      })
      .sum::<usize>(),
  )
}

fn part2(fname: &str) -> AocResult<usize> {
  let input_data = parse_input(fname)?;
  let mut res = 0;

  fn intersect(a: &str, b: &str) -> i32 {
    a.bytes()
      .cartesian_product(b.bytes())
      .map(|(x, y)| (x == y) as i32)
      .sum()
  }

  for data in input_data {
    let mut digit_pos = [255; 10];
    for (i, p) in data.patterns.iter().enumerate() {
      match p.len() {
        2 => digit_pos[1] = i,
        3 => digit_pos[7] = i,
        4 => digit_pos[4] = i,
        7 => digit_pos[8] = i,
        _ => (),
      };
    }

    let digit = |i: usize| data.patterns[digit_pos[i]].as_ref();

    res += data
      .digits
      .into_iter()
      .map(|e| {
        match e.len() {
          2 => 1,
          4 => 4,
          3 => 7,
          7 => 8,
          _ => {
            match intersect(digit(4), &e) {
              2 => 2,
              3 => match intersect(digit(1), &e) {
                1 => e.len(), // Because funny
                2 => {
                  if e.len() == 6 {
                    0
                  } else {
                    3
                  }
                }
                _ => panic!("AAA"),
              },
              4 => 9,
              _ => panic!("AAA"),
            }
          }
        }
      })
      .fold(0, |accu, e| accu * 10 + e);
  }
  Ok(res)
}

#[allow(dead_code)]
fn part2_slow(fname: &str) -> AocResult<usize> {
  let vec = parse_input(fname)?;
  let thing: HashMap<&str, usize> = HashMap::from([
    ("abcdefg", 8),
    ("bcdef", 5),
    ("acdfg", 2),
    ("abcdf", 3),
    ("abd", 7),
    ("abcdef", 9),
    ("bcdefg", 6),
    ("abef", 4),
    ("abcdeg", 0),
    ("ab", 1),
  ]);

  let alpha = "abcdefg";

  let mut res = 0;
  for x in vec {
    // Let's be very dumb, iterate over all possible substitutions.
    // There are only 7! of them.
    for p in (0..7).permutations(7) {
      let map_string = |s: &String| {
        s.chars()
          .map(|c| {
            let ix = ((c as i32) - ('a' as i32)) as usize;
            alpha.chars().nth(p[ix]).unwrap_or('?')
          })
          .sorted()
          .collect::<String>()
      };

      let v: HashSet<String> = x.patterns.iter().map(map_string).collect();
      if thing.keys().all(|e| v.contains(&e.to_string())) {
        res += x
          .digits
          .iter()
          .map(|d| thing[map_string(d).as_str()])
          .fold(0, |accu, e| accu * 10 + e);
        break;
      }
    }
  }
  Ok(res)
}

pub fn run(fname: &str) -> AocResult<(impl Display, impl Display)> {
  Ok((part1(fname)?, part2(fname)?))
}
