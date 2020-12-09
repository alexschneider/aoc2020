#![allow(unreachable_patterns)]
use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Clone)]
enum Instruction {
  Acc(i32),
  Jmp(i32),
  Nop(i32),
}
#[aoc_generator(day8)]
fn program(input: &str) -> Vec<Instruction> {
  input
    .lines()
    .map(|x| {
      let spl: Vec<&str> = x.split(' ').collect();
      let num = spl[1].parse::<i32>().unwrap();
      match spl[0] {
        "acc" => Instruction::Acc(num),
        "jmp" => Instruction::Jmp(num),
        "nop" => Instruction::Nop(num),
        _ => unreachable!(),
      }
    })
    .collect()
}

#[aoc(day8, part1)]
fn part1(program: &[Instruction]) -> String {
  let mut visited = vec![false; program.len()];
  let mut acc: i32 = 0;
  let mut op_index: i32 = 0;
  loop {
    visited[op_index as usize] = true;
    match program[op_index as usize] {
      Instruction::Nop(_) => op_index += 1,
      Instruction::Acc(num) => {
        op_index += 1;
        acc += num as i32
      }
      Instruction::Jmp(num) => op_index += num,
    }
    if visited[op_index as usize] {
      break;
    }
  }
  acc.to_string()
}

#[aoc(day8, part2)]
fn part2(program: &[Instruction]) -> String {
  let mut visited = vec![false; program.len()];
  let mut acc: i32 = 0;
  let mut op_index: i32 = 0;
  let mut state = Vec::new();
  let mut brute_force = false;
  loop {
    visited[op_index as usize] = true;
    match program[op_index as usize] {
      Instruction::Nop(_) => {
        if !brute_force {
          state.push((op_index, acc, visited.clone()));
        }
        op_index += 1;
      }
      Instruction::Acc(num) => {
        op_index += 1;
        acc += num as i32
      }
      Instruction::Jmp(num) => {
        if !brute_force {
          state.push((op_index, acc, visited.clone()));
        }
        op_index += num;
      }
      Instruction::Nop(_) => unreachable!(),
    }
    if op_index as usize >= program.len() {
      break;
    }
    if visited[op_index as usize] {
      let (index, accum, visit) = state.pop().unwrap();
      op_index = index;
      acc = accum;
      visited = visit;
      match program[index as usize] {
        Instruction::Jmp(_) => op_index += 1,
        Instruction::Nop(n) => op_index += n,
        _ => unreachable!(),
      }
      brute_force = true;
    }
  }

  acc.to_string()
}
