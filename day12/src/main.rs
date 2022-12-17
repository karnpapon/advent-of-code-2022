use itertools::Itertools;
use petgraph::algo::dijkstra;
use petgraph::prelude::*;
// use std::collections::BTreeSet;
use std::error::Error;
// use std::fmt::format;
// use std::fs::File;
use std::io::{self, Read, Write};

type Result<T> = ::std::result::Result<T, Box<dyn Error>>;

// #[derive(Debug)]
// enum Direction {
//   Up,
//   Down,
//   Left,
//   Right,
//   Start,
// }

// #[derive(Debug)]
// struct Position {
//   val: u8,
//   x: usize,
//   y: usize,
//   count: i32,
//   visited: BTreeSet<String>,
// }

fn main() -> Result<()> {
  let mut input = String::new();
  io::stdin().read_to_string(&mut input)?;
  solve_part1(&input)?;
  solve_part2(&input)?;
  Ok(())
}

// fn check_neighbor(
//   current_idx: usize,
//   row_len: usize,
//   start_row_pos: usize,
//   end_row_pos: usize,
// ) -> (bool, bool, bool, bool) {
//   let go_up = match current_idx.overflowing_sub(row_len) {
//     (_, false) => {
//       (start_row_pos - row_len..=end_row_pos - row_len).contains(&(current_idx - row_len))
//     }
//     (_, true) => false,
//   };
//   // println!(
//   //   "start_row_pos={start_row_pos:}, row_len={row_len:}, end_row_pos={end_row_pos:}, current_idx={current_idx:}"
//   // );
//   let go_down = match current_idx + row_len > 39 {
//     true => false,
//     false => (start_row_pos + row_len..=end_row_pos + row_len).contains(&(current_idx + row_len)),
//   };
//   let go_left = current_idx > start_row_pos;
//   let go_right = current_idx < end_row_pos;

//   (go_up, go_down, go_left, go_right)
// }

// fn walk(
//   current_point: &mut Position,
//   row_len: usize,
//   map: &Vec<&u8>,
//   permutations: &mut Vec<i32>,
// ) -> Option<i32> {
//   current_point.count += 1;
//   current_point
//     .visited
//     .insert(format!("({:?}:{:?})", current_point.x, current_point.y));
//   println!("current_point={:?}", current_point);

//   let current_idx = current_point.x + (current_point.y * row_len); // converting cartesian coord to array indexing.
//   let row = (current_idx / row_len) + 1; // start from 1.
//   let end_row_pos = (row_len * row) - 1; // boring offsetting stuff.
//   let start_row_pos = (end_row_pos - row_len) + 1; // also this.

//   let (mut top, mut bottom, mut left, mut right) =
//     check_neighbor(current_idx, row_len, start_row_pos, end_row_pos);

//   println!("top={top:}, bottom={bottom:}, left={left:}, right={right:}");

//   match (top, bottom, left, right) {
//     (true, _, _, _) => {
//       // case: find possible next step.
//       if *map[current_idx - row_len] - current_point.val == 1
//         || *map[current_idx - row_len] == current_point.val
//       {
//         current_point.val = *map[current_idx - row_len];
//         current_point.y -= 1;
//         walk(current_point, row_len, map, permutations);
//       }

//       // case: target was found!.
//       if *map[current_idx - row_len] == 69 {
//         return Some(current_point.count);
//       };

//       walk(current_point, row_len, map, permutations)
//       // None
//     }
//     (_, true, _, _) => {
//       // case: find possible next step.
//       if *map[current_idx + row_len] - current_point.val == 1
//         || *map[current_idx + row_len] == current_point.val
//       {
//         current_point.val = *map[current_idx + row_len];
//         current_point.y += 1;
//         walk(current_point, row_len, map, permutations);
//       }

//       // case: target was found!.
//       if *map[current_idx + row_len] == 69 {
//         return Some(current_point.count);
//       };

//       walk(current_point, row_len, map, permutations)
//       // None
//     }
//     (_, _, true, _) => {
//       // case: find possible next step.
//       if *map[current_idx - 1] - current_point.val == 1
//         || *map[current_idx - 1] == current_point.val
//       {
//         current_point.val = *map[current_idx - 1];
//         current_point.x -= 1;
//         walk(current_point, row_len, map, permutations);
//       }
//       // case: target was found!.
//       if *map[current_idx - 1] == 69 {
//         return Some(current_point.count);
//       };

//       walk(current_point, row_len, map, permutations)
//       // None
//     }
//     (_, _, _, true) => {
//       // case: find possible next step.
//       if *map[current_idx + 1] - current_point.val == 1
//         || *map[current_idx + 1] == current_point.val
//       {
//         current_point.val = *map[current_idx + 1];
//         current_point.x += 1;
//         walk(current_point, row_len, map, permutations);
//       }

//       // case: target was found!.
//       if *map[current_idx + 1] == 69 {
//         return Some(current_point.count);
//       };

//       walk(current_point, row_len, map, permutations)
//       // None
//     }
//     _ => None,
//   }
// }

fn get_point(map: &[Vec<char>], char: char) -> Option<(i32, i32)> {
  map
    .iter()
    .enumerate()
    .flat_map(|(i, v)| v.iter().enumerate().zip(std::iter::repeat(i)))
    .find_map(|((x, &c), y)| {
      if c == char {
        Some((x as i32, y as i32))
      } else {
        None
      }
    })
}

// Q: What is the fewest steps required to move from your current position
// to the location that should get the best signal?
fn solve_part1(input: &str) -> Result<()> {
  let row_len = input.lines().next().unwrap().len() as i32;
  let map = input
    .lines()
    .map(|line| line.chars().collect::<Vec<char>>())
    .collect::<Vec<_>>();

  let start = get_point(&map, 'S').unwrap();
  let end = get_point(&map, 'E').unwrap();

  let map: Vec<Vec<char>> = map
    .iter()
    .map(|vec| {
      vec
        .iter()
        .map(|c| match c {
          'S' => 'a',
          'E' => 'z',
          v => *v,
        })
        .collect()
    })
    .collect();

  let range_y: Vec<i32> = (0..(map.len() as i32)).collect();
  let range_x: Vec<i32> = (0..row_len).collect();
  let cartesian_map: Vec<(i32, i32)> = range_y
    .iter()
    .flat_map(|y| range_x.iter().map(|x| (*y, *x)).collect::<Vec<_>>())
    .collect();

  let edges = cartesian_map
    .into_iter()
    .flat_map(|(y, x)| {
      let neighbors = vec![(x - 1, y), (x + 1, y), (x, y - 1), (x, y + 1)];
      let current_node_id = (x, y);
      neighbors
        .iter()
        .filter_map(|cell| {
          map
            .get(cell.1 as usize)
            .and_then(|vec| vec.get(cell.0 as usize))
            .and_then(|existing_cell| {
              let current_node_height = map[y as usize][x as usize];
              if current_node_height as u8 + 1 >= *existing_cell as u8 {
                Some((
                  (current_node_id.0, current_node_id.1, current_node_height),
                  (cell.0, cell.1, *existing_cell),
                ))
              } else {
                None
              }
            })
        })
        .collect::<Vec<_>>()
    })
    .collect::<Vec<((i32, i32, char), (i32, i32, char))>>();

  let graph = DiGraphMap::<(i32, i32, char), ()>::from_edges(&edges);

  let res = dijkstra(
    &graph,
    (start.0, start.1, 'a'),
    Some((end.0, end.1, 'z')),
    |_| 1,
  );

  writeln!(io::stdout(), "{:?}", res[&(end.0, end.1, 'z')])?;
  Ok(())
}

// What is the fewest steps required to move starting from any square
//  with elevation a to the location that should get the best signal?
fn solve_part2(input: &str) -> Result<()> {
  let row_len = input.lines().next().unwrap().len() as i32;
  let map = input
    .lines()
    .map(|line| line.chars().collect::<Vec<char>>())
    .collect::<Vec<_>>();

  let end = get_point(&map, 'E').unwrap();

  let map: Vec<Vec<char>> = map
    .iter()
    .map(|vec| {
      vec
        .iter()
        .map(|c| match c {
          'S' => 'a',
          'E' => 'z',
          v => *v,
        })
        .collect()
    })
    .collect();

  let range_y: Vec<i32> = (0..(map.len() as i32)).collect();
  let range_x: Vec<i32> = (0..row_len).collect();
  let cartesian_map: Vec<(i32, i32)> = range_y
    .iter()
    .flat_map(|y| range_x.iter().map(|x| (*y, *x)).collect::<Vec<_>>())
    .collect();

  let edges = cartesian_map
    .into_iter()
    .flat_map(|(y, x)| {
      let neighbors = vec![(x - 1, y), (x + 1, y), (x, y - 1), (x, y + 1)];
      let current_node_id = (x, y);
      neighbors
        .iter()
        .filter_map(|cell| {
          map
            .get(cell.1 as usize)
            .and_then(|vec| vec.get(cell.0 as usize))
            .and_then(|existing_cell| {
              let current_node_height = map[y as usize][x as usize];
              if current_node_height as u8 + 1 >= *existing_cell as u8 {
                Some((
                  (current_node_id.0, current_node_id.1, current_node_height),
                  (cell.0, cell.1, *existing_cell),
                ))
              } else {
                None
              }
            })
        })
        .collect::<Vec<_>>()
    })
    .collect::<Vec<((i32, i32, char), (i32, i32, char))>>();

  let graph = DiGraphMap::<(i32, i32, char), ()>::from_edges(edges.iter().map(|(a, b)| (*b, *a)));

  let dijk = dijkstra(&graph, (end.0, end.1, 'z'), None, |_| 1);

  let mut results: Vec<i32> = dijk
    .iter()
    .filter_map(
      |(node, cost)| {
        if node.2 == 'a' {
          Some(*cost)
        } else {
          None
        }
      },
    )
    .collect();
  results.sort();
  let res = results.first().unwrap();
  writeln!(io::stdout(), "{:?}", res)?;
  Ok(())
}
