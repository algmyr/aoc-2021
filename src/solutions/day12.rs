use std::collections::HashMap;
use std::fmt::Display;
use std::iter::repeat;

use itertools::Itertools;

use crate::error::AocResult;

struct Graph {
  conn: Vec<Vec<usize>>,
  start: usize,
  end: usize,
  is_big: Vec<bool>,
}

fn parse_input(fname: &str) -> AocResult<Graph> {
  let content = std::fs::read_to_string(fname)?;

  let mut index_map = HashMap::new();

  let mut get_index = |s: &str| {
    let sz = index_map.len();
    *index_map.entry(s.to_owned()).or_insert(sz)
  };

  let mut g = Graph {
    conn: vec![],
    is_big: vec![],
    start: 0,
    end: 0,
  };

  for parts in content.lines().map(|x| x.split('-').collect_vec()) {
    match parts[..] {
      [astr, bstr] => {
        let a = get_index(astr);
        let b = get_index(bstr);
        if a >= g.conn.len() {
          g.conn.push(vec![]);
          g.is_big.push(
            astr
              .chars()
              .next()
              .map(|c| c.is_uppercase())
              .unwrap_or(false),
          );
        }
        if b >= g.conn.len() {
          g.conn.push(vec![]);
          g.is_big.push(
            bstr
              .chars()
              .next()
              .map(|c| c.is_uppercase())
              .unwrap_or(false),
          );
        }
        g.conn[a].push(b);
        g.conn[b].push(a);
        if astr == "start" {
          g.start = a
        }
        if astr == "end" {
          g.end = a
        }
        if bstr == "start" {
          g.start = b
        }
        if bstr == "end" {
          g.end = b
        }
      }
      _ => panic!("Weird input"),
    };
  }

  Ok(g)
}

fn dfs(g: &Graph, visited: &mut Vec<bool>, cur: usize, target: usize) -> i32 {
  if cur == target {
    return 1;
  }
  if visited[cur] {
    return 0;
  }
  let mut count: i32 = 0;
  if !g.is_big[cur] {
    visited[cur] = true;
  }
  for neigh in &g.conn[cur] {
    count += dfs(g, visited, *neigh, target);
  }
  visited[cur] = false;
  count
}

fn part1(fname: &str) -> AocResult<i32> {
  let g = parse_input(fname)?;
  let mut visited = repeat(false).take(g.conn.len()).collect_vec();
  Ok(dfs(&g, &mut visited, g.start, g.end))
}

fn dfs2(
  g: &Graph,
  visited: &mut Vec<bool>,
  cur: usize,
  start: usize,
  target: usize,
  dup_used: &mut i32,
) -> i32 {
  if cur == target {
    return 1;
  }
  if !(!visited[cur] || (*dup_used == -1 && cur != start)) {
    return 0;
  }
  let mut count: i32 = 0;
  if !g.is_big[cur] {
    if !visited[cur] {
      visited[cur] = true;
    } else {
      *dup_used = cur as i32;
    }
  }
  for neigh in &g.conn[cur] {
    count += dfs2(g, visited, *neigh, start, target, dup_used);
  }
  if !g.is_big[cur] {
    if *dup_used == cur as i32 {
      *dup_used = -1;
    } else {
      visited[cur] = false;
    }
  }
  count
}

fn part2(fname: &str) -> AocResult<i32> {
  let g = parse_input(fname)?;
  let mut visited = repeat(false).take(g.conn.len()).collect_vec();
  Ok(dfs2(&g, &mut visited, g.start, g.start, g.end, &mut -1))
}

pub fn run(fname: &str) -> AocResult<(impl Display, impl Display)> {
  Ok((part1(fname)?, part2(fname)?))
}
