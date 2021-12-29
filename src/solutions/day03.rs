use std::fmt::Display;

use crate::error::AocResult;

fn most_common_in_column(v: &Vec<Vec<i8>>, col_i: usize) -> i8 {
  let mut counts = [0, 0];
  for e in v {
    counts[e[col_i] as usize] += 1;
  }
  (counts[1] >= counts[0]) as i8
}

fn to_int(v: &[i8]) -> i32 { v.into_iter().fold(0, |accu, x| accu << 1 | (*x as i32)) }

fn o2gen(v: &Vec<Vec<i8>>) -> i32 {
  let mut w = v.clone();
  let mut i = 0;
  while w.len() > 1 {
    let mc = most_common_in_column(&w, i);
    w = w.into_iter().filter(|x| x[i] == mc).collect();
    i += 1;
  }
  to_int(&w[0])
}

fn co2scrub(v: &Vec<Vec<i8>>) -> i32 {
  let mut w = v.clone();
  let mut i = 0;
  while w.len() > 1 {
    let mc = most_common_in_column(&w, i);
    w = w.into_iter().filter(|x| x[i] != mc).collect();
    i += 1;
  }
  to_int(&w[0])
}

fn parse_input(fname: &str) -> Vec<Vec<i8>> {
  std::fs::read_to_string(fname)
    .expect("Reading input failed")
    .trim()
    .lines()
    .map(|x| x.chars().map(|x| (x == '1') as i8).collect())
    .collect()
}

fn part1(fname: &str) -> i32 {
  let inp = parse_input(fname);
  let n = inp[0].len();
  let val = to_int(
    &(0..n)
      .map(|i| most_common_in_column(&inp, i))
      .collect::<Vec<i8>>(),
  );
  let val_inv = ((1 << n) - 1) - val;
  val * val_inv
}

fn part2(fname: &str) -> i32 {
  let inp = parse_input(fname);
  o2gen(&inp) * co2scrub(&inp)
}

pub fn run(fname: &str) -> AocResult<(impl Display, impl Display)> {
  Ok((part1(fname), part2(fname)))
}
