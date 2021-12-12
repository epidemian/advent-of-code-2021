pub fn run() {
  let entries: Vec<Entry> = include_str!("inputs/day08")
    .lines()
    .map(parse_entry)
    .collect();

  let easy_digits_count: usize = entries.iter().map(count_easy_digits).sum();
  println!("{}", easy_digits_count);

  let output_sum: usize = entries.iter().map(decode_output).sum();
  println!("{}", output_sum)
}

struct Entry<'a> {
  signals: Vec<&'a str>,
  output: Vec<&'a str>,
}

fn parse_entry(s: &str) -> Entry {
  let (signals, output) = s.split_once(" | ").unwrap();
  Entry {
    signals: signals.split(" ").collect(),
    output: output.split(" ").collect(),
  }
}

fn count_easy_digits(entry: &Entry) -> usize {
  entry
    .output
    .iter()
    .filter(|digit| [2, 3, 4, 7].contains(&digit.len()))
    .count()
}

fn decode_output(Entry { signals, output }: &Entry) -> usize {
  let one_signal = *signals.iter().find(|s| s.len() == 2).unwrap();
  let four_signal = *signals.iter().find(|s| s.len() == 4).unwrap();
  let output_digits: Vec<usize> = output
    .iter()
    .map(|&s| {
      let overlap_with_one = s.chars().filter(|ch| one_signal.contains(*ch)).count();
      let overlap_with_four = s.chars().filter(|ch| four_signal.contains(*ch)).count();
      match s.len() {
        2 => 1,
        3 => 7,
        4 => 4,
        5 => {
          // 2, 3 and 5 have 5 segments
          if overlap_with_one == 2 {
            3
          } else if overlap_with_four == 3 {
            5
          } else {
            2
          }
        }
        6 => {
          // 0, 6 and 9 have 6 segments
          if overlap_with_four == 4 {
            9
          } else if overlap_with_one == 2 {
            0
          } else {
            6
          }
        }
        7 => 8,
        _ => unreachable!(),
      }
    })
    .collect();
  output_digits.iter().fold(0, |a, b| a * 10 + b)
}
