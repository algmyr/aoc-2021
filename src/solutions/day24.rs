use std::fmt::Display;

use hashbrown::HashSet;
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

fn op(mut z: i64, input: i64, a: i64, b: i64, c: i64) -> i64 {
  let w = input;
  let x = (z % 26 + b != w) as i64;
  z /= a;
  z + x * (z*25 + (w + c))
}

fn solve() -> AocResult<(String, String)> {
  fn brute_all((a, b, c): (i64, i64, i64), targets: &HashSet<i64>) -> HashSet<i64> {
    let mut res = HashSet::new();
    for d in 1..=9 {
      for z in 0..=500000 {
        let res_z = op(z, d, a, b, c);
        if targets.contains(&res_z) {
          res.insert(z);
        }
      }
    }
    res
  }
  
  let args = [
    (1, 13, 8),
    (1, 12, 13),
    (1, 12, 8),
    (1, 10, 10),
    (26, -11, 12),
    (26, -13, 1),
    (1, 15, 13),
    (1, 10, 5),
    (26, -2, 10),
    (26, -6, 3),
    (1, 14, 2),
    (26, 0, 2),
    (26, -15, 12),
    (26, -4, 7),
  ];

  let mut v_brute = HashSet::from_iter([0]);
  let mut valid = vec![v_brute.clone()];
  for i in (1..args.len()).rev() {
    v_brute = brute_all(args[i], &v_brute);
    valid.push(v_brute.clone());
  }
  valid.reverse();

  let mut min_digits = vec![];
  let mut max_digits = vec![];
  let mut max_z = 0;
  let mut min_z = 0;
  for i in 0..args.len() {
    let (a, b, c) = args[i];
    for d in (1..=9).rev() {
      let nz = op(max_z, d, a, b, c);
      if valid[i].contains(&nz) {
        max_z = nz;
        max_digits.push(d);
        break;
      }
    }
    for d in 1..=9 {
      let nz = op(min_z, d, a, b, c);
      if valid[i].contains(&nz) {
        min_z = nz;
        min_digits.push(d);
        break;
      }
    }
  }
  Ok((max_digits.iter().join(""), min_digits.iter().join("")))
}

pub fn run(_fname: &str) -> AocResult<(impl Display, impl Display)> {
  solve()
}
