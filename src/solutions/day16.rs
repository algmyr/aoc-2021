use std::fmt::Display;

use itertools::Itertools;

use crate::error::{AocError, AocResult};

type ArithInt = i64;
type ParseInt = i64;

// Bitstream impl
struct BitStream {
  i: usize,
  data: Vec<char>,
}

impl BitStream {
  fn new(hex_string: String) -> BitStream {
    BitStream {
      i: 0,
      data: hex_string.chars().collect_vec(),
    }
  }
  fn cur_pos(&self) -> usize { self.i }
  fn read_int(&mut self, n: usize) -> ParseInt {
    self
      .take(n)
      .fold(0, |accu, el| (accu as ParseInt) << 1 | (el as ParseInt))
  }
}

impl Iterator for BitStream {
  type Item = bool;
  fn next(&mut self) -> Option<Self::Item> {
    let (i, offset) = (self.i / 4, self.i % 4);
    let digit = self.data.get(i)?.to_digit(16)?;
    self.i += 1;
    return Some(((digit >> (3 - offset)) & 1) == 1);
  }
}

// AST node

#[allow(dead_code)]
#[derive(Debug)]
#[repr(u8)]
enum NodeType {
  Sum = 0,
  Prod = 1,
  Min = 2,
  Max = 3,
  Literal = 4,
  Greater = 5,
  Less = 6,
  Equal = 7,
}

impl NodeType {
  fn from_u8(n: u8) -> NodeType {
    // This is a bad idea
    unsafe { std::mem::transmute(n) }
  }
}

#[derive(Debug)]
enum Node {
  Literal {
    version: u8,
    #[allow(dead_code)]
    node_type: NodeType,
    value: ArithInt,
  },
  Operator {
    version: u8,
    node_type: NodeType,
    subpackages: Vec<Node>,
  },
}

fn parse(bs: &mut BitStream) -> Node {
  fn read_varint(bs: &mut BitStream) -> ArithInt {
    // 1+4 bit int encoding
    let mut res = 0;
    loop {
      let x = bs.read_int(5);
      res = res << 4 | (x & 0b1111);
      if x & 0b10000 == 0 {
        break;
      }
    }
    res
  }

  fn subpkg_by_len(mut bs: &mut BitStream) -> Vec<Node> {
    let bit_len = bs.read_int(15) as usize;
    let start = bs.cur_pos();
    let mut subpackages = vec![];
    while bs.cur_pos() < start + bit_len {
      subpackages.push(parse(&mut bs));
    }
    subpackages
  }

  fn subpkg_by_count(mut bs: &mut BitStream) -> Vec<Node> {
    let n_sub_packages = bs.read_int(11);
    (0..n_sub_packages).map(|_| parse(&mut bs)).collect_vec()
  }

  let version = bs.read_int(3) as u8;
  let node_type = NodeType::from_u8(bs.read_int(3) as u8);
  match node_type {
    NodeType::Literal => Node::Literal {
      version,
      node_type,
      value: read_varint(bs),
    },
    _ => {
      let subpackages: Vec<Node> = match bs.read_int(1) {
        0 => subpkg_by_len(bs),
        1 => subpkg_by_count(bs),
        _ => panic!("impossible"),
      };

      Node::Operator {
        version,
        node_type,
        subpackages,
      }
    }
  }
}

fn parse_input(fname: &str) -> AocResult<Node> {
  let first_line = std::fs::read_to_string(fname)?
    .lines()
    .next()
    .ok_or(AocError::Custom("Input parse failed".to_owned()))?
    .to_owned();
  let mut bs = BitStream::new(first_line);
  Ok(parse(&mut bs))
}

fn part1(fname: &str) -> AocResult<i32> {
  let root = parse_input(fname)?;

  fn traverse(node: &Node) -> i32 {
    match node {
      Node::Literal { version, .. } => *version as i32,
      Node::Operator {
        version,
        subpackages,
        ..
      } => (*version as i32) + subpackages.iter().map(|x| traverse(x)).sum::<i32>(),
    }
  }

  Ok(traverse(&root))
}

fn part2(fname: &str) -> AocResult<ArithInt> {
  let root = parse_input(fname)?;

  fn traverse(node: &Node) -> ArithInt {
    match node {
      Node::Literal { value, .. } => *value,
      Node::Operator {
        node_type: type_id,
        subpackages,
        ..
      } => {
        let sub_eval = subpackages.iter().map(|x| traverse(x)).collect_vec();
        match *type_id {
          NodeType::Sum => sub_eval.into_iter().sum(),      // sum
          NodeType::Prod => sub_eval.into_iter().product(), // prod
          NodeType::Min => sub_eval.into_iter().min().expect("no elements"), // minimum
          NodeType::Max => sub_eval.into_iter().max().expect("no elements"), // maximum
          NodeType::Greater => (sub_eval[0] > sub_eval[1]).into(), // greater than
          NodeType::Less => (sub_eval[0] < sub_eval[1]).into(), // less than
          NodeType::Equal => (sub_eval[0] == sub_eval[1]).into(), // equal_to
          _ => panic!("Unknown type"),
        }
      }
    }
  }

  Ok(traverse(&root))
}

pub fn run(fname: &str) -> AocResult<(impl Display, impl Display)> {
  Ok((part1(fname)?, part2(fname)?))
}
