use std::collections::HashMap;
use std::fmt::Display;

use itertools::Itertools;

use crate::error::{aoc_error, AocResult};

fn parse_input(fname: &str) -> AocResult<(String, Vec<(String, String)>)> {
  let content = std::fs::read_to_string(fname)?;
  let sections = content.split("\n\n").collect_vec();

  if sections.len() != 2 {
    return Err(aoc_error("Wrong number of sections"));
  }

  let template = sections[0].to_owned();
  let rules = sections[1]
    .lines()
    .map(|s| {
      let parts = s.split(" -> ").collect_vec();
      (parts[0].to_owned(), parts[1].to_owned())
    })
    .collect();

  Ok((template, rules))
}

fn solve(fname: &str, steps: i32) -> AocResult<i64> {
  let (template, rules) = parse_input(fname)?;

  let x = "$".to_owned() + &template + "$";
  let mut counts = HashMap::new();
  (0..x.len() - 1).map(|i| &x[i..i + 2]).for_each(|s| {
    *counts.entry(s.to_owned()).or_insert(0) += 1;
  });

  for _ in 0..steps {
    let mut new_counts = counts.clone();
    for (a, b) in &rules {
      let first = a[0..1].to_owned() + b;
      let second = b.to_owned() + &a[1..2];
      let count = counts.get(a.as_str()).unwrap_or(&0);
      *new_counts.entry(a.clone()).or_insert(0) -= count; // TODO(algmyr): wtf, why do I need a clone?
      *new_counts.entry(first).or_insert(0) += count;
      *new_counts.entry(second).or_insert(0) += count;
    }
    counts = new_counts.clone();
  }

  let mut final_counts = HashMap::new();
  for (key, val) in counts {
    for c in key.chars() {
      *final_counts.entry(c).or_insert(0) += val;
    }
  }

  let freqs: Vec<i64> = final_counts
    .iter()
    .map(|(_, v)| v / 2)
    .sorted()
    .collect_vec();
  Ok(freqs[freqs.len() - 1] - freqs[1])
}

fn part1(fname: &str) -> AocResult<i64> { solve(fname, 10) }

fn part2(fname: &str) -> AocResult<i64> { solve(fname, 40) }

pub fn run(fname: &str) -> AocResult<(impl Display, impl Display)> {
  Ok((part1(fname)?, part2(fname)?))
}
