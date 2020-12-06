
use aoc_runner_derive::{aoc};
use std::collections::HashSet;



#[aoc(day6, part1)]
fn part1(input: &str) -> String {
  input.split("\n\n").map(|group| {
    group.chars().filter(|c| c != &'\n').collect::<HashSet<char>>().len()
  }).sum::<usize>().to_string()
}

#[aoc(day6, part2)]
fn part2(input: &str) -> String {
  input.split("\n\n").map(|group| {
    let mut it = group.lines().map(|x| x.chars().collect::<HashSet<char>>());
    let hs = it.next().unwrap();
    it.fold(hs, |left, right| left.intersection(&right).cloned().collect::<HashSet<char>>()).len()
  }).sum::<usize>().to_string()
}
