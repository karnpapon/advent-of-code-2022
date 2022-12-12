use std::collections::BTreeSet;
use std::error::Error;
use std::io::{self, Read, Write};
use std::iter;

type Result<T> = ::std::result::Result<T, Box<dyn Error>>;

#[derive(Debug, Clone)]
enum Direction {
  Left(i32),
  Right(i32),
  Up(i32),
  Down(i32),
  Unsupported,
}

#[derive(Debug, Clone)]
struct Rope {
  head: Position,
  tail: Position,
  starting_point: Position,
  visited: BTreeSet<String>,
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct Position {
  x: i32,
  y: i32,
}

impl Rope {
  fn new() -> Self {
    Rope {
      head: Position { x: 0, y: 0 },
      tail: Position { x: 0, y: 0 },
      starting_point: Position { x: 0, y: 0 },
      visited: BTreeSet::new(),
    }
  }
}

fn parse(cmd: &str) -> Direction {
  let c = cmd.split(' ').collect::<Vec<_>>();
  let move_steps = c[1].parse::<i32>().unwrap();

  match c[0] {
    "R" => Direction::Right(move_steps),
    "L" => Direction::Left(move_steps),
    "U" => Direction::Up(move_steps),
    "D" => Direction::Down(move_steps),
    _ => Direction::Unsupported,
  }
}

fn main() -> Result<()> {
  let mut input = String::new();
  io::stdin().read_to_string(&mut input)?;
  solve_part1(&input)?;
  solve_part2(&input)?;
  Ok(())
}

// How many positions does the tail of the rope visit at least once?
fn solve_part1(input: &str) -> Result<()> {
  let mut res = 0;
  let mut rope_state = Rope::new();
  rope_state.visited.insert(format!(
    "[{},{}]",
    rope_state.starting_point.x, rope_state.starting_point.y
  ));

  input.lines().for_each(|line| {
    match parse(line) {
      Direction::Up(val) => iter::repeat(1)
        .take(val.try_into().unwrap())
        .for_each(|val| {
          rope_state.head.y -= val;
          move_tail(
            &mut rope_state.tail,
            &mut rope_state.head,
            &mut rope_state.visited,
          );
        }),
      Direction::Down(val) => iter::repeat(1)
        .take(val.try_into().unwrap())
        .for_each(|val| {
          rope_state.head.y += val;
          move_tail(
            &mut rope_state.tail,
            &mut rope_state.head,
            &mut rope_state.visited,
          );
        }),
      Direction::Right(val) => iter::repeat(1)
        .take(val.try_into().unwrap())
        .for_each(|val| {
          rope_state.head.x += val;
          move_tail(
            &mut rope_state.tail,
            &mut rope_state.head,
            &mut rope_state.visited,
          );
        }),
      Direction::Left(val) => iter::repeat(1)
        .take(val.try_into().unwrap())
        .for_each(|val| {
          rope_state.head.x -= val;
          move_tail(
            &mut rope_state.tail,
            &mut rope_state.head,
            &mut rope_state.visited,
          );
        }),
      Direction::Unsupported => {}
    };
  });

  res = rope_state.visited.len();
  writeln!(io::stdout(), "{:?}", res)?;
  Ok(())
}

// How many positions does the tail of the rope (10 knots 0-9) visit at least once?
fn solve_part2(input: &str) -> Result<()> {
  let mut res = 0;
  let mut rope_state_h_1 = Rope::new();
  let mut rope_state_2_3 = Rope::new();
  let mut rope_state_4_5 = Rope::new();
  let mut rope_state_6_7 = Rope::new();
  let mut rope_state_8_9 = Rope::new();

  let mut rope_vec = [
    &mut rope_state_h_1,
    &mut rope_state_2_3,
    &mut rope_state_4_5,
    &mut rope_state_6_7,
    &mut rope_state_8_9,
  ];

  rope_vec.iter_mut().for_each(|rope_state| {
    rope_state.visited.insert(format!(
      "[{},{}]",
      rope_state.starting_point.x, rope_state.starting_point.y
    ));
  });

  input.lines().for_each(|line| {
    match parse(line) {
      Direction::Up(val) => iter::repeat(1)
        .take(val.try_into().unwrap())
        .for_each(|val| {
          // move first  (H,1)
          rope_vec[0].head.y -= val;
          move_tail(
            &mut rope_vec[0].tail,
            &mut rope_vec[0].head,
            &mut rope_vec[0].visited,
          );

          // move others knots sequencially  (2,3) ~ (8,9)
          move_knots(&mut rope_vec);
        }),
      Direction::Down(val) => iter::repeat(1)
        .take(val.try_into().unwrap())
        .for_each(|val| {
          rope_vec[0].head.y += val;
          move_tail(
            &mut rope_vec[0].tail,
            &mut rope_vec[0].head,
            &mut rope_vec[0].visited,
          );
          move_knots(&mut rope_vec);
        }),
      Direction::Right(val) => iter::repeat(1)
        .take(val.try_into().unwrap())
        .for_each(|val| {
          rope_vec[0].head.x += val;
          move_tail(
            &mut rope_vec[0].tail,
            &mut rope_vec[0].head,
            &mut rope_vec[0].visited,
          );
          move_knots(&mut rope_vec);
        }),
      Direction::Left(val) => iter::repeat(1)
        .take(val.try_into().unwrap())
        .for_each(|val| {
          rope_vec[0].head.x -= val;
          move_tail(
            &mut rope_vec[0].tail,
            &mut rope_vec[0].head,
            &mut rope_vec[0].visited,
          );
          move_knots(&mut rope_vec);
        }),
      Direction::Unsupported => {}
    };
  });

  res = rope_vec[4].visited.len();

  writeln!(io::stdout(), "{:?}", res)?;
  Ok(())
}

// -------------- helpers -------------------

fn move_tail(
  rope_state_tail: &mut Position,
  rope_state_head: &mut Position,
  rope_state_visited: &mut BTreeSet<String>,
) {
  match (
    rope_state_head.x - rope_state_tail.x,
    rope_state_head.y - rope_state_tail.y,
  ) {
    (0, 2) => {
      rope_state_tail.y += 1;
      rope_state_visited.insert(format!("[{:?},{:?}]", rope_state_tail.x, rope_state_tail.y));
    }
    (-1, 2) | (-2, 2) | (-2, 1) => {
      rope_state_tail.x -= 1;
      rope_state_tail.y += 1;
      rope_state_visited.insert(format!("[{:?},{:?}]", rope_state_tail.x, rope_state_tail.y));
    }

    (-2, 0) => {
      rope_state_tail.x -= 1;
      rope_state_visited.insert(format!("[{:?},{:?}]", rope_state_tail.x, rope_state_tail.y));
    }
    (-2, -1) | (-2, -2) | (-1, -2) => {
      rope_state_tail.x -= 1;
      rope_state_tail.y -= 1;
      rope_state_visited.insert(format!("[{:?},{:?}]", rope_state_tail.x, rope_state_tail.y));
    }

    (0, -2) => {
      rope_state_tail.y -= 1;
      rope_state_visited.insert(format!("[{:?},{:?}]", rope_state_tail.x, rope_state_tail.y));
    }
    (1, -2) | (2, -2) | (2, -1) => {
      rope_state_tail.x += 1;
      rope_state_tail.y -= 1;
      rope_state_visited.insert(format!("[{:?},{:?}]", rope_state_tail.x, rope_state_tail.y));
    }

    (2, 0) => {
      rope_state_tail.x += 1;
      rope_state_visited.insert(format!("[{:?},{:?}]", rope_state_tail.x, rope_state_tail.y));
    }
    (2, 1) | (2, 2) | (1, 2) => {
      rope_state_tail.x += 1;
      rope_state_tail.y += 1;
      rope_state_visited.insert(format!("[{:?},{:?}]", rope_state_tail.x, rope_state_tail.y));
    }
    _ => {}
  }
}

fn move_knots(rope_vec: &mut [&mut Rope]) {
  // boring offsetting stuff.
  (0..rope_vec.len() - 1).for_each(|idx| {
    let mut visited_idx = 0;
    if idx == rope_vec.len() - 2 {
      visited_idx = rope_vec.len() - 1;
    };

    move_tail(
      &mut rope_vec[idx + 1].head,
      &mut rope_vec[idx].tail,
      &mut rope_vec[0].visited,
    );

    move_tail(
      &mut rope_vec[idx + 1].tail,
      &mut rope_vec[idx + 1].head,
      &mut rope_vec[visited_idx].visited,
    );
  });
}
