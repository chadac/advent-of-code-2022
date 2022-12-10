mod input;
use input::INPUT;

use std::ops::{Add, Sub};
use std::collections::HashSet;

#[derive(Debug)]
enum Dir {
  U,
  D,
  L,
  R,
}

#[derive(Debug)]
struct Move {
  direction: Dir,
  amount: u32,
}

#[derive(Debug, Eq, PartialEq, Copy, Clone, Hash)]
struct Coord {
  x: i32,
  y: i32,
}

impl Add for Coord {
  type Output = Self;

  fn add(self, other: Self) -> Self {
    Coord { x: self.x + other.x, y: self.y + other.y }
  }
}

impl Sub for Coord {
  type Output = Self;

  fn sub(self, other: Self) -> Self {
    Coord { x: self.x - other.x, y: self.y - other.y }
  }
}

#[derive(Debug, Eq, PartialEq)]
struct State {
  head: Coord,
  tail: Vec<Coord>,
}

impl State {
  fn last(&self) -> &Coord {
    self.tail.last().unwrap()
  }
}

fn delta(direction: &Dir) -> Coord {
  match direction {
    Dir::U => Coord { x: 0, y: -1 },
    Dir::D => Coord { x: 0, y: 1 },
    Dir::L => Coord { x: -1, y: 0 },
    Dir::R => Coord { x: 1, y: 0 },
  }
}

fn follow(new_head: Coord, tail: Coord) -> Coord {
  let dist = new_head - tail;
  let m = dist.x.abs() + dist.y.abs();
  let dt = match m {
    0 => Coord { x: 0, y: 0},
    1 => Coord { x: 0, y: 0},
    2 => {
      match (dist.x.abs(), dist.y.abs()) {
        (1, 1) => Coord { x: 0, y: 0 },
        _ => Coord { x: dist.x.signum(), y: dist.y.signum() },
      }
    },
    3 => {
      Coord { x: dist.x.signum(), y: dist.y.signum() }
    },
    4 => {
      Coord { x: dist.x.signum(), y: dist.y.signum() }
    },
    5 => {
      Coord { x: 2*dist.x.signum(), y: 2*dist.y.signum() }
    },
    6 => {
      Coord { x: 2*dist.x.signum(), y: 2*dist.y.signum() }
    },
    _ => {
      println!("{:?} {:?}", new_head, tail);
      panic!("You can't move this far!")
    }
  };
  tail + dt
}

fn simulate(moves: &Vec<Move>, count: usize) -> Vec<State> {
  let init: State = State {
    head: Coord { x: 0, y: 0 },
    tail: vec![Coord { x: 0, y: 0 }; count],
  };
  let mut states: Vec<State> = vec![init];
  let mut current = &states[0];
  let mut new = &states[0].head;
  for m in moves.iter() {
    // println!("{:?}", m);
    let dh = delta(&m.direction);
    for _ in 0..m.amount {
      let new_head = current.head + dh;
      let mut new_tail: Vec<Coord> = Vec::new();
      new = &new_head;
      for (idx, knot) in current.tail.iter().enumerate() {
        let new_knot = follow(*new, *knot);
        // println!("{} {:?} {:?} {:?}", idx, knot, new_knot, *new);
        new_tail.push(new_knot);
        new = &new_tail.last().unwrap();
      }
      states.push(State {
        head: new_head,
        tail: new_tail,
      });
      current = &states.last().unwrap();
      // println!("{:?}", current);
    }
  }
  return states;
}

fn part1(moves: &Vec<Move>) {
  let states = simulate(moves, 1);
  // println!("{:?}", states[0]);
  // println!("{:?}", states.last());
  let mut tailp = states.iter().map(|s| *s.last()).collect::<HashSet<Coord>>();
  // println!("{:?}", states[5]);
  println!("Part 1: {}", tailp.len());
}

fn part2(moves: &Vec<Move>) {
  let states = simulate(&moves, 9);
  let mut tailp = states.iter().map(|s| *s.last()).collect::<HashSet<Coord>>();
  println!("Part 2: {}", tailp.len());
}

fn parse_input() -> Vec<Move> {
  INPUT[1..(INPUT.len()-1)]
    .split("\n")
    .map(|line| {
      let s: Vec<&str> = line.split(" ").collect();
      match &s[..] {
        ["U", amt] => Move { direction: Dir::U, amount: amt.parse().unwrap() },
        ["D", amt] => Move { direction: Dir::D, amount: amt.parse().unwrap() },
        ["L", amt] => Move { direction: Dir::L, amount: amt.parse().unwrap() },
        ["R", amt] => Move { direction: Dir::R, amount: amt.parse().unwrap() },
        _ => panic!("Unrecognized line."),
      }
    })
    .collect::<Vec<Move>>()
}

fn main() {
  let moves = parse_input();
  part1(&moves);
  part2(&moves);
}
