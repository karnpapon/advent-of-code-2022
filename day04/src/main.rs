use std::error::Error;
use std::io::{self, Read, Write};

type Result<T> = ::std::result::Result<T, Box<dyn Error>>;

fn main() -> Result<()> {
  let mut input = String::new();
  io::stdin().read_to_string(&mut input)?;

  solve_part1(&input)?;
  solve_part2(&input)?;
  Ok(())
}

fn solve_part1(input: &str) -> Result<()> {
  let res = input.lines().fold(0, |mut acc, line| {
    let first = 0;
    let last = 1;
    let pairs = line
      .split(',')
      .collect::<Vec<&str>>()
      .iter()
      .map(|g| {
        let group_range_num = g
          .split('-')
          .collect::<Vec<&str>>()
          .iter()
          .map(|x| x.parse::<i32>().unwrap())
          .collect::<Vec<i32>>();
        group_range_num[0]..=group_range_num[1]
      })
      .collect::<Vec<_>>();

    if pairs[first].contains(pairs[last].start()) && pairs[first].contains(pairs[last].end())
      || pairs[last].contains(pairs[first].start()) && pairs[last].contains(pairs[first].end())
    {
      acc += 1
    }

    acc
  });

  writeln!(io::stdout(), "{}", res)?;
  Ok(())
}

fn solve_part2(input: &str) -> Result<()> {
  let res = input.lines().fold(0, |mut acc, line| {
    let first = 0;
    let last = 1;
    let pairs = line
      .split(',')
      .collect::<Vec<&str>>()
      .iter()
      .map(|g| {
        let group_range_num = g
          .split('-')
          .collect::<Vec<&str>>()
          .iter()
          .map(|x| x.parse::<i32>().unwrap())
          .collect::<Vec<i32>>();
        group_range_num[0]..=group_range_num[1]
      })
      .collect::<Vec<_>>();

    if pairs[first].contains(pairs[last].start())
      || pairs[first].contains(pairs[last].end())
      || pairs[last].contains(pairs[first].start())
      || pairs[last].contains(pairs[first].end())
    {
      acc += 1
    }

    acc
  });

  writeln!(io::stdout(), "{}", res)?;
  Ok(())
}
