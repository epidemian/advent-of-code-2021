use std::collections::HashMap;

pub fn run() {
  let mut cave_graph: Graph = HashMap::new();
  for line in include_str!("inputs/day12").lines() {
    let (cave_1, cave_2) = line.split_once("-").unwrap();
    cave_graph.entry(cave_1).or_insert(vec![]).push(cave_2);
    cave_graph.entry(cave_2).or_insert(vec![]).push(cave_1);
  }

  println!("{}", count_paths(&cave_graph, "start", "end", &vec!()));
  println!("{}", count_paths_2(&cave_graph, "start", "end", &vec!()));
}

type Graph = HashMap<&'static str, Vec<&'static str>>;

fn count_paths(cave_graph: &Graph, from: &str, to: &str, visited_caves: &Vec<&str>) -> i32 {
  if from == to {
    return 1;
  }
  let mut visited_caves = visited_caves.clone();
  visited_caves.push(from);
  cave_graph[from]
    .iter()
    .filter(|cave| is_big(cave) || !visited_caves.contains(cave))
    .map(|cave| count_paths(cave_graph, cave, to, &visited_caves))
    .sum()
}

fn is_big(cave: &str) -> bool {
  cave.chars().next().unwrap().is_ascii_uppercase()
}

fn count_paths_2(cave_graph: &Graph, from: &str, to: &str, visited_caves: &Vec<&str>) -> i32 {
  if from == to {
    return 1;
  }
  let mut visited_caves = visited_caves.clone();
  visited_caves.push(from);
  cave_graph[from]
    .iter()
    .filter(|&cave| {
      is_big(cave)
        || !visited_caves.contains(cave)
        || (*cave != "start" && !small_cave_visited_twice(&visited_caves))
    })
    .map(|cave| count_paths_2(cave_graph, cave, to, &visited_caves))
    .sum()
}

fn small_cave_visited_twice(visited_caves: &Vec<&str>) -> bool {
  visited_caves
    .iter()
    .filter(|cave| !is_big(cave))
    .any(|cave| visited_caves.iter().filter(|&c| c == cave).count() >= 2)
}
