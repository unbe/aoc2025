#!/usr/bin/env rust-script
//! ```cargo
//! [dependencies]
//! itertools = "0.13.0"
//! ```

use std::fs;
use std::env;
use itertools::Itertools;

fn main() {
  let input = fs::read_to_string(env::args().nth(1).expect("arg?")).expect("read?");
  let mut it = input.lines();
  let first_part = it.by_ref().take_while(|l| !l.trim().is_empty());
  let atoi = |x: &str| x.parse::<i64>().unwrap();
  let ranges = first_part.map(|x| x.split("-").map(atoi).collect_tuple::<(i64, i64)>().unwrap()).sorted_unstable().collect::<Vec<_>>();
  let mut merged = vec![];
  let mut acc = None;
  for (ra, rb) in ranges {
    if let Some((a,b)) = acc {
      if ra <= b {
        acc = Some((a, b.max(rb)));
        continue;
      } else {
        merged.push((a,b));
      }
    }
    acc = Some((ra, rb));
  }
  merged.push(acc.unwrap());

  let fresh = it.map(atoi).filter(|r| merged.iter().any(|(a,b)| r >= a && r <= b)).count();
  std::println!("part1: {}", fresh);
  let total = merged.iter().map(|(a,b)| b - a + 1).sum::<i64>();
  std::println!("part2: {}", total);
}