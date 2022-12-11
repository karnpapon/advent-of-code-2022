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

#[derive(Debug, Clone)]
struct LongRope {
  pos: Position,
  next: Option<Box<LongRope>>,
  visited: BTreeSet<String>,
  nth: i32,
}

impl LongRope {
  fn new() -> Self {
    LongRope {
      pos: Position { x: 0, y: 0 },
      next: None,
      visited: BTreeSet::new(),
      nth: 0,
    }
  }

  fn append(&mut self, element: Box<LongRope>) {
    self.next = Some(element);
  }

  fn add_knot(&mut self, knot_num: usize) {
    let mut idx = 1;
    iter::repeat(1).take(knot_num).for_each(|_| {
      let rope = Box::new(LongRope {
        pos: Position { x: 0, y: 0 },
        next: None,
        visited: BTreeSet::new(),
        nth: idx,
      });

      match self.next {
        None => self.next = Some(rope),
        Some(ref mut next) => next.append(rope),
      };

      // self.next = Some(&rope);
      idx += 1;
    });
  }
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
          move_tail_y(&mut rope_state, Direction::Up(0));
        }),
      Direction::Down(val) => iter::repeat(1)
        .take(val.try_into().unwrap())
        .for_each(|val| {
          rope_state.head.y += val;
          move_tail_y(&mut rope_state, Direction::Down(0));
        }),
      Direction::Right(val) => iter::repeat(1)
        .take(val.try_into().unwrap())
        .for_each(|val| {
          rope_state.head.x += val;
          move_tail_x(&mut rope_state, Direction::Right(0));
        }),
      Direction::Left(val) => iter::repeat(1)
        .take(val.try_into().unwrap())
        .for_each(|val| {
          rope_state.head.x -= val;
          move_tail_x(&mut rope_state, Direction::Left(0));
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
    rope_state_h_1,
    rope_state_2_3,
    rope_state_4_5,
    rope_state_6_7,
    rope_state_8_9,
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
          rope_vec[0].head.y -= val;
          move_y(
            &mut rope_vec[0].tail,
            &mut rope_vec[0].head,
            Direction::Up(0),
            &mut rope_vec[0].visited,
          );

          move_y(
            &mut rope_vec[1].head,
            &mut rope_vec[0].tail,
            Direction::Up(0),
            &mut rope_vec[0].visited,
          );
          move_y(
            &mut rope_vec[1].tail,
            &mut rope_vec[1].head,
            Direction::Up(0),
            &mut rope_vec[0].visited,
          );

          move_y(
            &mut rope_vec[2].head,
            &mut rope_vec[1].tail,
            Direction::Up(0),
            &mut rope_vec[0].visited,
          );
          move_y(
            &mut rope_vec[2].tail,
            &mut rope_vec[2].head,
            Direction::Up(0),
            &mut rope_vec[0].visited,
          );

          move_y(
            &mut rope_vec[3].head,
            &mut rope_vec[2].tail,
            Direction::Up(0),
            &mut rope_vec[0].visited,
          );
          move_y(
            &mut rope_vec[3].tail,
            &mut rope_vec[3].head,
            Direction::Up(0),
            &mut rope_vec[0].visited,
          );

          move_y(
            &mut rope_vec[4].head,
            &mut rope_vec[3].tail,
            Direction::Up(0),
            &mut rope_vec[0].visited,
          );
          move_y(
            &mut rope_vec[4].tail,
            &mut rope_vec[4].head,
            Direction::Up(0),
            &mut rope_vec[4].visited,
          );
        }),
      Direction::Down(val) => iter::repeat(1)
        .take(val.try_into().unwrap())
        .for_each(|val| {
          rope_vec[0].head.y += val;
          move_y(
            &mut rope_vec[0].tail,
            &mut rope_vec[0].head,
            Direction::Down(0),
            &mut rope_vec[0].visited,
          );

          move_y(
            &mut rope_vec[1].head,
            &mut rope_vec[0].tail,
            Direction::Down(0),
            &mut rope_vec[0].visited,
          );
          move_y(
            &mut rope_vec[1].tail,
            &mut rope_vec[1].head,
            Direction::Down(0),
            &mut rope_vec[1].visited,
          );

          move_y(
            &mut rope_vec[2].head,
            &mut rope_vec[1].tail,
            Direction::Down(0),
            &mut rope_vec[1].visited,
          );
          move_y(
            &mut rope_vec[2].tail,
            &mut rope_vec[2].head,
            Direction::Down(0),
            &mut rope_vec[1].visited,
          );

          move_y(
            &mut rope_vec[3].head,
            &mut rope_vec[2].tail,
            Direction::Down(0),
            &mut rope_vec[1].visited,
          );
          move_y(
            &mut rope_vec[3].tail,
            &mut rope_vec[3].head,
            Direction::Down(0),
            &mut rope_vec[3].visited,
          );

          move_y(
            &mut rope_vec[4].head,
            &mut rope_vec[3].tail,
            Direction::Down(0),
            &mut rope_vec[0].visited,
          );
          move_y(
            &mut rope_vec[4].tail,
            &mut rope_vec[4].head,
            Direction::Down(0),
            &mut rope_vec[4].visited,
          );
        }),
      Direction::Right(val) => iter::repeat(1)
        .take(val.try_into().unwrap())
        .for_each(|val| {
          rope_vec[0].head.x += val;
          move_x(
            &mut rope_vec[0].tail,
            &mut rope_vec[0].head,
            Direction::Right(0),
            &mut rope_vec[0].visited,
          );

          move_x(
            &mut rope_vec[1].head,
            &mut rope_vec[0].tail,
            Direction::Right(0),
            &mut rope_vec[0].visited,
          );
          move_x(
            &mut rope_vec[1].tail,
            &mut rope_vec[1].head,
            Direction::Right(0),
            &mut rope_vec[0].visited,
          );

          move_x(
            &mut rope_vec[2].head,
            &mut rope_vec[1].tail,
            Direction::Right(0),
            &mut rope_vec[0].visited,
          );
          move_x(
            &mut rope_vec[2].tail,
            &mut rope_vec[2].head,
            Direction::Right(0),
            &mut rope_vec[0].visited,
          );

          move_x(
            &mut rope_vec[3].head,
            &mut rope_vec[2].tail,
            Direction::Right(0),
            &mut rope_vec[0].visited,
          );
          move_x(
            &mut rope_vec[3].tail,
            &mut rope_vec[3].head,
            Direction::Right(0),
            &mut rope_vec[0].visited,
          );

          move_x(
            &mut rope_vec[4].head,
            &mut rope_vec[3].tail,
            Direction::Right(0),
            &mut rope_vec[0].visited,
          );
          move_x(
            &mut rope_vec[4].tail,
            &mut rope_vec[4].head,
            Direction::Right(0),
            &mut rope_vec[4].visited,
          );
        }),
      Direction::Left(val) => iter::repeat(1)
        .take(val.try_into().unwrap())
        .for_each(|val| {
          rope_vec[0].head.x -= val;
          move_x(
            &mut rope_vec[0].tail,
            &mut rope_vec[0].head,
            Direction::Left(0),
            &mut rope_vec[0].visited,
          );

          move_x(
            &mut rope_vec[1].head,
            &mut rope_vec[0].tail,
            Direction::Left(0),
            &mut rope_vec[0].visited,
          );

          move_x(
            &mut rope_vec[1].tail,
            &mut rope_vec[1].head,
            Direction::Left(0),
            &mut rope_vec[0].visited,
          );

          move_x(
            &mut rope_vec[2].head,
            &mut rope_vec[1].tail,
            Direction::Left(0),
            &mut rope_vec[0].visited,
          );

          move_x(
            &mut rope_vec[2].tail,
            &mut rope_vec[2].head,
            Direction::Left(0),
            &mut rope_vec[0].visited,
          );

          move_x(
            &mut rope_vec[3].head,
            &mut rope_vec[2].tail,
            Direction::Left(0),
            &mut rope_vec[0].visited,
          );

          move_x(
            &mut rope_vec[3].tail,
            &mut rope_vec[3].head,
            Direction::Left(0),
            &mut rope_vec[0].visited,
          );

          move_x(
            &mut rope_vec[4].head,
            &mut rope_vec[3].tail,
            Direction::Left(0),
            &mut rope_vec[0].visited,
          );
          move_x(
            &mut rope_vec[4].tail,
            &mut rope_vec[4].head,
            Direction::Left(0),
            &mut rope_vec[4].visited,
          );
        }),
      Direction::Unsupported => {}
    };
  });

  res = rope_vec[4].visited.len();
  // test: 36
  println!("[0].head={:?}", rope_vec[0].head);
  println!("[0].tail={:?}", rope_vec[0].tail);

  println!("[1].head={:?}", rope_vec[1].head);
  println!("[1].tail={:?}", rope_vec[1].tail);

  println!("[2].head={:?}", rope_vec[2].head);
  println!("[2].tail={:?}", rope_vec[2].tail);

  println!("[3].head={:?}", rope_vec[3].head);
  println!("[3].tail={:?}", rope_vec[3].tail);

  println!("[4].head={:?}", rope_vec[4].head);
  println!("[4].tail={:?}", rope_vec[4].tail);
  writeln!(io::stdout(), "{:?}", res)?;
  Ok(())
}

// -------------- helpers -------------------

fn move_tail_y(rope_state: &mut Rope, dir: Direction) {
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

fn move_tail_x(rope_state: &mut Rope, dir: Direction) {
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

fn move_x(
  rope_state_tail: &mut Position,
  rope_state_head: &mut Position,
  dir: Direction,
  rope_state_visited: &mut BTreeSet<String>,
) {
  let step = match dir {
    Direction::Left(_) => 1,
    Direction::Right(_) => -1,
    _ => 0,
  };

  if is_consecutively_aligned(rope_state_head, rope_state_tail)
    || is_diagonally_aligned(rope_state_head, rope_state_tail)
  {
    return;
  }

  if rope_state_head.x != rope_state_tail.x {
    // normal move.
    if rope_state_head.y == rope_state_tail.y {
      rope_state_tail.x = rope_state_head.x + step;
      rope_state_visited.insert(format!("[{:?},{:?}]", rope_state_tail.x, rope_state_tail.y));
      return;
    }

    // special case:
    if (rope_state_head.x - rope_state_tail.x).abs() == 2
      && (rope_state_head.y - rope_state_tail.y).abs() == 2
    {
      rope_state_tail.x = rope_state_head.x;
      rope_state_tail.y = rope_state_head.y + step;
      rope_state_visited.insert(format!("[{:?},{:?}]", rope_state_tail.x, rope_state_tail.y));
      return;
    }

    // case: horizontal diagonally aligned.
    if (rope_state_head.x - rope_state_tail.x).abs() == 2 {
      rope_state_tail.x = rope_state_head.x + step;
      rope_state_tail.y = rope_state_head.y;
      rope_state_visited.insert(format!("[{:?},{:?}]", rope_state_tail.x, rope_state_tail.y));
      return;
    }

    // case: vertical diagonally aligned.
    if (rope_state_head.y - rope_state_tail.y).abs() == 2 {
      rope_state_tail.x = rope_state_head.x;
      rope_state_tail.y = rope_state_head.y + step;
      rope_state_visited.insert(format!("[{:?},{:?}]", rope_state_tail.x, rope_state_tail.y));
    }
  }
}

fn move_y(
  rope_state_tail: &mut Position,
  rope_state_head: &mut Position,
  dir: Direction,
  rope_state_visited: &mut BTreeSet<String>,
) {
  let step = match dir {
    Direction::Up(_) => 1,
    Direction::Down(_) => -1,
    _ => 0,
  };

  if is_consecutively_aligned(rope_state_head, rope_state_tail)
    || is_diagonally_aligned(rope_state_head, rope_state_tail)
  {
    return;
  };

  if rope_state_head.y != rope_state_tail.y {
    // normal
    if rope_state_head.x == rope_state_tail.x {
      rope_state_tail.y = rope_state_head.y + step;
      rope_state_visited.insert(format!("[{:?},{:?}]", rope_state_tail.x, rope_state_tail.y));
      return;
    }

    // special case:
    if (rope_state_head.x - rope_state_tail.x).abs() == 2
      && (rope_state_head.y - rope_state_tail.y).abs() == 2
    {
      rope_state_tail.x = rope_state_head.x + step;
      rope_state_tail.y = rope_state_head.y;
      rope_state_visited.insert(format!("[{:?},{:?}]", rope_state_tail.x, rope_state_tail.y));
      return;
    }

    // case: fill horizontal diagonally aligned.
    if (rope_state_head.x - rope_state_tail.x).abs() == 2 {
      rope_state_tail.x = rope_state_head.x + step;
      rope_state_tail.y = rope_state_head.y;
      rope_state_visited.insert(format!("[{:?},{:?}]", rope_state_tail.x, rope_state_tail.y));
    }

    // case: fill vertical diagonally aligned.
    if (rope_state_head.y - rope_state_tail.y).abs() == 2 {
      rope_state_tail.x = rope_state_head.x;
      rope_state_tail.y = rope_state_head.y + step;
      rope_state_visited.insert(format!("[{:?},{:?}]", rope_state_tail.x, rope_state_tail.y));
    }
  }
}

fn is_diagonally_aligned(rope_state_head: &Position, rope_state_tail: &Position) -> bool {
  (rope_state_head.x - rope_state_tail.x).abs() == 1 && rope_state_head.y == rope_state_tail.y
    || (rope_state_head.y - rope_state_tail.y).abs() == 1 && rope_state_head.x == rope_state_tail.x
}

fn is_consecutively_aligned(rope_state_head: &Position, rope_state_tail: &Position) -> bool {
  (rope_state_head.x - rope_state_tail.x).abs() == 1
    && (rope_state_head.y - rope_state_tail.y).abs() == 1
}

// U 2
// ..........................
// ..........................
// ..........................
// ................H.........
// ............54321.........
// ...........6..............
// ..........................
// ..........................
// ..........................
// ..........................
// ..........................

// U 3
// ..........................
// ..........................
// ................H.........
// ................1.........
// ............5432..........
// ...........6..............
// ..........................
// ..........................
// ..........................
// ..........................
// ..........................

// U 4
// ..........................
// ................H.........
// ................1.........
// .............5432.........
// ............6.............
// ...........7..............
// ..........................
// ..........................
// ..........................
// ..........................
// ..........................

//  L 1
// ..........................
// ..........................
// ..........................
// ..........................
// ..........................
// ..........................
// ..........................
// ..............H1..........
// ...............2..........
// ...............3..........
// ...............4..........
// ...............5..........
// ..............6...........
// .............7............
// ............8.............
// ...........9..............
// ..........................
// ..........................
// ..........................
// ..........................
// ..........................

//  L 2
// x= 3,4,4,4,4,4,3,2,1,0
// y= -8,-8,-7,-6,-5,-4,-3,-2,-1,0
// ..........................
// ..........................
// ..........................
// ..........................
// ..........................
// ..........................
// ..........................
// ..............H1..........
// ...............2..........
// ...............3..........
// ...............4..........
// ...............5..........
// ..............6...........
// .............7............
// ............8.............
// ...........9..............
// ..........................
// ..........................
// ..........................
// ..........................
// ..........................

// ....H.
// ....1.
// ...32.
// .4....
// 5.....

// ..H1..
// ...2..
// ..43..
// .5....
// 6.....

// ..........................
// ..........................
// ..........................
// ..........................
// ..........................
// ..........................
// ..........................
// ..........................
// ..........................
// ..........................
// ................987654....
// .......................321
// .........................H
// ..........................
// ..........................
// ...........s..............
// ..........................
// ..........................
// ..........................
// ..........................
// ..........................
