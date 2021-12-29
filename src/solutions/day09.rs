use std::fmt::Display;

use crate::error::AocResult;

fn parse_input(fname: &str) -> AocResult<Vec<Vec<i8>>> {
  let content = std::fs::read_to_string(fname)?;
  content
    .lines()
    .map(|s| {
      s.chars()
        .map(|c: char| Ok(c.to_digit(10).unwrap() as i8))
        .collect()
    })
    .collect()
}

fn basin_locations(width: i32, height: i32, vec: &Vec<Vec<i8>>) -> Vec<(i32, i32)> {
  let value_at = |x: i32, y: i32| -> i8 {
    if x < 0 || y < 0 || x >= width || y >= height {
      100
    } else {
      vec[y as usize][x as usize]
    }
  };
  let mut basins: Vec<(i32, i32)> = vec![];
  for y in 0..height {
    for x in 0..width {
      let v = value_at(x, y);
      if v < value_at(x - 1, y)
        && v < value_at(x + 1, y)
        && v < value_at(x, y - 1)
        && v < value_at(x, y + 1)
      {
        basins.push((x, y));
      }
    }
  }
  basins
}

fn part1(fname: &str) -> AocResult<i32> {
  let vec = parse_input(fname)?;
  let height: i32 = vec.len() as i32;
  let width: i32 = vec[0].len() as i32;

  let value_at = |x: i32, y: i32| -> i8 {
    if x < 0 || y < 0 || x >= width || y >= height {
      100
    } else {
      vec[y as usize][x as usize]
    }
  };

  let basins = basin_locations(width, height, &vec);

  let mut res: i32 = 0;
  for (x, y) in basins {
    res += (value_at(x, y) as i32) + 1;
  }
  Ok(res)
}

fn part2(fname: &str) -> AocResult<i32> {
  let mut vec = parse_input(fname)?;
  let height: i32 = vec.len() as i32;
  let width: i32 = vec[0].len() as i32;

  let value_at = |vec: &Vec<Vec<i8>>, x: i32, y: i32| -> i8 {
    if x < 0 || y < 0 || x >= width || y >= height {
      100
    } else {
      vec[y as usize][x as usize]
    }
  };

  fn set_value(vec: &mut Vec<Vec<i8>>, x: i32, y: i32) -> &mut i8 {
    &mut vec[y as usize][x as usize]
  }

  let basins = basin_locations(width, height, &vec);

  let mut basin_sizes: Vec<i32> = vec![];

  for basin in basins {
    let mut stack: Vec<(i32, i32)> = vec![basin];
    let mut basin_size = 0;
    while !stack.is_empty() {
      let (x, y) = stack.pop().unwrap();

      if value_at(&vec, x, y) >= 9 {
        continue;
      }
      *set_value(&mut vec, x, y) = 9;
      basin_size += 1;

      for (dx, dy) in [(0, 1), (0, -1), (1, 0), (-1, 0)] {
        stack.push((x + dx, y + dy))
      }
    }
    basin_sizes.push(basin_size);
  }

  basin_sizes.sort_by(|a, b| b.cmp(a));
  Ok(basin_sizes.into_iter().take(3).product())
}

pub fn run(fname: &str) -> AocResult<(impl Display, impl Display)> {
  Ok((part1(fname)?, part2(fname)?))
}
