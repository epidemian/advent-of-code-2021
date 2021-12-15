use std::collections::{HashMap, HashSet};

pub fn run() {
  let (polymer_part, rules_part) = include_str!("inputs/day14").split_once("\n\n").unwrap();
  let mut polymer: Vec<_> = polymer_part.bytes().collect();
  let insertion_rules: HashMap<(u8, u8), u8> = rules_part
    .lines()
    .map(|l| {
      let bytes = l.as_bytes();
      ((bytes[0], bytes[1]), bytes[bytes.len() - 1])
    })
    .collect();

  for _ in 0..10 {
    let mut new_polymer = Vec::with_capacity(polymer.len() * 2 - 1);
    for pair in polymer.windows(2) {
      new_polymer.push(pair[0]);
      new_polymer.push(insertion_rules[&(pair[0], pair[1])]);
    }
    new_polymer.push(polymer[polymer.len() - 1]);
    polymer = new_polymer;
  }

  let elements: HashSet<_> = insertion_rules.values().collect();
  let mut counts: Vec<usize> = elements
    .iter()
    .map(|elem| polymer.iter().filter(|e| e == elem).count())
    .collect();
  counts.sort();
  println!("{}", counts[counts.len() - 1] - counts[0]);
}
