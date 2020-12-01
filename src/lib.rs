// Days
pub mod day01;

pub type DayFn = fn(String);

pub fn get_day(day: u32) -> (Option<DayFn>, Option<DayFn>) {
    match day {
        1 => (Some(day01::part1), Some(day01::part2)),
        _ => {
            println!("Unknown day: {}", day);
            (None, None)
        }
    }
}
