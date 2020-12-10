use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::BTreeSet;

#[aoc_generator(day10)]
fn gen(input: &str) -> BTreeSet<u32> {
  input.lines().map(|x| x.parse::<u32>().unwrap()).collect()
}

#[aoc(day10, part1)]
fn part1(input: &BTreeSet<u32>) -> String {
  let mut one_jolt = 0;
  let mut three_jolt = 1;
  let mut prev = 0;

  for jolt in input {
    match jolt - prev {
      1 => one_jolt += 1,
      2 => (),
      3 => three_jolt += 1,
      _ => unreachable!(),
    }
    prev = *jolt;
  }
  (one_jolt * three_jolt).to_string()
}
#[aoc(day10, part2)]
fn part2(input: &BTreeSet<u32>) -> String {
  let mut items: Vec<u32> = input.iter().copied().collect();
  println!("{:?}", items);
  dynamic(items);
  "".to_string()
}

fn dynamic(input: Vec<u32>) {
  let len = input.len();
  let mut arr = vec![vec![0; len]; 3];

  for x in (0..len).rev() {
    for to_remove in 1..=3 {
      if try_remove(&input, x, to_remove) {
        arr[to_remove - 1][x] = 1;
      }
      if x < len - 1 {
        arr[to_remove - 1][x] += arr[to_remove - 1][x + 1];
      }
      // if to_remove > 1 && arr[to_remove - 2][x] > arr[to_remove - 1][x] {
      //   arr[to_remove - 1][x] += arr[to_remove - 2][x];
      // }
    }
  }
  println!("{:?}", arr);
}

fn try_remove(input: &[u32], idx: usize, to_remove: usize) -> bool {
  if idx + to_remove >= input.len() {
    false
  } else {
    let first = if idx == 0 { 0 } else { input[idx - 1] };
    let second = input[idx + to_remove];
    println!("{} {} {}", first, second, second - first <= 3);
    second - first <= 3
  }
}
