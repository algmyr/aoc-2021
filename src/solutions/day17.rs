use std::fmt::Display;

use crate::error::{AocError, AocResult};

fn parse_input(fname: &str) -> AocResult<((i32, i32), (i32, i32))> {
  fn parse_interval(s: &str) -> Option<(i32, i32)> {
    let (l, r) = s[2..].split_once("..")?;
    let (l, r) = (l.parse::<i32>().ok()?, r.parse::<i32>().ok()?);
    Some((l, r))
  }
  let s = std::fs::read_to_string(fname)?;

  let res = s
    .trim()
    .split_once(": ")
    .and_then(|(_, s)| s.split_once(", "))
    .and_then(|(s1, s2)| Some((parse_interval(s1)?, parse_interval(s2)?)));
  res.ok_or(AocError::Custom("Parse error".to_owned()))
}

// Simple binary search helper
fn bs(mut l: i32, mut r: i32, pred: impl Fn(i32) -> bool) -> (i32, i32) {
  // Assumed: pred(l) is true, pred(r) is false
  while r - l > 1 {
    let mid = (l + r) / 2; // yeah yeah, overflow and whatnot
    if pred(mid) {
      l = mid;
    } else {
      r = mid;
    }
  }
  (l, r)
}

fn tri_num(n: i32) -> i32 { n * (n + 1) / 2 }

fn solve(fname: &str) -> AocResult<(i32, i32)> {
  // Assumptions:
  // * Box is to the right and down of (0, 0).
  //   * x could be trivially fixed by reflecting.
  //   * y is a slighly different case than what is handled here
  //     (e.g. you can hit the box on the way up)
  //
  // Big insights to be had:
  //   x and y are completely independent

  let ((xmin, xmax), (ymin, ymax)) = parse_input(fname)?;

  // Precompute all valid intervals of vx for all relevant time steps.
  // x >= t*(t+1)/2
  // t <= sqrt(2x + 1/4) - 1/2
  let time_max = ((2.0 * xmax as f32 + 0.25).sqrt() - 0.5).floor() as i32;

  let mut vx_intervals = vec![(0, 0)];
  vx_intervals.extend((1..=time_max).map(|time_lim| {
    let upper_lim = xmax + 1; // Tight for time_lim = 1
    let (_, first_time) = bs(0, upper_lim, |n| tri_num(n) - tri_num(n - time_lim) < xmin);
    let (last_time, _) = bs(0, upper_lim, |n| tri_num(n) - tri_num(n - time_lim) <= xmax);
    (first_time, last_time + 1)
  })); // Time can be larger, but then the interval is the same as the last element.

  //

  let probe_hit_times = |mut vy: i32| -> Vec<i32> {
    let mut t = 0;
    let mut times = vec![];
    let mut y = 0;
    while y >= ymin {
      if ymin <= y && y <= ymax {
        times.push(t);
      }
      y += vy;
      vy -= 1;
      t += 1;
    }
    times
  };

  // Try all potentially relevant y velocities to find hit times.
  let ylim = ymin.abs().max(ymax.abs());

  let mut y_max = 0;
  let mut n_hits = 0;
  for vy in -ylim..=ylim {
    // Answer is the size of the union of all the vx intervals.
    // Intervals are in decreasing order, which is used here.
    let mut cutoff = i32::MAX;
    for t in probe_hit_times(vy) {
      y_max = y_max.max(vy * (vy + 1) / 2);
      let &(l, r) = vx_intervals
        .get(t as usize)
        .unwrap_or(vx_intervals.last().unwrap());
      n_hits += r.min(cutoff) - l;
      cutoff = l;
    }
  }

  Ok((y_max, n_hits))
}
pub fn run(fname: &str) -> AocResult<(impl Display, impl Display)> { solve(fname) }
