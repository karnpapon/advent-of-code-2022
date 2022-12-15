use nom::{
  branch::alt,
  bytes::complete::tag,
  character::complete::multispace1,
  multi::separated_list1,
  sequence::{delimited, preceded},
  *,
};
use std::collections::VecDeque;
use std::error::Error;
use std::io::{self, Read, Write};

type Result<T> = ::std::result::Result<T, Box<dyn Error>>;

#[derive(Debug)]
enum Value {
  Old,
  Num(u64),
}

#[derive(Debug)]
enum Operation {
  Multiply((Value, Value)),
  Add((Value, Value)),
}

#[derive(Debug)]
struct Test {
  divisible_by: u64,
  next_monkey_true: u64,
  next_monkey_false: u64,
}

#[derive(Debug)]
struct Monkey {
  id: u64,
  start_items: VecDeque<u64>,
  operation: Operation,
  test: Test,
  inspected_counter: u64,
}

impl Monkey {
  fn inspect(&mut self, disable_worry_level_divider: bool, factor: u64) -> u64 {
    let start_item = self.start_items.pop_front().unwrap();
    let worry_level = match &self.operation {
      Operation::Add((v1, v2)) => match (v1, v2) {
        (Value::Old, Value::Num(num)) | (Value::Num(num), Value::Old) => start_item + num,
        (Value::Old, Value::Old) => start_item + start_item,
        _ => 0, // actually it should cover all possible cases, but for the sake of AoC i'll leave it anyway.
      },
      Operation::Multiply((v1, v2)) => match (v1, v2) {
        (Value::Old, Value::Num(num)) | (Value::Num(num), Value::Old) => start_item * num,
        (Value::Old, Value::Old) => start_item * start_item,
        _ => 0, // same as above
      },
    };

    match disable_worry_level_divider {
      true => worry_level % factor,
      false => (worry_level % factor) / 3,
    }
  }

  fn test(&mut self, level: u64) -> u64 {
    match level % self.test.divisible_by == 0 {
      true => self.test.next_monkey_true,
      false => self.test.next_monkey_false,
    }
  }
}

// ----------- parser -------------------

fn value(input: &str) -> IResult<&str, Value> {
  alt((
    tag("old").map(|_| Value::Old),
    nom::character::complete::u64.map(Value::Num),
  ))(input)
}
fn operation(input: &str) -> IResult<&str, Operation> {
  let (input, _) = tag("Operation: new = ")(input)?;
  let (input, value_1) = value(input)?;
  let (input, operator) = delimited(multispace1, alt((tag("*"), tag("+"))), multispace1)(input)?;
  let (input, value_2) = value(input)?;

  let result = match operator {
    "*" => Operation::Multiply((value_1, value_2)),
    "+" => Operation::Add((value_1, value_2)),
    _ => panic!("unsupported operator"),
  };
  Ok((input, result))
}
fn test(input: &str) -> IResult<&str, Test> {
  let (input, divisible_by) =
    preceded(tag("Test: divisible by "), nom::character::complete::u64)(input)?;
  let (input, _) = multispace1(input)?;
  let (input, next_monkey_true) = preceded(
    tag("If true: throw to monkey "),
    nom::character::complete::u64,
  )(input)?;
  let (input, _) = multispace1(input)?;
  let (input, next_monkey_false) = preceded(
    tag("If false: throw to monkey "),
    nom::character::complete::u64,
  )(input)?;
  Ok((
    input,
    Test {
      divisible_by,
      next_monkey_true,
      next_monkey_false,
    },
  ))
}
fn parse_monkey(input: &str) -> IResult<&str, Monkey> {
  let (input, id) = delimited(tag("Monkey "), nom::character::complete::u64, tag(":"))(input)?;
  let (input, _) = multispace1(input)?;
  let (input, items) = preceded(
    tag("Starting items: "),
    separated_list1(tag(", "), nom::character::complete::u64),
  )(input)?;
  let (input, _) = multispace1(input)?;
  let (input, op) = operation(input)?;
  let (input, _) = multispace1(input)?;
  let (input, test) = test(input)?;

  Ok((
    input,
    Monkey {
      id,
      start_items: VecDeque::from(items),
      operation: op,
      test,
      inspected_counter: 0,
    },
  ))
}

fn main() -> Result<()> {
  let mut input = String::new();
  io::stdin().read_to_string(&mut input)?;
  solve_part1(&input)?;
  solve_part2(&input)?;
  Ok(())
}

// Q: What is the level of monkey business after 20 rounds of stuff-slinging simian shenanigans?
fn solve_part1(input: &str) -> Result<()> {
  let (_, mut monkeys) = separated_list1(tag("\n\n"), parse_monkey)(input).unwrap();
  let rounds = 20;

  // actually in puzzle part-1, this `factor` is not neccessarily needs.
  // in the otherwords, part-1 still be able to calculate with or without `factor`.
  let factor = monkeys
    .iter()
    .map(|monkey| monkey.test.divisible_by)
    .product::<u64>();

  (0..(monkeys.len() * rounds)).for_each(|idx| {
    let i = idx % monkeys.len();
    (0..monkeys[i].start_items.len()).for_each(|_| {
      let monkey = &mut monkeys[i];
      let worry_level = monkey.inspect(false, factor);
      let next_monkey = monkey.test(worry_level);
      monkey.inspected_counter += 1;
      monkeys
        .iter_mut()
        .find(|m| m.id == next_monkey)
        .unwrap()
        .start_items
        .push_back(worry_level);
    });
  });

  let mut total_inspected_times_list = monkeys
    .iter()
    .map(|m| m.inspected_counter)
    .collect::<Vec<u64>>();
  total_inspected_times_list.sort_by(|a, b| b.partial_cmp(a).unwrap());

  let res = total_inspected_times_list.iter().take(2).product::<u64>();
  writeln!(io::stdout(), "{:?}", res)?;

  Ok(())
}

// Q: what is the level of monkey business after 10000 rounds?
fn solve_part2(input: &str) -> Result<()> {
  let (_, mut monkeys) = separated_list1(tag("\n\n"), parse_monkey)(input).unwrap();
  let rounds = 10_000;

  // [FROM THIS LINE in AdvenOfCode2022] (https://adventofcode.com/2022/day/11)
  // >> Unfortunately, that relief was all that was keeping your worry levels from reaching ridiculous levels.
  // You'll need to find another way to keep your worry levels manageable.<<
  // basically this `factor` will keep `worry level` within sane levels.
  let factor = monkeys
    .iter()
    .map(|monkey| monkey.test.divisible_by)
    .product::<u64>();

  (0..(monkeys.len() * rounds)).for_each(|idx| {
    let i = idx % monkeys.len();
    (0..monkeys[i].start_items.len()).for_each(|_| {
      let monkey = &mut monkeys[i];
      let worry_level = monkey.inspect(true, factor);
      let next_monkey = monkey.test(worry_level);
      monkey.inspected_counter += 1;
      monkeys
        .iter_mut()
        .find(|m| m.id == next_monkey)
        .unwrap()
        .start_items
        .push_back(worry_level);
    });
  });

  let mut total_inspected_times_list = monkeys
    .iter()
    .map(|m| m.inspected_counter)
    .collect::<Vec<u64>>(); // `i32` is not enough for this puzzle.
  total_inspected_times_list.sort_by(|a, b| b.partial_cmp(a).unwrap());

  let res = total_inspected_times_list.iter().take(2).product::<u64>();

  writeln!(io::stdout(), "{:?}", res)?;

  Ok(())
}
