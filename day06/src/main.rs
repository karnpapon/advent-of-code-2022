use std::collections::HashSet;
use std::error::Error;
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
  let bytes = input.as_bytes();

  let (_, list) = bytes.windows(steps).enumerate().fold(
    (0, vec![]),
    |mut acc, curr| -> (usize, std::vec::Vec<i32>) {
      let mut set: HashSet<&u8> = HashSet::new();
      // let mut vec: Vec<&u8> = Vec::new();
      let mut sub_routine_idx = acc.0;
      let mut sub_routine_count = 0;

      while sub_routine_count < steps - 1 {
        bytes
          .iter()
          .skip(sub_routine_idx)
          .take(curr.1.len())
          .enumerate()
          .for_each(|(_idx, val)| {
            set.insert(val);
            sub_routine_idx += 1;
          });
        sub_routine_count += 1;
        sub_routine_idx = acc.0 + sub_routine_count;
        if set.len() == steps {
          acc.1.push((sub_routine_idx + steps - 1) as i32);
          sub_routine_count = steps + 2;
        }
        set.clear();
      }
      acc.0 += curr.1.len() - 1;
      acc
    },
  );

  let res = list.first().unwrap();

  writeln!(io::stdout(), "{}", res)?;
  Ok(())
}

fn solve_part2(input: &str) -> Result<()> {
  let steps = 14;
  let bytes = input.as_bytes();

  let (_, list) = bytes.windows(steps).enumerate().fold(
    (0, vec![]),
    |mut acc, curr| -> (usize, std::vec::Vec<i32>) {
      let mut set: HashSet<&u8> = HashSet::new();
      // let mut vec: Vec<&u8> = Vec::new();
      let mut sub_routine_idx = acc.0;
      let mut sub_routine_count = 0;

      while sub_routine_count < steps - 1 {
        bytes
          .iter()
          .skip(sub_routine_idx)
          .take(curr.1.len())
          .enumerate()
          .for_each(|(_idx, val)| {
            set.insert(val);
            sub_routine_idx += 1;
          });
        sub_routine_count += 1;
        sub_routine_idx = acc.0 + sub_routine_count;
        if set.len() == steps {
          acc.1.push((sub_routine_idx + steps - 1) as i32);
          sub_routine_count = steps + 2;
        }
        set.clear();
      }
      acc.0 += curr.1.len() - 1;
      acc
    },
  );

  let res = list.first().unwrap();

  writeln!(io::stdout(), "{}", res)?;
  Ok(())
}
