#!/usr/bin/env rust-script
//! ```cargo
//! [dependencies]
//! ```

use std::fs;
use std::env;
use std::collections::HashMap;

fn upd_neighbors(nei: &mut HashMap<(i32, i32),i32>, r: i32, c: i32, a: i32) {
  let dirs = [(-1, 0), (1, 0), (0, -1), (0, 1), (-1, -1), (-1, 1), (1, -1), (1, 1)];
  for (dr, dc) in dirs {
    *nei.entry((r+dr, c+dc)).or_insert(0) += a;
  }
}

fn main() {
  let input = fs::read_to_string(env::args().nth(1).expect("arg?")).expect("read?");
  let mut nei = HashMap::new();
  let mut ri = 0;
  let mut ci = 0;
  for (r, line) in input.trim().lines().enumerate() {
    ri = r as i32;
    for (c, ch) in line.chars().enumerate() {
      ci = c as i32;
      if ch == '@' {
        nei.entry((ri, ci)).or_insert(0);
        upd_neighbors(&mut nei, ri, ci, 1);
      } else {
        nei.insert((ri, ci), 99999);
      }
    }
  }
  let mut total = 0;
  let mut first = None;
  loop {
    let rm = nei.iter().filter(|(rc, &v)| rc.0 >= 0 && rc.1 >=0 && rc.0 <= ri && rc.1 <= ci && v < 4).map(|(k, _)| *k).collect::<Vec<_>>();
    if rm.is_empty() {
      break;
    }
    total += rm.len();
    first.get_or_insert(total);
    for (r, c) in rm {
      upd_neighbors(&mut nei, r, c, -1);
      nei.insert((r, c), 99999);
    }
  }
  std::println!("part1: {:?}", first.unwrap());
  std::println!("part2: {:?}", total);
}