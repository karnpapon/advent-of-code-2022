use std::collections::HashSet;
use std::error::Error;
use std::hash::Hash;
use std::io::{self, Read, Write};
use std::str::from_utf8;

type Result<T> = ::std::result::Result<T, Box<dyn Error>>;

fn main() -> Result<()> {
  let mut input = String::new();
  io::stdin().read_to_string(&mut input)?;

  solve_part1(&input)?;
  solve_part2(&input)?;

  Ok(())
}

fn solve_part1(input: &str) -> Result<()> {
  let steps = 4;
  let chars = input.as_bytes();
  let signal = chars
    .windows(steps)
    .enumerate()
    .find(|(_, slice)| {
      let set = slice.iter().collect::<HashSet<&u8>>();
      slice.len() == set.len()
    })
    .unwrap();
  let res = signal.0 + steps;

  writeln!(io::stdout(), "{}", res)?;
  Ok(())
}

fn solve_part2(input: &str) -> Result<()> {
  let steps = 14;
  let chars = input.as_bytes();
  let signal = chars
    .windows(steps)
    .enumerate()
    .find(|(_, slice)| {
      let set = slice.iter().collect::<HashSet<&u8>>();
      slice.len() == set.len()
    })
    .unwrap();
  let res = signal.0 + steps;

  writeln!(io::stdout(), "{}", res)?;
  Ok(())
}
