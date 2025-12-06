#!/usr/bin/env rust-script
//! ```cargo
//! [dependencies]
//! ```

use std::fs;
use std::env;

fn main() {
  let input = fs::read_to_string(env::args().nth(1).expect("arg?")).expect("read?");
  let mut lines = input.lines().collect::<Vec<_>>();
  let ops = lines.pop().unwrap().chars().enumerate().filter(|(_, ch)| *ch != ' ');
  let width = lines.iter().map(|x| x.len()).max().unwrap();
  let sentinel = Some((width + 1, 'X'));
  let ops_range = ops.clone().zip(ops.skip(1).chain(sentinel)).map(|((col, op), (col2, _))| (col, col2, op));
  let mut part1 = 0;
  let mut part2 = 0;
  for (idx, end, op) in ops_range {
    let hz_args = lines.iter().map(|x| x[idx..end.min(x.len())].trim().parse::<u64>().unwrap());
    let vert_args = (idx..end-1).map(|c| lines.iter().map(|l| l.chars().nth(c).unwrap_or(' ')).filter(|&c| c != ' ').collect::<String>().parse::<u64>().unwrap());
    let (p1, p2) : (u64, u64) = match op {
      '+' => (hz_args.sum(), vert_args.sum()),
      '*' => (hz_args.product(), vert_args.product()),
      _ => panic!("unknown op"),
    };
    part1 += p1;
    part2 += p2;
  }
  std::println!("part1: {}", part1);
  std::println!("part2: {}", part2);
}
