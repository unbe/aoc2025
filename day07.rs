#!/usr/bin/env rust-script
//! ```cargo
//! [dependencies]
//! ```

use std::fs;
use std::env;
use std::collections::HashMap;

fn main() {
  let input = fs::read_to_string(env::args().nth(1).expect("arg?")).expect("read?");
  let mut lines = input.lines();
  let mut beams = lines.by_ref().next().unwrap().char_indices().filter_map(|(i, c)| (c == 'S').then_some((i, 1))).collect::<HashMap<_,u64>>();
  let mut part1 = 0;
  for line in lines {
    for i in line.char_indices().filter_map(|(i, c)| (c == '^').then_some(i)) {
      if let Some(v) = beams.remove(&i) {
        *beams.entry(i+1).or_insert(0) += v;
        *beams.entry(i-1).or_insert(0) += v;
        part1 += 1;
      }
    }
  }
  std::println!("part1: {}", part1);
  std::println!("part2: {}", beams.values().sum::<u64>()); 
}