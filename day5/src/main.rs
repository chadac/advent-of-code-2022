use std::fs;
use regex::Regex;


type Stack = Vec<char>;
type Stacks = Vec<Stack>;

struct Input {
  stacks: Stacks,
  moves: Vec<Move>,
}

#[derive(Debug, PartialEq)]
struct Move {
  amount: u32,
  from: usize,
  to: usize,
}


fn transfer(amount: u32, from: &mut Stack, to: &mut Stack) {
  for _ in 0..amount {
    let x = from.pop();
    match x {
      Some(c) => to.push(c),
      None => ()
    }
  }
}

impl Move {
  fn apply(&self, stacks: &mut Stacks) {
    let from = &mut stacks[self.from - 1];
    let mut new: Stack = vec![];
    transfer(self.amount, from, &mut new);

    stacks[self.to - 1].append(&mut new);
  }

  fn new_apply(&self, stacks: &mut Stacks) {
    let mut mid: Vec<char> = vec![];
    transfer(self.amount, &mut stacks[self.from - 1], &mut mid);
    transfer(self.amount, &mut mid, &mut stacks[self.to - 1]);
  }
}

fn parse_stack(lines: &[&str]) -> Stacks {
  (0..9)
    .map(|i| lines.iter().filter_map(
      |line| match line.chars().nth(1 + 4*i) {
        Some(' ') => None,
        x => x
      }
    ).rev().collect::<Stack>())
    .collect::<Stacks>()
}

fn parse_moves(lines: &[&str]) -> Vec<Move> {
  let re = Regex::new(r"move ([0-9]+) from ([0-9]+) to ([0-9]+)").unwrap();
  lines.iter().filter_map(|line| {
    match re.captures(line) {
      Some(cap) =>
        Some(Move {
          amount: cap[1].parse().unwrap(),
          from: cap[2].parse().unwrap(),
          to: cap[3].parse().unwrap(),
        }),
      None => None
    }
  }).collect()
}

fn parse_input() -> Input {
  let binding = fs::read_to_string("input/day5.txt")
    .expect("File input/day5.txt does not exist.");
  let lines = binding
    .split("\n")
    .collect::<Vec<&str>>();
  Input {
    stacks: parse_stack(&lines[0..8]),
    moves: parse_moves(&lines[10..]),
  }
}

fn simulate(input: &Input, new: bool) -> Stacks {
  let mut new_stacks: Stacks = input.stacks.iter().map(|s| s.clone()).collect();
  for m in &input.moves {
    if new {
      m.new_apply(&mut new_stacks);
    } else {
      m.apply(&mut new_stacks);
    }
  }
  new_stacks
}

fn printstack(stacks: &Stacks) {
  let msg: String = stacks.iter().filter_map(|s| s.last()).collect();
  println!("{}", msg);
}

fn main() {
  let input = parse_input();
  let sim1 = simulate(&input, false);
  printstack(&sim1);
  let sim2 = simulate(&input, true);
  printstack(&sim2);
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_parse_stack() {
    assert_eq!(parse_stack(&[
      "[V]     [B]                     [F]",
      "[N] [Q] [W]                 [R] [B]",
      "[F] [D] [S]     [B]         [L] [P]",
      "[S] [J] [C] [X] [F] [C] [I] [D] [G]",
    ]), vec![
      vec!['S', 'F', 'N', 'V'],
      vec!['J', 'D', 'Q'],
      vec!['C', 'S', 'W', 'B'],
      vec!['X'],
      vec!['F', 'B'],
      vec!['C'],
      vec!['I'],
      vec!['D', 'L', 'R'],
      vec!['G', 'P', 'B', 'F'],
    ])
  }

  #[test]
  fn test_parse_moves() {
    assert_eq!(parse_moves(&[
      "move 1 from 8 to 4",
      "move 4 from 7 to 8",
    ]), vec![
      Move { amount: 1, from: 8, to: 4 },
      Move { amount: 4, from: 7, to: 8 },
    ])
  }

  #[test]
  fn test_simulation() {
    assert_eq!(simulate(
      &Input {
        stacks: vec![
          vec!['C', 'B', 'A'],
          vec!['F', 'E', 'D'],
        ],
        moves: vec![
          Move { amount: 2, from: 1, to: 2 },
        ]
      }, false
    ), vec![
      vec!['C'],
      vec!['F', 'E', 'D', 'A', 'B'],
    ])
  }
}
