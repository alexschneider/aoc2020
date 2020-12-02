use regex::Regex;
use aoc_runner_derive::{aoc,aoc_generator};

#[derive(Debug)]
struct PasswordPolicy {
  num1: usize,
  num2: usize,
  letter: char,
  password: String
}

#[aoc_generator(day2)]
fn input_gen(input: &str) -> Vec<PasswordPolicy> {
  let re = Regex::new(r"(\d+)-(\d+) (\w): (\w+)").unwrap();
  input.lines().map(|line: &str| {
    if let Some(captures) = re.captures(line) {
      PasswordPolicy {
        num1: captures.get(1).unwrap().as_str().parse().unwrap(),
        num2: captures.get(2).unwrap().as_str().parse().unwrap(),
        letter: captures.get(3).unwrap().as_str().parse().unwrap(),
        password: captures.get(4).unwrap().as_str().to_string()
      }
    } else {
      panic!("Unexpected line: {}", line);
    }
  }).collect()
}

#[aoc(day2, part1)]
fn part1(input: &[PasswordPolicy]) -> String {
  input.iter().filter(|policy| {
    let num_chars = policy.password.matches(policy.letter).count();
    num_chars >= policy.num1 && num_chars <= policy.num2
  }).count().to_string()
}
#[aoc(day2, part2)]
fn part2(input: &[PasswordPolicy]) -> String {
  input.iter().filter(|policy| {
    let pw: Vec<char> = policy.password.chars().collect();
    let first_match = pw[policy.num1 - 1] == policy.letter;
    let second_match = pw[policy.num2 - 1] == policy.letter;
    first_match ^ second_match
  }).count().to_string()
}