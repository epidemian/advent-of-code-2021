pub fn run() {
  let (enhancement_algo, input_image) = include_str!("inputs/day20").split_once("\n\n").unwrap();
  let enhancement_algo: Vec<_> = enhancement_algo.chars().map(|ch| ch == '#').collect();
  let input_image: Image = input_image
    .lines()
    .map(|l| l.chars().map(|ch| ch == '#').collect())
    .collect();

  let steps = 50;
  let mut image = extend(&input_image, steps);

  for n in 1..=steps {
    image = enhance(&image, &enhancement_algo);
    if n == 2 || n == 50 {
      println!("{}", count_lit_pixels(&image));
    }
  }
}

type Image = Vec<Vec<bool>>;

fn extend(image: &Image, steps: usize) -> Image {
  let size = image.len();
  let extended_size = size + (steps + 1) * 2;
  let mut extended_image = vec![vec![false; extended_size]; extended_size];

  for y in 0..size {
    for x in 0..size {
      extended_image[y + steps + 1][x + steps + 1] = image[y][x];
    }
  }
  extended_image
}

fn enhance(image: &Image, enhancement_algo: &Vec<bool>) -> Image {
  let size = image.len();
  let bit_at = |y: usize, x: usize| image[y][x] as usize;

  let enhanced_border_value = enhancement_algo[if image[0][0] { 511 } else { 0 }];
  let mut enhanced_image = vec![vec![enhanced_border_value; size]; size];

  for y in 1..size - 1 {
    for x in 1..size - 1 {
      let index = 0
        | bit_at(y - 1, x - 1) << 8
        | bit_at(y - 1, x) << 7
        | bit_at(y - 1, x + 1) << 6
        | bit_at(y, x - 1) << 5
        | bit_at(y, x) << 4
        | bit_at(y, x + 1) << 3
        | bit_at(y + 1, x - 1) << 2
        | bit_at(y + 1, x) << 1
        | bit_at(y + 1, x + 1);
      enhanced_image[y][x] = enhancement_algo[index];
    }
  }
  enhanced_image
}

fn count_lit_pixels(image: &Image) -> usize {
  image
    .iter()
    .map(|line| line.iter().filter(|px| **px).count())
    .sum()
}
