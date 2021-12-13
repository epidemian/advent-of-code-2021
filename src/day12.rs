use std::collections::HashMap;

pub fn run() {
  let mut cave_graph: Graph = HashMap::new();
  for line in include_str!("inputs/day12").lines() {
    let (cave_1, cave_2) = line.split_once("-").unwrap();
    cave_graph.entry(cave_1).or_insert(vec![]).push(cave_2);
    cave_graph.entry(cave_2).or_insert(vec![]).push(cave_1);
  }

  println!("{}", part_1(&cave_graph));
  println!("{}", part_2(&cave_graph));
}

type Graph = HashMap<&'static str, Vec<&'static str>>;

fn part_1(cave_graph: &Graph) -> i32 {
  // Can visit small caves at most once.
  let can_visit =
    |cave: &str, visited_caves: &Vec<&str>| is_big(cave) || !visited_caves.contains(&cave);
  count_paths(&cave_graph, "start", &vec![], can_visit)
}

fn part_2(cave_graph: &Graph) -> i32 {
  // Can visit a single small cave twice.
  let can_visit = |cave: &str, visited_caves: &Vec<&str>| {
    is_big(cave)
      || !visited_caves.contains(&cave)
      || (cave != "start" && !small_cave_visited_twice(&visited_caves))
  };
  count_paths(&cave_graph, "start", &vec![], can_visit)
}

fn count_paths(
  cave_graph: &Graph,
  from: &str,
  visited_caves: &Vec<&str>,
  can_visit: fn(&str, &Vec<&str>) -> bool,
) -> i32 {
  if from == "end" {
    return 1;
  }
  let mut visited_caves = visited_caves.clone();
  visited_caves.push(from);
  cave_graph[from]
    .iter()
    .filter(|cave| can_visit(cave, &visited_caves))
    .map(|cave| count_paths(cave_graph, cave, &visited_caves, can_visit))
    .sum()
}

fn is_big(cave: &str) -> bool {
  cave.chars().next().unwrap().is_ascii_uppercase()
}

fn small_cave_visited_twice(visited_caves: &Vec<&str>) -> bool {
  visited_caves
    .iter()
    .filter(|cave| !is_big(cave))
    .any(|cave| visited_caves.iter().filter(|&c| c == cave).count() >= 2)
}
