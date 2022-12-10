mod input;
use input::INPUT;

#[derive(Debug)]
enum Instr {
  NOOP,
  ADDX { num: i32 },
}

fn parse_input(input: &str) -> Vec<Instr> {
  input.split("\n")
    .map(|line| {
      let cmd = line.split(" ").collect::<Vec<&str>>();
      match &cmd[..] {
        ["noop"] => Instr::NOOP,
        ["addx", amt] => Instr::ADDX { num: amt.parse().unwrap() },
        _ => panic!("Unrecognized instruction."),
      }
    })
    .collect::<Vec<Instr>>()
}

fn simx(cmds: &Vec<Instr>) -> Vec<i32> {
  let mut x: Vec<i32> = vec![1];
  for cmd in cmds {
    let c = *x.last().unwrap();
    match cmd {
      Instr::NOOP => {
        x.push(c);
      },
      Instr::ADDX { num } => {
        x.push(c);
        x.push(c + num);
      },
    }
  }
  x
}

fn part1(cmds: &Vec<Instr>) -> i32 {
  let sim = simx(cmds).into_iter().enumerate().collect::<Vec<(usize, i32)>>();
  let idx = &[20, 60, 100, 140, 180, 220]
    .map(|i| sim[i - 1]);
  let values = idx
    .iter()
    .map(|(i, x)| (*i as i32 + 1)*(*x));
  values.sum()
}

fn part2(cmds: &Vec<Instr>) -> String {
  let sim = simx(cmds).into_iter().enumerate()
    .map(|(i, x)| (i as i32 + 1, x))
    .collect::<Vec<(i32, i32)>>();
  let screen: String = sim.into_iter().flat_map(|(cycle, sprite_loc)| {
    let pixel = (cycle - 1) % 40;
    let c = if pixel >= sprite_loc - 1 && pixel <= sprite_loc + 1 {
      '#'
    } else {
      '.'
    };
    match pixel {
      39 => vec![c, '\n'],
      _ => vec![c],
    }
  }).collect();
  (&screen[0..245]).to_string()
}

fn main() {
  let cmds = parse_input(INPUT);
  println!("Part 1: {}", part1(&cmds));
  println!("Part 2:\n{}", part2(&cmds));
}

#[cfg(test)]
mod tests {
  use super::*;
  use input::{SAMPLE, SAMPLE_DISPLAY};

  #[test]
  fn test_sim() {
    assert_eq!(
      simx(&vec![
        Instr::NOOP,
        Instr::ADDX { num: 3 },
        Instr::ADDX { num: -5 },
      ]),
      vec![1, 1, 1, 4, 4, -1]
    );
  }

  #[test]
  fn test_part1_sample() {
    assert_eq!(
      part1(&parse_input(SAMPLE)),
      13140
    )
  }

  #[test]
  fn test_part2_sample() {
    assert_eq!(
      part2(&parse_input(SAMPLE)),
      SAMPLE_DISPLAY
    );
  }
}
