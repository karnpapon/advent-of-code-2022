use std::collections::HashMap;
use std::io::{self, Read, Write};
use std::vec;
use std::{error::Error, result};

type Result<T> = result::Result<T, Box<dyn Error>>;

#[derive(Debug, Clone)]
enum Command {
  ChangeDirectory(DirectoryDirection),
  Listing(Vec<FileType>),
  FileList,
}

#[derive(Debug, Clone)]
enum DirectoryDirection {
  Root,
  Up,
  Down(String),
}

#[derive(Debug, Clone)]
enum FileType {
  File { size: i32, _name: String },
  Folder(String),
}

#[derive(Debug)]
struct File {
  size: i32,
}

fn parse_cmd(input: &str) -> Command {
  let cmd = input
    .lines()
    .map(|line| {
      let cmd_list = line.split(' ').collect::<Vec<_>>();
      match cmd_list[0] {
        "cd" => match cmd_list[1] {
          "/" => Some(Command::ChangeDirectory(DirectoryDirection::Root)),
          ".." => Some(Command::ChangeDirectory(DirectoryDirection::Up)),
          name => Some(Command::ChangeDirectory(DirectoryDirection::Down(
            name.to_string(),
          ))),
        },
        "ls" => {
          let file_list = input.lines().skip(1).map(parse_type).collect::<Vec<_>>();
          Some(Command::Listing(file_list))
        }
        _ => Some(Command::FileList),
      }
    })
    .collect::<Vec<_>>();

  cmd[0].clone().unwrap()
}

fn parse_type(cmd: &str) -> FileType {
  let cmd_list = cmd.split(' ').collect::<Vec<_>>();
  match cmd_list[0] {
    "dir" => FileType::Folder(cmd_list[1].to_string()),
    _ => FileType::File {
      size: cmd_list[0].parse::<i32>().unwrap(),
      _name: cmd_list[1].to_string(),
    },
  }
}

// Find all of the directories with a total size of at most 100000.
// What is the sum of the total sizes of those directories?
fn solve_part1(input: &str) -> Result<()> {
  let mut filesystem: HashMap<String, Vec<File>> = HashMap::new();
  let mut context: Vec<String> = vec![];
  let mut sizes: HashMap<String, i32> = HashMap::new();

  input
    .split("$ ")
    .skip(1) // skip first empty string
    .for_each(|line| match parse_cmd(line) {
      Command::ChangeDirectory(cmd) => match cmd {
        DirectoryDirection::Root => context.push("".to_string()),
        DirectoryDirection::Up => {
          context.pop();
        }
        DirectoryDirection::Down(dir) => context.push(dir),
      },
      Command::Listing(files) => {
        filesystem.entry(context.join("/")).or_insert(vec![]);

        files.iter().for_each(|file| match file {
          FileType::File { size, _name } => {
            filesystem.entry(context.join("/")).and_modify(|vec| {
              vec.push(File { size: *size });
            });
          }
          FileType::Folder(_) => (),
        })
      }
      // skip filelist (after `ls`),
      // since we already collect the list in `Command::Listing` process.
      Command::FileList => (),
    });

  filesystem.iter().for_each(|(path, files)| {
    let dirs = path.split('/').collect::<Vec<&str>>();
    let size = files.iter().map(|File { size, .. }| size).sum::<i32>();

    (0..dirs.len()).for_each(|i| {
      sizes
        .entry(dirs[0..=i].join("/"))
        .and_modify(|v| *v += size)
        .or_insert(size);
    })
  });

  let res = sizes
    .iter()
    .filter_map(|(_, &size)| if size < 100000 { Some(size) } else { None })
    .sum::<i32>();

  writeln!(io::stdout(), "{:?}", res)?;
  Ok(())
}

// Find the smallest directory that, if deleted,
// would free up enough space on the filesystem to run the update.
// What is the total size of that directory?
fn solve_part2(input: &str) -> Result<()> {
  let mut filesystem: HashMap<String, Vec<File>> = HashMap::new();
  let mut context: Vec<String> = vec![];
  let mut sizes: HashMap<String, i32> = HashMap::new();

  input
    .split("$ ")
    .skip(1) // skip first empty string
    .for_each(|line| match parse_cmd(line) {
      Command::ChangeDirectory(cmd) => match cmd {
        DirectoryDirection::Root => context.push("".to_string()),
        DirectoryDirection::Up => {
          context.pop();
        }
        DirectoryDirection::Down(dir) => context.push(dir),
      },
      Command::Listing(files) => {
        filesystem.entry(context.join("/")).or_insert(vec![]);

        files.iter().for_each(|file| match file {
          FileType::File { size, _name } => {
            filesystem.entry(context.join("/")).and_modify(|vec| {
              vec.push(File { size: *size });
            });
          }
          FileType::Folder(_) => (),
        })
      }
      Command::FileList => (),
    });

  filesystem.iter().for_each(|(path, files)| {
    let dirs = path.split('/').collect::<Vec<&str>>();
    let size = files.iter().map(|File { size, .. }| size).sum::<i32>();

    (0..dirs.len()).for_each(|i| {
      sizes
        .entry(dirs[0..=i].join("/"))
        .and_modify(|v| *v += size)
        .or_insert(size);
    })
  });

  // total: 70,000,000
  // require: 30,000,000

  // current_used: 40,268,565
  // current_free_space: 29,731,435 (70,000,000 - 40,268,565)
  // space_to_freeup: 268,565 (30,000,000 - 29,731,435)

  let total = 70_000_000;
  let require = 30_000_000;

  let current_used = sizes.get("").unwrap();
  let current_free_space = total - current_used;
  let space_to_freeup = require - current_free_space;

  // find dir size that, at least when being freed up
  // there will be enough space for updating.
  let s = sizes
    .iter()
    .filter_map(|(_, &size)| {
      if size > space_to_freeup {
        Some(size)
      } else {
        None
      }
    })
    .collect::<Vec<i32>>();

  let res = s.iter().min().unwrap(); // get the smallest size of dir-size that match space_to_freeup.
  writeln!(io::stdout(), "{:?}", res)?;
  Ok(())
}

fn main() -> Result<()> {
  let mut input = String::new();
  io::stdin().read_to_string(&mut input)?;

  solve_part1(&input)?;
  solve_part2(&input)?;
  Ok(())
}
