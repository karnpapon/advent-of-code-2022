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
  register_counter: i32,
  stack: Vec<(Instructions, i32)>,
  signal_strength: Vec<i32>,
}

impl Processor {
  fn new() -> Processor {
    Processor {
      register_counter: 1,
      stack: vec![],
      signal_strength: vec![],
    }
  }
  fn countdown(&mut self) {
    self
      .stack
      .iter_mut()
      .skip_while(|x| x.0 == Instructions::Done)
      .for_each(|(_, ref mut tick)| *tick -= 1);
  }

  fn get_remove_index(&self) -> Vec<usize> {
    let remove_index = self
      .stack
      .iter()
      .enumerate()
      .filter_map(|(idx, (ins, ref tick))| {
        if *tick == 0 && *ins != Instructions::Done {
          Some(idx)
        } else {
          None
        }
      })
      .collect::<Vec<usize>>();
    remove_index
  }

  fn collect_signal(&mut self, cycles: &i32) {
    match cycles {
      20 => self.signal_strength.push(self.register_counter * 20),
      60 => self.signal_strength.push(self.register_counter * 60),
      100 => self.signal_strength.push(self.register_counter * 100),
      140 => self.signal_strength.push(self.register_counter * 140),
      180 => self.signal_strength.push(self.register_counter * 180),
      220 => self.signal_strength.push(self.register_counter * 220),
      _ => {}
    };
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

    processor.countdown();

    let remove_idx = processor.get_remove_index();

    remove_idx.iter().for_each(|idx| {
      let register_add_value = &processor.stack[*idx];
      match register_add_value.0 {
        Instructions::AddX(val) => {
          (0..2).for_each(|_| {
            processor.collect_signal(&cycles);
            cycles += 1;
          });
          processor.register_counter += val;
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

  // [420, 1140, 1800, 2940, 2880, 3960]
  // 13140
  println!("processor = {:?}", processor.signal_strength);
  let res: i32 = processor.signal_strength.iter().sum();
  writeln!(io::stdout(), "{:?}", res)?;
  Ok(())
}

// Render the image given by your program. What eight capital letters appear on your CRT?
fn solve_part2(input: &str) -> Result<()> {
  let mut res = 0;

  writeln!(io::stdout(), "{:?}", res)?;
  Ok(())
}
