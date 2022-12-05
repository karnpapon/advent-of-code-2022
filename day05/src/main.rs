use nom::{branch::permutation, bytes::complete::tag, character::complete::digit1, IResult};
use std::error::Error;
use std::io::{self, Read, Write};

type Result<T> = ::std::result::Result<T, Box<dyn Error>>;

#[derive(Debug)]
struct Command {
  move_amount: i32,
  from: usize,
  to: usize,
}

fn main() -> Result<()> {
  let mut input = String::new();
  io::stdin().read_to_string(&mut input)?;

  let input_crates = vec![
    vec!['N', 'S', 'D', 'C', 'V', 'Q', 'T'],
    vec!['M', 'F', 'V'],
    vec!['F', 'Q', 'W', 'D', 'P', 'N', 'H', 'M'],
    vec!['D', 'Q', 'R', 'T', 'F'],
    vec!['R', 'F', 'M', 'N', 'Q', 'H', 'V', 'B'],
    vec!['C', 'F', 'G', 'N', 'P', 'W', 'Q'],
    vec!['W', 'F', 'R', 'L', 'C', 'T'],
    vec!['T', 'Z', 'N', 'S'],
    vec!['M', 'S', 'D', 'J', 'R', 'Q', 'H', 'T'],
  ];

  let mut test_crates = vec![vec!['Z', 'N'], vec!['M', 'C', 'D'], vec!['P']];

  solve_part1(&input, &mut input_crates.clone())?;
  solve_part2(&input, &mut input_crates.clone())?;
  Ok(())
}

// test creates.
//     [D]
// [N] [C]
// [Z] [M] [P]
//  1   2   3

// input crates.
//         [M]     [B]             [N]
// [T]     [H]     [V] [Q]         [H]
// [Q]     [N]     [H] [W] [T]     [Q]
// [V]     [P] [F] [Q] [P] [C]     [R]
// [C]     [D] [T] [N] [N] [L] [S] [J]
// [D] [V] [W] [R] [M] [G] [R] [N] [D]
// [S] [F] [Q] [Q] [F] [F] [F] [Z] [S]
// [N] [M] [F] [D] [R] [C] [W] [T] [M]
//  1   2   3   4   5   6   7   8   9

fn solve_part1(input: &str, crates: &mut [Vec<char>]) -> Result<()> {
  input.lines().for_each(|curr| {
    let (_, cmd) = parse_command(curr).unwrap();

    for _ in 0..cmd.move_amount {
      let moved_item = crates[cmd.from].pop().unwrap();
      crates[cmd.to].push(moved_item);
    }
  });

  let res = get_result(crates);

  writeln!(io::stdout(), "{}", res)?;
  Ok(())
}

fn solve_part2(input: &str, crates: &mut [Vec<char>]) -> Result<()> {
  input.lines().for_each(|curr| {
    let (_, cmd) = parse_command(curr).unwrap();

    let drain_end = (crates[cmd.from].len()) as i32;
    let drain_from = (drain_end - cmd.move_amount) as usize;

    let mut moved_items: Vec<char> = crates[cmd.from].drain(drain_from..).collect();
    crates[cmd.to].append(&mut moved_items);
  });

  let res = get_result(crates);

  writeln!(io::stdout(), "{}", res)?;
  Ok(())
}

// ---------------------- UTILS -------------------------

fn parse_command(input: &str) -> IResult<&str, Command> {
  // I know, but please forgive my unintuitively use of `nom`.
  let (input, res) = permutation((
    tag("move "),
    digit1,
    tag(" from "),
    digit1,
    tag(" to "),
    digit1,
  ))(input)?;

  Ok((
    input,
    Command {
      move_amount: res.1.parse().unwrap(),
      from: res.3.parse::<usize>().unwrap() - 1, // normalized for `vec! `indexing.
      to: res.5.parse::<usize>().unwrap() - 1,
    },
  ))
}

fn get_result(crates: &mut [Vec<char>]) -> String {
  let res = crates.iter_mut().fold(String::new(), |mut acc, curr| {
    let new_char = curr.pop().unwrap();
    acc.push(new_char);
    acc
  });

  res
}

// ---------------------------------------------------------
