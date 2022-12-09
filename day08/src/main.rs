use std::collections::HashMap;
use std::error::Error;
use std::io::{self, Read, Write};

type Result<T> = ::std::result::Result<T, Box<dyn Error>>;

fn main() -> Result<()> {
  let mut input = String::new();
  io::stdin().read_to_string(&mut input)?;
  solve_part1(&input)?;
  solve_part2(&input)?;
  Ok(())
}

// how many trees are visible from outside the grid?
fn solve_part1(input: &str) -> Result<()> {
  let mut res = 0;

  let grids = input.lines().enumerate().fold(vec![], |mut acc, line| {
    acc.push(
      line
        .1
        .split("")
        .filter(|x| !x.is_empty())
        .map(|l| l.parse::<i32>().unwrap())
        .collect::<Vec<i32>>(),
    );
    acc
  });

  let mut map: HashMap<String, i32> = HashMap::new();

  grids
    .iter()
    .skip(1) // skip top edge
    .enumerate()
    .take_while(|(idx, _)| *idx < grids.len() - 2) // skip bottom edge.
    .fold((0, Vec::new()), |mut acc, curr| {
      // iter only interior trees.
      curr
        .1
        .iter()
        .skip(1) // skip left edge.
        .enumerate()
        .take_while(|(idx, _)| *idx < grids.len() - 2) // skip right edge.
        .for_each(|(index_x, digit)| {
          // individual interior tree.
          let index_y = acc.0;
          let current_x_idx = index_x + 1;
          let current_y_idx = index_y + 1;

          let mut upward_idx = index_y;
          let mut downward_idx = index_y + 2;
          let mut leftward_idx = current_x_idx - 1;
          let mut rightward_idx = current_x_idx + 1;

          let mut find_downward = true;
          let mut find_leftward = true;
          let mut find_rightward = true;
          let mut find_upward = true;
          let mut up_idx = current_y_idx;
          let mut left_idx = current_x_idx;

          // find downward
          while downward_idx < grids.len() && find_downward {
            if digit <= &grids[downward_idx][current_x_idx] {
              find_downward = false;
              break;
            }
            downward_idx += 1;
          }

          // find rightward
          while rightward_idx < curr.1.len() && find_rightward {
            if digit <= &curr.1[rightward_idx] {
              find_rightward = false;
              break;
            }
            rightward_idx += 1;
          }

          // find leftward
          while left_idx > 0 || find_leftward {
            if digit <= &curr.1[leftward_idx] {
              find_leftward = false;
              break;
            }
            left_idx -= 1;
            if leftward_idx == 0 {
              find_leftward = false;
              break;
            }
            leftward_idx -= 1;
          }

          // find upward
          while up_idx > 0 || find_upward {
            if digit <= &grids[upward_idx][current_x_idx] {
              find_upward = false;
              break;
            }
            up_idx -= 1;
            if upward_idx == 0 {
              find_upward = false;
              break;
            }
            upward_idx -= 1;
          }

          if downward_idx == grids.len()
            || up_idx == 0
            || rightward_idx == curr.1.len()
            || left_idx == 0
          {
            map
              .entry(format!("[{current_x_idx:},{current_y_idx:}]"))
              .and_modify(|counter| *counter += 1)
              .or_insert(1);
          }
        });

      acc = (acc.0 + 1, curr.1.to_vec());
      acc
    });

  let visible_edges_tree = (grids[0].len() * 2) + ((grids.len() - 2) * 2);
  res = map.len() + visible_edges_tree;

  writeln!(io::stdout(), "{:?}", res)?;
  Ok(())
}

// What is the highest scenic score possible for any tree?
// test: 8
fn solve_part2(input: &str) -> Result<()> {
  let mut res = 0;

  let grids = input.lines().enumerate().fold(vec![], |mut acc, line| {
    acc.push(
      line
        .1
        .split("")
        .filter(|x| !x.is_empty())
        .map(|l| l.parse::<i32>().unwrap())
        .collect::<Vec<i32>>(),
    );
    acc
  });

  let mut map: HashMap<String, i32> = HashMap::new();

  grids
    .iter()
    .skip(1) // skip top edge
    .enumerate()
    .take_while(|(idx, _)| *idx < grids.len() - 2) // skip bottom edge.
    .fold((0, Vec::new()), |mut acc, curr| {
      // iter only interior trees.
      curr
        .1
        .iter()
        .skip(1) // skip left edge.
        .enumerate()
        .take_while(|(idx, _)| *idx < grids.len() - 2) // skip right edge.
        .for_each(|(index_x, digit)| {
          // individual interior tree.
          let index_y = acc.0;
          let current_x_idx = index_x + 1;
          let current_y_idx = index_y + 1;

          let mut upward_idx = index_y;
          let mut downward_idx = index_y + 2;
          let mut leftward_idx = current_x_idx - 1;
          let mut rightward_idx = current_x_idx + 1;

          let mut find_downward = true;
          let mut find_leftward = true;
          let mut find_rightward = true;
          let mut find_upward = true;
          let mut up_idx = current_y_idx;
          let mut left_idx = current_x_idx;

          let mut count_upward = 0;
          let mut count_downward = 0;
          let mut count_leftward = 0;
          let mut count_rightward = 0;

          // find downward
          while downward_idx < grids.len() && find_downward {
            count_downward += 1;
            if digit <= &grids[downward_idx][current_x_idx] {
              find_downward = false;
              break;
            }
            downward_idx += 1;
          }

          // find rightward
          while rightward_idx < curr.1.len() && find_rightward {
            count_rightward += 1;
            if digit <= &curr.1[rightward_idx] {
              find_rightward = false;
              break;
            }
            rightward_idx += 1;
          }

          // find leftward
          while left_idx > 0 || find_leftward {
            count_leftward += 1;
            if digit <= &curr.1[leftward_idx] {
              find_leftward = false;
              break;
            }
            left_idx -= 1;
            if leftward_idx == 0 {
              find_leftward = false;
              break;
            }
            leftward_idx -= 1;
          }

          // find upward
          while up_idx > 0 || find_upward {
            count_upward += 1;
            if digit <= &grids[upward_idx][current_x_idx] {
              find_upward = false;
              break;
            }
            up_idx -= 1;
            if upward_idx == 0 {
              find_upward = false;
              break;
            }
            upward_idx -= 1;
          }

          let scenic_score = count_downward * count_leftward * count_rightward * count_upward;
          map.insert(format!("[{current_x_idx:},{current_y_idx:}]"), scenic_score);
        });

      acc = (acc.0 + 1, curr.1.to_vec());
      acc
    });

  res = *map
    .iter()
    .max_by(|a, b| a.1.cmp(b.1))
    .map(|(_, v)| v)
    .unwrap();

  writeln!(io::stdout(), "{:?}", res)?;
  Ok(())
}
