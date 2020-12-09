use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::HashSet;

#[aoc_generator(day9)]
fn gen(input: &str) -> Vec<u64> {
  input.lines().map(|x| x.parse::<u64>().unwrap()).collect()
}

#[aoc(day9, part1)]
fn part1(xmas: &[u64]) -> String {
  for x in 25..xmas.len() {
    let slice: HashSet<u64> = xmas.iter().skip(x - 25).take(25).copied().collect();
    if !find_num(xmas[x], slice) {
      return xmas[x].to_string();
    }
  }
  "".to_string()
}

fn find_num(num: u64, slice: HashSet<u64>) -> bool {
  for y in slice.iter() {
    let num2 = *y;
    if slice.contains(&(num - num2)) {
      return true;
    }
  }
  false
}

#[aoc(day9, part2)]
fn part2(input: &[u64]) -> String {
  let xmas: Vec<u64> = input[0..500].iter().copied().collect();
  let nums = find_num2(xmas);
  let min = nums.iter().min().unwrap();
  let max = nums.iter().max().unwrap();
  format!("{}", min + max)
}

fn find_num2(slice: Vec<u64>) -> Vec<u64> {
  let mut len = 1;
  let mut skip = 0;
  loop {
    let attempt: Vec<u64> = slice.iter().skip(skip).take(len).copied().collect();
    if attempt.iter().sum::<u64>() == 14360655 {
      return attempt;
    }
    skip += 1;
    if skip == slice.len() {
      skip = 0;
      len += 1;
    }
  }
}
