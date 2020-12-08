use aoc_runner_derive::{aoc};
use std::collections::HashSet;



#[aoc(day8, part1)]
fn part1(input: &str) -> String {
  let program: Vec<(&str, i32)> = input.lines().map(|x| {
    let spl: Vec<&str> = x.split(' ').collect();
    (spl[0], spl[1].parse::<i32>().unwrap())
  }).collect();
  let mut visited = HashSet::new();
  let mut acc: i32 = 0;
  let mut op_index: i32 = 0;
  loop {
    visited.insert(op_index);
    let (op, num) = program[op_index as usize];
    match op {
      "nop" => op_index += 1,
      "acc" => {op_index += 1; acc += num as i32},
      "jmp" => op_index += num,
      _ => unreachable!()
    }
    if visited.contains(&op_index) {
      break;
    }
  }
  acc.to_string()
}

#[aoc(day8, part2)]
fn part2(input: &str) -> String {
  let program: Vec<(&str, i32)> = input.lines().map(|x| {
    let spl: Vec<&str> = x.split(' ').collect();
    (spl[0], spl[1].parse::<i32>().unwrap())
  }).collect();

  let mut visited = HashSet::new();
  let mut acc: i32 = 0;
  let mut op_index: i32 = 0;
  let mut state = Vec::new();
  let mut brute_force = false;
  loop {
    visited.insert(op_index);
    let (op, num) = program[op_index as usize];
    match op {
      "nop" => {
        if !brute_force {
          state.push((op_index, acc, visited.clone())); 
        }
        op_index += 1; 
      },
      "acc" => {op_index += 1; acc += num as i32},
      "jmp" => {
        if !brute_force {
          state.push((op_index, acc, visited.clone())); 
        }
        op_index += num; 
      },
      _ => unreachable!()
    }
    if visited.contains(&op_index) {
      let (index, accum, visit) = state.pop().unwrap();
      op_index = index;
      acc = accum;
      visited = visit;
      let (o, n) = program[index as usize];
      match o {
        "jmp" => {op_index += 1},
        "nop" => {op_index += n},
        _ => unreachable!()
      }
      brute_force = true;
    }
    if op_index as usize >= program.len() {
      break;
    }
  }

  acc.to_string()
}
