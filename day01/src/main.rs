use std::cmp::{max, min};
use std::error::Error;
use std::io::{self, Read, Write};

type Result<T> = ::std::result::Result<T, Box<dyn Error>>;

fn main() -> Result<()> {
  let mut input = String::new();
  io::stdin().read_to_string(&mut input)?;

  // append EOF to `input` so we can include the last `total`.
  // otherwise `line.parse()` will stop before the last chunk.
  // since `total` will be calculated only after encounter `\n`.
  input.push_str("\n\n");

  solve_part1(&input)?;
  solve_part2(&input)?;
  Ok(())
}

fn solve_part1(input: &str) -> Result<()> {
  let mut total = 0;
  let mut max_val = 0;

  for line in input.lines() {
    match line.parse::<i32>() {
      Ok(cal) => total += cal,
      Err(_) => {
        max_val = max(total, max_val);
        total = 0
      }
    }
  }
  writeln!(io::stdout(), "{}", max_val)?;
  Ok(())
}

fn solve_part2(input: &str) -> Result<()> {
  let mut total = 0;
  let mut max_val = [0i32; 3];

  for line in input.lines() {
    match line.parse::<i32>() {
      Ok(cal) => total += cal,
      Err(_) => {
        max_val = [
          max(total, max_val[0]),
          min(max(total, max_val[1]), max_val[0]),
          min(max(total, max_val[2]), max_val[1]),
        ];
        total = 0
      }
    }
  }

  let sum: i32 = max_val.iter().sum();

  writeln!(io::stdout(), "{:?}", sum)?;
  Ok(())
}
