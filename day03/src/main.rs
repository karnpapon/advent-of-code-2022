use std::error::Error;
use std::io::{self, Read, Write};
use std::ops::{Add, Div, Mul, Sub};

type Result<T> = ::std::result::Result<T, Box<dyn Error>>;

fn main() -> Result<()> {
  let mut input = String::new();
  io::stdin().read_to_string(&mut input)?;

  solve_part1(&input)?;
  solve_part2(&input)?;

  Ok(())
}

fn solve_part1(input: &str) -> Result<()> {
  let res: i32 = input
    .lines()
    .fold(vec![], |mut acc, line| {
      let (first, last) = line.split_at(line.len() / 2);
      let m = first
        .as_bytes()
        .iter()
        .reduce(|mut f_acc, curr| {
          if let Some(val) = last.as_bytes().iter().find(|x| *x == curr) {
            f_acc = val;
          }
          f_acc
        })
        .unwrap();
      acc.push(byte_to_priority(*m as i32));
      acc
    })
    .iter()
    .sum();

  writeln!(io::stdout(), "solve_part1 = {:?}", res)?;
  Ok(())
}

fn solve_part2(input: &str) -> Result<()> {
  let steps = 3;
  let first_line = 0;
  let second_line = 1;
  let third_line = 2;
  let res: i32 = input
    .lines()
    .step_by(steps)
    .enumerate()
    .fold(vec![], |mut acc, line| {
      let offset = line.0 * steps;
      let chunks = input
        .lines()
        .skip(offset)
        .take(steps)
        .collect::<Vec<&str>>();
      let elf_badge = chunks[first_line]
        .chars()
        .fold(None, |mut chunk_accu, chunk_curr| {
          if chunks[second_line].contains(chunk_curr) && (chunks[third_line].contains(chunk_curr)) {
            chunk_accu = Some(chunk_curr)
          }
          chunk_accu
        })
        .unwrap();

      acc.push(elf_badge);
      acc
    })
    .iter()
    .map(|x| byte_to_priority(*x as i32))
    .collect::<Vec<i32>>()
    .iter()
    .sum();

  writeln!(io::stdout(), "solve_part2 = {:?}", res)?;
  Ok(())
}

fn byte_to_priority(byte: i32) -> i32 {
  let mut res = 0;

  // Lowercase item types a through z have priorities 1 through 26.
  // Uppercase item types A through Z have priorities 27 through 52.

  // a-z
  if (65..=90).contains(&byte) {
    res = map_range((65, 90), (27, 52), byte);
  }

  // A-Z
  if (97..=122).contains(&byte) {
    res = map_range((97, 122), (1, 26), byte);
  };

  res
}

fn map_range<T: Copy>(from_range: (T, T), to_range: (T, T), s: T) -> T
where
  T: Add<T, Output = T> + Sub<T, Output = T> + Mul<T, Output = T> + Div<T, Output = T>,
{
  to_range.0 + (s - from_range.0) * (to_range.1 - to_range.0) / (from_range.1 - from_range.0)
}
