use std::collections::HashMap;

pub fn run() {
  let (template_str, rules_str) = include_str!("inputs/day14").split_once("\n\n").unwrap();
  let polymer_template: Vec<_> = template_str.bytes().collect();
  let insertion_rules: HashMap<(u8, u8), u8> = rules_str
    .lines()
    .map(|l| {
      let bytes = l.as_bytes();
      ((bytes[0], bytes[1]), bytes[bytes.len() - 1])
    })
    .collect();
  let last_element = *polymer_template.last().unwrap();

  let mut pair_counts: HashMap<(u8, u8), usize> = HashMap::new();
  for pair in polymer_template.windows(2) {
    *pair_counts.entry((pair[0], pair[1])).or_insert(0) += 1;
  }

  for n in 1..=40 {
    let mut new_counts = HashMap::new();
    for (&(l, r), &count) in pair_counts.iter() {
      let middle = insertion_rules[&(l, r)];
      *new_counts.entry((l, middle)).or_insert(0) += count;
      *new_counts.entry((middle, r)).or_insert(0) += count;
    }
    pair_counts = new_counts;

    if n == 10 || n == 40 {
      let mut element_counts: HashMap<u8, usize> = HashMap::new();
      for (&(l, _), &count) in pair_counts.iter() {
        *element_counts.entry(l).or_insert(0) += count
      }
      *element_counts.entry(last_element).or_insert(0) += 1;

      let max_elem_count = element_counts.values().max().unwrap();
      let min_elem_count = element_counts.values().min().unwrap();
      println!("{}", max_elem_count - min_elem_count);
    }
  }
}
