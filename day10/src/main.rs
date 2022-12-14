use std::error::Error;
use std::io::{self, Read, Write};

type Result<T> = ::std::result::Result<T, Box<dyn Error>>;

#[derive(Debug, PartialEq)]
enum Instructions {
  Noop,
  AddX(i32),
  Done,
}

#[derive(Debug)]
struct Processor {
  x_register: i32,
  stack: Vec<(Instructions, i32)>,
  signal_strength: Vec<i32>,
}

impl Processor {
  fn new() -> Processor {
    Processor {
      x_register: 1,
      stack: vec![],
      signal_strength: vec![],
    }
  }

  fn countdown(&mut self) -> Vec<usize> {
    let remove_index = self
      .stack
      .iter_mut()
      .enumerate()
      .skip_while(|(_, (x, _))| *x == Instructions::Done)
      .filter_map(|(idx, (_, ref mut tick))| {
        *tick -= 1;
        if *tick == 0 {
          Some(idx)
        } else {
          None
        }
      })
      .collect::<Vec<usize>>();
    remove_index
  }

  fn collect_signal(&mut self, cycles: &i32) {
    let nth_signal = [20, 60, 100, 140, 180, 220];
    if let Some(nth) = nth_signal.iter().find(|nth| *nth == cycles) {
      self.signal_strength.push(self.x_register * nth)
    }
  }
}

fn parse(line: &str) -> Instructions {
  let l = line.split(' ').collect::<Vec<_>>();

  match l[0] {
    "noop" => Instructions::Noop,
    "addx" => Instructions::AddX(l[1].parse::<i32>().unwrap()),
    _ => Instructions::Done,
  }
}

fn main() -> Result<()> {
  let mut input = String::new();
  io::stdin().read_to_string(&mut input)?;
  solve_part1(&input)?;
  solve_part2(&input)?;
  Ok(())
}

// Find the signal strength during the 20th, 60th, 100th, 140th, 180th, and 220th cycles.
// What is the sum of these six signal strengths?
fn solve_part1(input: &str) -> Result<()> {
  let mut cycles = 1;
  let mut processor = Processor::new();

  input.lines().for_each(|line| {
    match parse(line) {
      Instructions::Noop => processor.stack.push((Instructions::Noop, 1)),
      Instructions::AddX(val) => processor.stack.push((Instructions::AddX(val), 2)),
      Instructions::Done => {}
    };

    let marked_done_idx = processor.countdown();

    marked_done_idx.iter().for_each(|idx| {
      let register_add_value = &processor.stack[*idx];
      match register_add_value.0 {
        Instructions::AddX(val) => {
          (0..2).for_each(|_| {
            processor.collect_signal(&cycles);
            cycles += 1;
          });
          processor.x_register += val;
        }
        Instructions::Noop => {
          processor.collect_signal(&cycles);
          cycles += 1;
        }
        Instructions::Done => {}
      }
      processor.stack[*idx] = (Instructions::Done, 0);
    });
  });

  let res: i32 = processor.signal_strength.iter().sum();
  writeln!(io::stdout(), "{:?}", res)?;
  Ok(())
}

// Render the image given by your program. What eight capital letters appear on your CRT?
fn solve_part2(input: &str) -> Result<()> {
  let mut cycles = 1;
  let mut processor = Processor::new();
  let mut sprite_pos = [0, 1, 2];
  let mut crt_pos = 0;
  let mut crt_pixels: String = String::new();

  input.lines().for_each(|line| {
    match parse(line) {
      Instructions::Noop => processor.stack.push((Instructions::Noop, 1)),
      Instructions::AddX(val) => processor.stack.push((Instructions::AddX(val), 2)),
      Instructions::Done => {}
    };

    let marked_done_idx = processor.countdown();

    marked_done_idx.iter().for_each(|idx| {
      let register_add_value = &processor.stack[*idx];
      match register_add_value.0 {
        Instructions::AddX(val) => {
          (0..2).for_each(|_| {
            processor.collect_signal(&cycles);
            match sprite_pos.contains(&crt_pos) {
              true => crt_pixels.push('#'),
              false => crt_pixels.push(' '),
            };

            if cycles % 40 == 0 {
              crt_pixels.push('\n');
              sprite_pos.iter_mut().for_each(|pos| *pos += 40);
            }
            cycles += 1;
            crt_pos += 1;
          });
          sprite_pos.iter_mut().for_each(|pos| *pos += val);
          processor.x_register += val;
        }
        Instructions::Noop => {
          processor.collect_signal(&cycles);
          match sprite_pos.contains(&crt_pos) {
            true => crt_pixels.push('#'),
            false => crt_pixels.push(' '),
          };

          if cycles % 40 == 0 {
            crt_pixels.push('\n');
            sprite_pos.iter_mut().for_each(|pos| *pos += 40);
          }
          cycles += 1;
          crt_pos += 1;
        }
        Instructions::Done => {}
      }
      processor.stack[*idx] = (Instructions::Done, 0);
    });
  });

  println!("{}", crt_pixels);
  Ok(())
}
