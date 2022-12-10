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

// #[derive(Debug, Clone)]
// struct LongRope {
//   next: Option<Box<LongRope>>,
//   pos: Position,
//   visited: BTreeSet<String>,
//   nth: i32,
// }

// impl LongRope {
//   fn new() -> Self {
//     LongRope {
//       next: None,
//       pos: Position { x: 0, y: 0 },
//       visited: BTreeSet::new(),
//       nth: 0,
//     }
//   }

//   fn append(&mut self, element: Box<LongRope>) {
//     self.next = Some(element);
//   }

//   fn add_knot(&mut self, knot_num: usize) {
//     let mut idx = 1;
//     iter::repeat(1).take(knot_num).for_each(|num| {
//       let rope = Box::new(LongRope {
//         next: None,
//         pos: Position { x: 0, y: 0 },
//         visited: BTreeSet::new(),
//         nth: idx,
//       });

//       match self.next {
//         None => self.next = Some(rope.clone()),
//         Some(ref mut body) => body.append(rope.clone()),
//       };

//       self.next = Some(rope);
//       idx += 1;
//     });
//   }
// }

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
          move_y(&mut rope_state, Direction::Up(0));
        }),
      Direction::Down(val) => iter::repeat(1)
        .take(val.try_into().unwrap())
        .for_each(|val| {
          rope_state.head.y += val;
          move_y(&mut rope_state, Direction::Down(0));
        }),
      Direction::Right(val) => iter::repeat(1)
        .take(val.try_into().unwrap())
        .for_each(|val| {
          rope_state.head.x += val;
          move_x(&mut rope_state, Direction::Right(0));
        }),
      Direction::Left(val) => iter::repeat(1)
        .take(val.try_into().unwrap())
        .for_each(|val| {
          rope_state.head.x -= val;
          move_x(&mut rope_state, Direction::Left(0));
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
  writeln!(io::stdout(), "{:?}", res)?;
  Ok(())
}

// -------------- helpers -------------------

fn move_y(rope_state: &mut Rope, dir: Direction) {
  let step = match dir {
    Direction::Up(_) => 1,
    Direction::Down(_) => -1,
    _ => 0,
  };

  if rope_state.head.y != rope_state.tail.y {
    if (rope_state.head.y - rope_state.tail.y).abs() == 2 && rope_state.tail.x != rope_state.head.x
    {
      rope_state.tail.x = rope_state.head.x;
      rope_state.tail.y = rope_state.head.y + step;
      rope_state
        .visited
        .insert(format!("[{:?},{:?}]", rope_state.tail.x, rope_state.tail.y));
    }

    if rope_state.head.x == rope_state.tail.x {
      rope_state.tail.y = rope_state.head.y + step;
      rope_state
        .visited
        .insert(format!("[{:?},{:?}]", rope_state.tail.x, rope_state.tail.y));
    }
  }
}

fn move_x(rope_state: &mut Rope, dir: Direction) {
  let step = match dir {
    Direction::Left(_) => 1,
    Direction::Right(_) => -1,
    _ => 0,
  };

  if rope_state.head.x != rope_state.tail.x {
    // if diagonally aligned.
    if (rope_state.head.x - rope_state.tail.x).abs() == 2 && rope_state.tail.y != rope_state.head.y
    {
      rope_state.tail.x = rope_state.head.x + step;
      rope_state.tail.y = rope_state.head.y;
      rope_state
        .visited
        .insert(format!("[{:?},{:?}]", rope_state.tail.x, rope_state.tail.y));
    }

    if rope_state.head.y == rope_state.tail.y {
      rope_state.tail.x = rope_state.head.x + step;
      rope_state
        .visited
        .insert(format!("[{:?},{:?}]", rope_state.tail.x, rope_state.tail.y));
    }
  }
}
