use aoc_runner_derive::{aoc,aoc_generator};

#[derive(Default)]
#[derive(Debug)]
struct Passport {
  byr: String,
  iyr: String,
  eyr: String,
  hgt: String,
  hcl: String,
  ecl: String,
  pid: String,
  cid: String
}

#[aoc_generator(day4)]
fn input_gen(input: &str) -> Vec<Passport> {
  input.split("\n\n").map(|pp| {
    let mut passport: Passport = Default::default();
    let pp_items = pp.split_ascii_whitespace();
    for item in pp_items {
      let item_split: Vec<&str> = item.split(':').collect();
      let (item_name, item_val) = (item_split[0], item_split[1]);
      match item_name {
        "byr" => passport.byr = item_val.to_string(),
        "iyr" => passport.iyr = item_val.to_string(),
        "eyr" => passport.eyr = item_val.to_string(),
        "hgt" => passport.hgt = item_val.to_string(),
        "hcl" => passport.hcl = item_val.to_string(),
        "ecl" => passport.ecl = item_val.to_string(),
        "pid" => passport.pid = item_val.to_string(),
        "cid" => passport.cid = item_val.to_string(),
        _ => unreachable!()
      };
    }
    passport
  }).collect()
}

#[aoc(day4, part1)]
fn part1(input: &[Passport]) -> String {
  input.iter().filter(|pp| {
    !(pp.byr.is_empty() || pp.iyr.is_empty() || pp.eyr.is_empty() || pp.hgt.is_empty() || pp.hcl.is_empty() || pp.ecl.is_empty() || pp.pid.is_empty() )
  }).count().to_string()
}

#[aoc(day4, part2)]
fn part2(input: &[Passport]) -> String {
  input.iter().filter(|pp| {
    !(pp.byr.is_empty() || pp.iyr.is_empty() || pp.eyr.is_empty() || pp.hgt.is_empty() || pp.hcl.is_empty() || pp.ecl.is_empty() || pp.pid.is_empty() )
  }).filter(|pp| {
    let byr = pp.byr.parse::<u32>().unwrap();
    if byr > 2002 || byr < 1920 {
      return false;
    }
    let iyr = pp.iyr.parse::<u32>().unwrap();
    if iyr < 2010 || iyr > 2020 {
      return false;
    }
    let eyr = pp.eyr.parse::<u32>().unwrap();
    if eyr < 2020 || eyr > 2030 {
      return false;
    }
    let hgt = pp.hgt.trim_end_matches("in").trim_end_matches("cm").parse::<u32>().unwrap_or_default();
    if pp.hgt.ends_with("cm") {
      if hgt < 150 || hgt > 193 {
        return false;
      }
    } else if hgt < 59 || hgt > 76 {
      return false;
    }
    if pp.hcl.len() != 7 || !pp.hcl.starts_with('#') {
      return false;
    } else {
      for ch in pp.hcl.trim_start_matches('#').chars() {
        if (ch >= '0' && ch <= '9') || (ch >= 'a' && ch <= 'f') {
          continue;
        }
        return false;
      }
    }
    match pp.ecl.as_str() {
      "amb" | "blu" | "brn" | "gry" | "grn" | "hzl" | "oth" => (),
      _ => return false
    }
    if pp.pid.len() != 9 {
      return false;
    }
    for ch in pp.pid.chars() {
      if ch < '0' || ch > '9' {
        return false;
      }
    }
     true
  }).count().to_string()
}
