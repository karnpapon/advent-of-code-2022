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
  fn pop_front(&mut self) {
    self.start_items.pop_front().unwrap();
  }
  fn inspect(&mut self, disable_worry_level_divider: bool) -> u64 {
    let worry_level = match &self.operation {
      Operation::Add((v1, v2)) => match (v1, v2) {
        (Value::Old, Value::Num(num)) => self.start_items[0] + num,
        (Value::Old, Value::Old) => self.start_items[0] + self.start_items[0],
        _ => 0,
      },
      Operation::Multiply((v1, v2)) => match (v1, v2) {
        (Value::Old, Value::Num(num)) => self.start_items[0] * num,
        (Value::Old, Value::Old) => self.start_items[0] * self.start_items[0],
        _ => 0,
      },
    };
    let res = if !disable_worry_level_divider {
      worry_level / 3
    } else {
      worry_level
    };
    res
  }
  fn test(&mut self, level: u64) -> u64 {
    match level % self.test.divisible_by == 0 {
      true => self.test.next_monkey_true.try_into().unwrap(),
      false => self.test.next_monkey_false.try_into().unwrap(),
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

// What is the level of monkey business after 20 rounds of stuff-slinging simian shenanigans?
fn solve_part1(input: &str) -> Result<()> {
  let (_, mut monkeys) = separated_list1(tag("\n\n"), parse_monkey)(input).unwrap();
  let rounds = 20;

  (0..(monkeys.len() * rounds)).for_each(|idx| {
    let i = idx % monkeys.len();
    (0..monkeys[i].start_items.len()).for_each(|_| {
      let monkey = &mut monkeys[i];
      let worry_level = monkey.inspect(false);
      let next_monkey = monkey.test(worry_level);
      monkey.pop_front();
      monkey.inspected_counter += 1;
      monkeys[next_monkey as usize]
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

// what is the level of monkey business after 10000 rounds?
fn solve_part2(input: &str) -> Result<()> {
  let (_, mut monkeys) = separated_list1(tag("\n\n"), parse_monkey)(input).unwrap();
  let rounds = 20;

  //   99 // 97 // 8 // 103
  (0..(monkeys.len() * rounds)).for_each(|idx| {
    let i = idx % monkeys.len();
    (0..monkeys[i].start_items.len()).for_each(|_| {
      let monkey = &mut monkeys[i];
      let worry_level = monkey.inspect(true);
      let next_monkey = monkey.test(worry_level);
      monkey.inspected_counter += 1;
      monkey.pop_front();
      monkeys[next_monkey as usize]
        .start_items
        .push_back(worry_level);
    });
  });

  let mut total_inspected_times_list = monkeys
    .iter()
    .map(|m| m.inspected_counter) // `i32` is not enough for this puzzle.
    .collect::<Vec<u64>>();
  total_inspected_times_list.sort_by(|a, b| b.partial_cmp(a).unwrap());

  let res = total_inspected_times_list.iter().take(2).product::<u64>();

  monkeys
    .iter()
    .for_each(|m| println!("{:?}", m.inspected_counter));

  writeln!(io::stdout(), "{:?}", res)?;

  Ok(())
}
