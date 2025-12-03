#!/usr/bin/env rust-script
//! ```cargo
//! [dependencies]
//! ```

use std::fs;
use std::env;

fn maxchar(s: &str) -> (usize, char) {
  s.chars().enumerate().max_by(|(ai, a), (bi, b)| a.cmp(b).then(bi.cmp(ai))).unwrap()
}

fn solve(d: usize, bank: &str) -> u64 {
  let mut prev = 0;
  let mut jolt : u64 = 0;
  for i in 0..d {
    let dig = maxchar(&bank[prev..bank.len() - d + i + 1]);
    prev += dig.0 + 1;
    jolt = jolt * 10 + dig.1.to_digit(10).unwrap() as u64;
  }
  return jolt;
}

fn main() {
  let input = fs::read_to_string(env::args().nth(1).expect("arg?")).expect("read?");
  let mut part1 = 0;
  let mut part2 = 0;
  for bank in input.trim().split("\n") {
    part1 += solve(2, bank);
    part2 += solve(12, bank);
  }
  std::println!("part1: {}", part1);
  std::println!("part2: {}", part2);
}