use aoc_runner_derive::{aoc,aoc_generator};

#[aoc_generator(day3)]
fn input_gen(input: &str) -> Vec<Vec<bool>> {
  input.lines().map(|line| {
    line.chars().map(|char| {
      char == '.'
    }).collect()
  }).collect()
}



fn num_trees(input: &[Vec<bool>], x_step: usize, y_step: usize) -> u32 {
  let mut x = 0;
  let mut y = 0;
  let mut trees = 0;
  while y < input.len() {
    if !input[y][x] {
      trees += 1;
    }
    x += x_step;
    x %= input[y].len();
    y += y_step;
  }
  trees
}

#[aoc(day3, part1)]
fn part1(input: &[Vec<bool>]) -> String {
  num_trees(input, 3, 2).to_string()
}

#[aoc(day3, part2)]
fn part2(input: &[Vec<bool>]) -> String {
  (num_trees(input, 1, 1) 
  * num_trees(input, 3, 1) 
  * num_trees(input, 5, 1) 
  * num_trees(input, 7, 1) 
  * num_trees(input, 1, 2)).to_string()
}
