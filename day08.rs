#!/usr/bin/env rust-script
//! ```cargo
//! [dependencies]
//! itertools = "0.14"
//! ```

use std::fs;
use std::env;
use std::collections::HashMap;
use std::collections::HashSet;
use itertools::Itertools;

fn dist(a: &[i64], b: &[i64]) -> i64 { 
  a.iter().zip(b.iter()).map(|(&p, &q)| (p - q).pow(2)).sum::<i64>().isqrt().try_into().unwrap()
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
  let input = fs::read_to_string(env::args().nth(1).expect("arg?"))?;
  let points = input.lines().map(|l| l.split(',').map(|i| i.parse::<i64>().unwrap()).collect::<Vec<_>>()).collect::<Vec<_>>();
  let mut dists = points.iter().enumerate().flat_map(|(ai, a)| points.iter().enumerate().filter_map(move |(bi, b)| (ai < bi).then_some((dist(&a, &b), (ai, bi))))).collect::<Vec<_>>();
  dists.sort_unstable();
  let mut owned_sets: Vec<_> = (0..points.len()).map(|i| HashSet::from([i])).collect();
  let mut chains: HashMap<_, _> = (0..points.len()).map(|i| (i, i)).collect();

  let mut last = (0,0);
  for (idx, (_, (ai, bi))) in dists.iter().enumerate() {
    if idx == 1000 {
      println!("part1: {}", chains.values().unique().map(|&c| owned_sets[c].len()).k_largest(3).product::<usize>());
    }
    if chains[&ai] == chains[&bi] {
      continue;
    } 
    last = (*ai, *bi);
    let merged = &owned_sets[chains[ai]] | &owned_sets[chains[bi]];
    for &x in &merged {
      chains.insert(x, owned_sets.len());
    }  
    owned_sets.push(merged);
  }
  println!("part2: {}", points[last.0][0]*points[last.1][0]);
  return Ok(());
}