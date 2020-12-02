use std::collections::HashSet;
use std::iter::Iterator;

use aoc_runner_derive::aoc;

fn get_nums(inp: &str) -> HashSet<i32> {
  inp.lines().map(|x| x.parse().unwrap()).collect()
}

#[aoc(day1, part1)]
pub fn part1(inp: &str) -> String {
    let nums = get_nums(inp);
    let mut hs = HashSet::new();
    for num in nums {
      let target_num = 2020 - num;
      if hs.get(&target_num).is_some() {
        return format!("Part 1: Numbers are {} and {}. Product is {}", num, target_num, num * target_num);
      } else {
        hs.insert(num);
      }
    }
    unreachable!()
}

#[aoc(day1, part2)]
pub fn part2(inp: &str) -> String {
    let nums = get_nums(inp);
    for (index, num) in nums.iter().enumerate() {
      for num_pair in nums.iter().skip(index) {
        let target_num = 2020 - (num_pair + num);
        if nums.get(&target_num).is_some() {
          return format!("Numbers are {}, {}, and {}. Product is {}", num, num_pair, target_num, num * num_pair * target_num);
        }
      }
    }
    unreachable!()
}