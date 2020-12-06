
use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day5)]
fn seat_ids(input: &str) -> Vec<i32> {
  input.lines().map(|f| {
    let (row, _, col, _) = f.chars().fold((0, 127, 0, 7), |(begin, end, left, right), ch| {
      match ch {
        'F' => (begin, (end - begin) / 2 + begin, left, right),
        'B' => ((end - begin) / 2 + begin + 1, end, left, right),
        'L' => (begin, end, left, (right - left) / 2 + left),
        'R' => (begin, end, (right - left) / 2 + left + 1, right),
        _ => unreachable!()
      }
    });
    row * 8 + col
  }).collect()
}

#[aoc(day5, part1)]
fn part1(input: &[i32]) -> String {
  input.iter().max().unwrap().to_string()
}

#[aoc(day5, part2)]
fn part2(input: &[i32]) -> String {
  let mut seats: Vec<&i32> = input.iter().collect();
  seats.sort_unstable();
  let mut peekable_seats = input.iter().peekable();

  while let Some(seat) = peekable_seats.next() {
    if &&(seat + 1) != peekable_seats.peek().unwrap() {
      return (seat + 1).to_string()
    }
  }
  unreachable!("");
}
