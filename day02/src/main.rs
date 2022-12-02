use std::collections::HashMap;
use std::error::Error;
use std::io::{self, Read, Write};

type Result<T> = ::std::result::Result<T, Box<dyn Error>>;

#[derive(Clone, Copy)]
enum HandShape {
  Rock = 1,
  Paper = 2,
  Scissors = 3,
}

enum Rules {
  Lost = 0,
  Draw = 3,
  Won = 6,
}

fn main() -> Result<()> {
  let mut input = String::new();
  let table = HashMap::from([
    ("A", HandShape::Rock),
    ("B", HandShape::Paper),
    ("C", HandShape::Scissors),
    ("X", HandShape::Rock),
    ("Y", HandShape::Paper),
    ("Z", HandShape::Scissors),
  ]);

  let table_opponent = HashMap::from([
    ("A", HandShape::Rock),
    ("B", HandShape::Paper),
    ("C", HandShape::Scissors),
  ]);

  let table_your_strategy =
    HashMap::from([("X", Rules::Lost), ("Y", Rules::Draw), ("Z", Rules::Won)]);

  io::stdin().read_to_string(&mut input)?;
  solve_part1(&input, &table)?;
  solve_part2(&input, &table_opponent, &table_your_strategy)?;
  Ok(())
}

fn solve_part1(input: &str, table: &HashMap<&str, HandShape>) -> Result<()> {
  let res = input.lines().fold(0, |acc, line| {
    acc
      + [line.split(' ').collect::<Vec<&str>>()]
        .iter()
        .fold(0, |acc2, curr| {
          let other_turn = table.get(curr[0]).unwrap();
          let your_turn = table.get(curr[1]).unwrap();

          let result = match (other_turn, your_turn) {
            (HandShape::Rock, HandShape::Paper) => Rules::Won,
            (HandShape::Rock, HandShape::Scissors) => Rules::Lost,

            (HandShape::Paper, HandShape::Scissors) => Rules::Won,
            (HandShape::Paper, HandShape::Rock) => Rules::Lost,

            (HandShape::Scissors, HandShape::Rock) => Rules::Won,
            (HandShape::Scissors, HandShape::Paper) => Rules::Lost,

            (_, _) => Rules::Draw,
          };

          acc2 + (result as i32) + *your_turn as i32
        })
  });
  writeln!(io::stdout(), "{}", res)?;
  Ok(())
}

fn solve_part2(
  input: &str,
  table_opponent: &HashMap<&str, HandShape>,
  table_your_strategy: &HashMap<&str, Rules>,
) -> Result<()> {
  let res = input.lines().fold(0, |acc, line| {
    acc
      + [line.split(' ').collect::<Vec<&str>>()]
        .iter()
        .fold(0, |acc2, curr| {
          let other_turn = table_opponent.get(curr[0]).unwrap();
          let your_turn = table_your_strategy.get(curr[1]).unwrap();

          let result = match (other_turn, your_turn) {
            (HandShape::Rock, Rules::Lost) => HandShape::Scissors,
            (HandShape::Rock, Rules::Won) => HandShape::Paper,

            (HandShape::Paper, Rules::Lost) => HandShape::Rock,
            (HandShape::Paper, Rules::Won) => HandShape::Scissors,

            (HandShape::Scissors, Rules::Lost) => HandShape::Paper,
            (HandShape::Scissors, Rules::Won) => HandShape::Rock,

            (_, _) => *other_turn,
          };

          acc2 + (result as i32) + *your_turn as i32
        })
  });
  writeln!(io::stdout(), "{}", res)?;
  Ok(())
}
