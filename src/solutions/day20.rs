use std::fmt::Display;

use itertools::Itertools;

use crate::error::{aoc_error, AocResult};

#[derive(Clone, Debug)]
struct Image {
  height: usize,
  width: usize,
  data: Vec<Vec<bool>>,
  codes: Vec<Vec<u16>>,
}

impl Image {
  fn from_string(s: &str, margin: usize) -> Self {
    let lines = s.lines().collect_vec();
    let height = lines.len() + 2 * margin;
    let width = lines[0].len() + 2 * margin;

    let mut image = vec![vec![false; height]; width];
    lines.iter().enumerate().for_each(|(i, line)| {
      line
        .bytes()
        .enumerate()
        .for_each(|(j, b)| image[margin + i][margin + j] = b == b'#')
    });
    let codes = vec![vec![0; height]; width];

    Self {
      height,
      width,
      data: image,
      codes,
    }
  }

  fn update_codes(&mut self) {
    for i in 1..self.height-1 {
      for j in 1..self.width-1 {
        self.codes[i][j] = 0;
      }
    }
    // 8 7 6   6 6 6
    // 5 4 3 = 3 3 3 + shifts
    // 2 1 0   0 0 0
    let mut vec = vec![0u16; self.width];
    for i in 0..2 {
      for j in 0..self.width {
        vec[j] = vec[j] << 3 | self.data[i][j] as u16;
      }
    }
    for i in 2..self.height {
      for j in 0..self.width {
        vec[j] = (vec[j] & 0b111111) << 3 | self.data[i][j] as u16;
      }
      let mut res: u16 = vec[0] << 1 | vec[1];
      for j in 2..self.width {
        // Unset bit 8 5 2 and shift.
        res = (0b011011011 & res) << 1 | vec[j];
        self.codes[i-1][j-1] = res;
      }
    }
    //for i in 1..self.height-1 {
    //  for j in 1..self.width-1 {
    //    let mut res = 0u16;
    //    for ii in i-1..=i+1 {
    //      for jj in j-1..=j+1 {
    //        res = res << 1 | self.data[ii][jj] as u16;
    //      }
    //    }
    //    self.codes[i][j] = res;
    //  }
    //}
  }

  fn get_code_at(&self, i: usize, j: usize) -> usize {
    self.codes[i][j] as usize
  }
}

// HashMap<(i32, i32)
fn parse_input(fname: &str, margin: usize) -> AocResult<(Vec<bool>, Image)> {
  let s = std::fs::read_to_string(fname)?;

  if let [first, second] = s.split("\n\n").collect_vec()[..] {
    let enc_str = first.bytes().map(|b| b == b'#').collect_vec();
    Ok((enc_str, Image::from_string(second, margin)))
  } else {
    Err(aoc_error("Wrong number of sections"))
  }
}

fn solve(fname: &str) -> AocResult<(i32, i32)> {
  let margin = 102;
  let (enc_str, mut image) = parse_input(fname, margin)?;

  fn update(enc_str: &[bool], image: &mut Image) {
    image.update_codes();
    for i in 1..image.height - 1 {
      for j in 1..image.width - 1 {
        image.data[i][j] = enc_str[image.get_code_at(i, j)]
      }
    }
  }

  let result = |image: &Image| {
    let hmargin = margin/2;
    let mut res = 0;
    for i in hmargin..image.height - hmargin {
      for j in hmargin..image.width - hmargin {
        res += (image.data[i][j] == true) as i32;
      }
    }
    res
  };

  for _ in 0..2 { update(enc_str.as_ref(), &mut image); }
  let res1 = result(&image);
  for _ in 2..50 { update(enc_str.as_ref(), &mut image); }
  let res2 = result(&image);

  Ok((res1, res2))
}
pub fn run(fname: &str) -> AocResult<(impl Display, impl Display)> { solve(fname) }
