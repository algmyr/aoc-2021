use std::fmt::Display;
use std::fs;

use binary_heap_plus::BinaryHeap;
use hashbrown::HashMap;
use itertools::Itertools;

use crate::error::{aoc_error, AocResult};

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
// Sample:
//
// #############
// #...........#
// ###B#C#B#D###
//   #A#D#C#A#
//   #########
//
// A - 1
// B - 10
// C - 100
// D - 1000
//
// One numbering:
//
//  01.2.3.4.56
//    7 8 9 A
//    B C D E
//
//
//
// Things to note:
// Because D is so much higher you can allow 10 moves for every 1 move of the cheaper kind.
// A detour with D is at least 2 extra length, so 20 moves.
//
// The optimal thing to do is very likely to just try to set in stone what
// needs to happen for D and work downwards.
//

#[derive(PartialEq)]
enum PodIsWhere {
  Hallway,
  CorrectRoom,
  OtherRoom,
}

#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
enum PodKind {
  Amber = 0,
  Bronze = 1,
  Copper = 2,
  Desert = 3,
}

impl PodKind {
  fn from(b: u8) -> PodKind {
    match b {
      b'A' => PodKind::Amber,
      b'B' => PodKind::Bronze,
      b'C' => PodKind::Copper,
      b'D' => PodKind::Desert,
      _ => unreachable!("Pod outside ABCD"),
    }
  }
}

#[derive(Clone, Copy, Eq, PartialEq, Hash, Debug)]
struct Amphipod {
  kind: PodKind,
  location: i8,
  id: u8,
  moves: i8,
}

impl Amphipod {
  fn whereis(&self) -> PodIsWhere {
    use PodIsWhere::*;
    if self.location <= 6 {
      Hallway
    } else {
      let f = |x: i8, m: i8| if x >= 0x7 && (x - 0x7) % 4 == m { CorrectRoom } else { OtherRoom };
      f(self.location, self.kind as i8)
    }
  }
  fn multiplier(&self) -> i32 { 10i32.pow(self.kind as u32) }
}

fn parse_input(fname: &str) -> AocResult<Vec<Amphipod>> {
  let s = fs::read_to_string(fname)?;
  let letters = s.bytes().filter(|b| b"ABCD".contains(b)).collect_vec();
  match letters[..] {
    [a, b, c, d, e, f, g, h] => Ok(vec![
      Amphipod { kind: PodKind::from(a), location: 0x7, id: 0, moves: 0 },
      Amphipod { kind: PodKind::from(b), location: 0x8, id: 1, moves: 0 },
      Amphipod { kind: PodKind::from(c), location: 0x9, id: 2, moves: 0 },
      Amphipod { kind: PodKind::from(d), location: 0xA, id: 3, moves: 0 },
      Amphipod { kind: PodKind::from(e), location: 0xB, id: 4, moves: 0 },
      Amphipod { kind: PodKind::from(f), location: 0xC, id: 5, moves: 0 },
      Amphipod { kind: PodKind::from(g), location: 0xD, id: 6, moves: 0 },
      Amphipod { kind: PodKind::from(h), location: 0xE, id: 7, moves: 0 },
    ]),
    _ => Err(aoc_error("Wrong number of things")),
  }
}

struct System<const N: usize> {
  init_pods: Vec<Amphipod>,
  dist: [[i8; N]; N],
  conn: Vec<Vec<i8>>,
  init_blocked: [bool; N],
  burrows: [Vec<i8>; 4],
}

impl<const N: usize> System<N> {
  fn new(pods: Vec<Amphipod>, connections: Vec<(i8, i8, i8)>, burrows: [Vec<i8>; 4]) -> Self {
    let mut conn = vec![vec![]; N];
    let mut dist = [[42i8; N]; N];
    for (a, b, d) in connections {
      conn[a as usize].push(b);
      conn[b as usize].push(a);
      dist[a as usize][b as usize] = d;
      dist[b as usize][a as usize] = d;
    }
    for i in 0..N {
      dist[i][i] = 0;
    }
    // Floyd-Warshall, all pairs shortest paths
    for k in 0..N {
      for i in 0..N {
        for j in 0..N {
          if dist[i][j] > dist[i][k] + dist[k][j] {
            dist[i][j] = dist[i][k] + dist[k][j];
          }
        }
      }
    }

    let mut blocked = [false; N];
    for pod in &pods {
      blocked[pod.location as usize] = true;
    }

    Self { init_pods: pods, dist, conn, init_blocked: blocked, burrows }
  }

  fn initial_state(&self) -> State<N> {
    State {
      pods: self.init_pods.clone(),
      blocked: self.init_blocked,
      cost_w_heuristic: 0,
      cost: 0,
      burrows: self.burrows.clone(),
    }
  }
}

#[derive(Clone, Debug)]
struct State<const N: usize> {
  cost_w_heuristic: i32,
  cost: i32,
  blocked: [bool; N],
  pods: Vec<Amphipod>,
  burrows: [Vec<i8>; 4],
}

impl<const N: usize> State<N> {
  fn solved(&self) -> bool {
    self
      .pods
      .iter()
      .all(|pod| pod.whereis() == PodIsWhere::CorrectRoom)
  }
  fn board_snapshot(&self) -> Vec<i8> { self.pods.iter().map(|pod| pod.location).collect_vec() }
  fn get_moves(&self, state: &State<N>, system: &System<N>) -> Vec<(u8, i8)> {
    use PodIsWhere::*;
    let mut moves = vec![];
    for pod in &self.pods {
      if pod.moves >= 2 {
        continue;
      }
      let mut stack = vec![(1 << pod.location, pod.location)];
      while let Some((visited, cur)) = stack.pop() {
        if cur != pod.location {
          let mut cand_pod = *pod;
          cand_pod.location = cur;
          let burrow = &state.burrows[pod.kind as usize];
          if (pod.whereis() == Hallway && cur == *burrow.last().unwrap())
            || (pod.whereis() != Hallway && cand_pod.whereis() == Hallway)
          {
            moves.push((pod.id, cur));
          }
        }
        let mut new_visited = visited;
        for &neigh in &system.conn[cur as usize] {
          new_visited |= 1i32 << neigh;
        }
        for &neigh in &system.conn[cur as usize] {
          if visited & 1i32 << neigh == 0 && !self.blocked[neigh as usize] {
            stack.push((new_visited, neigh));
          }
        }
      }
    }
    moves
  }
}

fn solve<const N: usize>(system: System<N>) -> Option<State<N>> {
  let mut init = system.initial_state();
  for (i, burrow) in init.burrows.iter_mut().enumerate() {
    loop {
      let mut popped = false;
      for pod in &mut init.pods {
        if pod.kind as usize != i {
          continue;
        }
        if let Some(leaf_burrow) = burrow.last() {
          if pod.location == *leaf_burrow {
            pod.moves = 2; // hack
            burrow.pop();
            popped = true;
            break;
          }
        }
      }
      if !popped {
        break;
      }
    }
  }
  let mut seen = HashMap::new();
  seen.insert(system.initial_state().board_snapshot(), 0);
  let mut pq = BinaryHeap::new_by(|a: &State<N>, b: &State<N>| {
    a.cost_w_heuristic.cmp(&b.cost_w_heuristic).reverse()
  });
  pq.push(init);
  let mut res = None;
  while let Some(state) = pq.pop() {
    // Success case
    if state.solved() {
      res = Some(state);
      break;
    }

    // Calc moves and expand
    let moves = state.get_moves(&state, &system);
    for (id, target) in moves {
      let mut new_state = state.clone();
      let mut pod = new_state.pods[id as usize];
      pod.moves += 1;

      // Cost update
      new_state.cost +=
        (system.dist[pod.location as usize][target as usize] as i32) * pod.multiplier();
      new_state.cost_w_heuristic = new_state.cost
        + new_state
          .pods
          .iter()
          .filter(|&pod| pod.whereis() != PodIsWhere::CorrectRoom)
          .map(|pod| {
            let top_pos = (pod.kind as usize) + 0x7;
            (system.dist[pod.location as usize][top_pos] as i32)*pod.multiplier()
          })
          .sum::<i32>();

      // Location update
      new_state.blocked[pod.location as usize] = false;
      pod.location = target;
      new_state.blocked[pod.location as usize] = true;

      // Update burrows
      if pod.moves == 2 {
        let burrow = &mut new_state.burrows[pod.kind as usize];
        if let Some(leaf_burrow) = burrow.last() {
          if target == *leaf_burrow {
            burrow.pop();
          }
        }
      }
      new_state.pods[id as usize] = pod;

      let snap = new_state.board_snapshot();
      let mini = seen.entry(snap).or_insert(i32::MAX);
      if new_state.cost < *mini {
        *mini = new_state.cost;
        pq.push(new_state);
      }
    }
  }
  res
}

//  01.2.3.4.56
//    7 8 9 A
//    B C D E
fn part1(fname: &str) -> AocResult<i32> {
  let pods = parse_input(fname)?;

  let connections = vec![
    (0x0, 0x1, 1),
    (0x1, 0x2, 2),
    (0x2, 0x3, 2),
    (0x3, 0x4, 2),
    (0x4, 0x5, 2),
    (0x5, 0x6, 1),
    //
    (0x1, 0x7, 2),
    (0x2, 0x7, 2),
    (0x2, 0x8, 2),
    (0x3, 0x8, 2),
    (0x3, 0x9, 2),
    (0x4, 0x9, 2),
    (0x4, 0xA, 2),
    (0x5, 0xA, 2),
    //
    (0x7, 0xB, 1),
    (0x8, 0xC, 1),
    (0x9, 0xD, 1),
    (0xA, 0xE, 1),
  ];

  let burrows = [
    vec![0x7, 0xB],
    vec![0x8, 0xC],
    vec![0x9, 0xD],
    vec![0xA, 0xE],
  ];

  let system = System::<0xF>::new(pods, connections, burrows);
  let res = solve(system);
  Ok(if let Some(state) = res { state.cost } else { -1 })
}

fn part2(fname: &str) -> AocResult<i32> {
  let mut pods = parse_input(fname)?;

  let connections = vec![
    (0x0, 0x1, 1),
    (0x1, 0x2, 2),
    (0x2, 0x3, 2),
    (0x3, 0x4, 2),
    (0x4, 0x5, 2),
    (0x5, 0x6, 1),
    //
    (0x1, 0x7, 2),
    (0x2, 0x7, 2),
    (0x2, 0x8, 2),
    (0x3, 0x8, 2),
    (0x3, 0x9, 2),
    (0x4, 0x9, 2),
    (0x4, 0xA, 2),
    (0x5, 0xA, 2),
    //
    (0x07, 0x0F, 1),
    (0x08, 0x10, 1),
    (0x09, 0x11, 1),
    (0x0A, 0x12, 1),
    //
    (0x0F, 0x13, 1),
    (0x10, 0x14, 1),
    (0x11, 0x15, 1),
    (0x12, 0x16, 1),
    //
    (0x13, 0x0B, 1),
    (0x14, 0x0C, 1),
    (0x15, 0x0D, 1),
    (0x16, 0x0E, 1),
  ];

  let burrows = [
    vec![0x07, 0x0F, 0x13, 0x0B],
    vec![0x08, 0x10, 0x14, 0x0C],
    vec![0x09, 0x11, 0x15, 0x0D],
    vec![0x0A, 0x12, 0x16, 0x0E],
  ];
  pods.push(Amphipod { kind: PodKind::from(b'D'), location: 0x0F, id: 8, moves: 0 });
  pods.push(Amphipod { kind: PodKind::from(b'C'), location: 0x10, id: 9, moves: 0 });
  pods.push(Amphipod { kind: PodKind::from(b'B'), location: 0x11, id: 10, moves: 0 });
  pods.push(Amphipod { kind: PodKind::from(b'A'), location: 0x12, id: 11, moves: 0 });
  pods.push(Amphipod { kind: PodKind::from(b'D'), location: 0x13, id: 12, moves: 0 });
  pods.push(Amphipod { kind: PodKind::from(b'B'), location: 0x14, id: 13, moves: 0 });
  pods.push(Amphipod { kind: PodKind::from(b'A'), location: 0x15, id: 14, moves: 0 });
  pods.push(Amphipod { kind: PodKind::from(b'C'), location: 0x16, id: 15, moves: 0 });

  let system = System::<0x17>::new(pods, connections, burrows);
  let res = solve(system);
  Ok(if let Some(state) = res { state.cost } else { -1 })
}

pub fn run(fname: &str) -> AocResult<(impl Display, impl Display)> {
  Ok((part1(fname)?, part2(fname)?))
}
