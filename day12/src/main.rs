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

// What is the fewest steps required to move from your current position to the location that should get the best signal?
fn solve_part1(input: &str) -> Result<()> {
  let res = 0;

  // test: 31
  writeln!(io::stdout(), "{:?}", res)?;
  Ok(())
}

fn solve_part2(input: &str) -> Result<()> {
  let res = 0;
  writeln!(io::stdout(), "{:?}", res)?;
  Ok(())
}
