use std::env;
use std::fs;
use std::io;
use std::time::{Duration, Instant};

use chrono::{Datelike, Utc};
use chrono_tz::US::Eastern;

use advent_of_code::get_day;

fn fmt_time(ms: f64) -> String {
    if ms <= 1.0 {
        let micro_sec = ms * 1000.0;
        return format!("{}µs", micro_sec.round());
    }
    if ms < 1000.0 {
        let whole_ms = ms.floor();
        let rem_ms = ms - whole_ms;
        return format!("{}ms ", whole_ms) + &fmt_time(rem_ms);
    }
    let sec: f64 = ms / 1000.0;
    if sec < 60.0 {
        let whole_sec = sec.floor();
        let rem_ms = ms - whole_sec * 1000.0;
        return format!("{}s ", whole_sec) + &fmt_time(rem_ms);
    }
    let min: f64 = sec / 60.0;
    format!("{}m ", min.floor()) + &fmt_time((sec % 60.0) * 1000.0)
}

fn fmt_dur(dur: Duration) -> String {
    fmt_time(dur.as_secs_f64() * 1000.0)
}

fn main() {
    // Get day string
    let args: Vec<String> = env::args().collect();
    let mut day = String::new();

    let today = Utc::now().with_timezone(&Eastern).day();

    if args.len() >= 2 {
        day = args[1].clone();
    } else {
        println!("Enter day: ");
        io::stdin()
            .read_line(&mut day)
            .expect("Failed to read line");
    }
    // Parse day as number
    day = day.trim().to_string();
    let day_num: u32 = match day.parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid day number: {}", day);
            return;
        }
    };
    // Read input file
    let cwd = env::current_dir().unwrap();
    let filename = cwd.join("inputs").join(format!("day{:02}.txt", day_num));
    println!("Reading {}", filename.display());
    let input = fs::read_to_string(filename).expect("Error while reading");

    // Get corresponding function

    let to_run = get_day(day_num);
    // Time it
    if let Some(fun) = to_run.0 {
        println!("Running Part 1");
        let part1_start = Instant::now();
        fun(input.clone());
        let part1_dur = part1_start.elapsed();
        println!("Took {}", fmt_dur(part1_dur));
    }

    if let Some(fun) = to_run.1 {
        println!("Running Part 2");
        let part2_start = Instant::now();
        fun(input);
        let part2_dur = part2_start.elapsed();
        println!("Took {}", fmt_dur(part2_dur));
    }
}
