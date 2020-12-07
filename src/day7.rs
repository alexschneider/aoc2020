use std::collections::{HashMap, HashSet};

use aoc_runner_derive::{aoc};



#[aoc(day7, part1)]
fn part1(input: &str) -> String {
  let bag_map: HashMap<&str, HashMap<String, u32>> = input.lines().map(|line| {
    let split: Vec<&str> = line.split("s contain ").collect();
    let bag = split[0];
    if split[1] == "no other bags." {
      (bag, HashMap::new())
    } else {
      let contains : HashMap<String, u32> = split[1].split(", ").map(|bag| {
        let bag_num = bag.matches(char::is_numeric).fold("".to_string(), |i, c| [i, c.to_string()].join(""));
        let bag_name = bag.trim_start_matches(bag_num.as_str()).trim_end_matches('.').trim().replace("bags", "bag");
        (bag_name, bag_num.parse::<u32>().unwrap())
      }).collect();
      (bag, contains)
    }
  }).collect();
  
  let mut inverted_map: HashMap<String, Vec<&str>> = HashMap::new();
  for (bag, contains) in bag_map {
    for (bag_contained, _) in contains {
      let contained = bag_contained.clone();
      if !inverted_map.contains_key(contained.as_str()) {
        inverted_map.insert(bag_contained.clone(), Vec::new());
      }
      let new_vec = inverted_map.get_mut(bag_contained.as_str()).unwrap();
      new_vec.push(bag);
    }
  }
  let bags = find_num_bags(inverted_map, "shiny gold bag");
  (bags.len() - 1).to_string()
}

fn find_num_bags(inverted_map: HashMap<String, Vec<&str>>, bag_name: &str) -> HashSet<String> {
  if let Some(current_bags) = inverted_map.get(bag_name) {
    let mut hs = HashSet::new();
    hs.insert(bag_name.to_string());
    current_bags.iter().map(|name| find_num_bags(inverted_map.clone(), &name)).fold(hs, |x, y| x.union(&y).cloned().collect())
  } else {
    let mut hs = HashSet::new();
    hs.insert(bag_name.to_string());
    hs
  }
}

#[aoc(day7, part2)]
fn part2(input: &str) -> String {
  let bag_map: HashMap<&str, HashMap<String, u32>> = input.lines().map(|line| {
    let split: Vec<&str> = line.split("s contain ").collect();
    let bag = split[0];
    if split[1] == "no other bags." {
      (bag, HashMap::new())
    } else {
      let contains : HashMap<String, u32> = split[1].split(", ").map(|bag| {
        let bag_num = bag.matches(char::is_numeric).fold("".to_string(), |i, c| [i, c.to_string()].join(""));
        let bag_name = bag.trim_start_matches(bag_num.as_str()).trim_end_matches('.').trim().replace("bags", "bag");
        (bag_name, bag_num.parse::<u32>().unwrap())
      }).collect();
      (bag, contains)
    }
  }).collect();
  (find_num_bags2(bag_map, "shiny gold bag".to_string()) - 1).to_string()
}

fn find_num_bags2(bag_map: HashMap<&str, HashMap<String, u32>>, bag_name: String) -> u32 {
  let bag_contains = bag_map.get(bag_name.as_str()).unwrap();
  if bag_contains.is_empty() {
    1
  } else {
    let mut counter = 1;
    for (k, v) in bag_contains {
      let bags = find_num_bags2(bag_map.clone(), k.clone());
      counter += v * bags;
    }
    counter
  }
}
