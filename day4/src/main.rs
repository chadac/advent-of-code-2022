use std::fs;
use std::option::Option;
use std::vec::Vec;

struct Range {
    left: i32,
    right: i32,
}

struct Pair {
    first: Range,
    second: Range,
}

impl Range {
    fn contains(&self, other: &Range) -> bool {
        self.left <= other.left && self.right >= other.right
    }

    fn loverlaps(&self, other: &Range) -> bool {
        self.left <= other.left && self.right >= other.left
    }
}

impl Pair {
    fn one_contains(&self) -> bool {
        self.first.contains(&self.second) || self.second.contains(&self.first)
    }

    fn overlaps(&self) -> bool {
        self.first.loverlaps(&self.second) || self.second.loverlaps(&self.first)
    }
}

fn parse_range(range: &str) -> Range {
    let items: Vec<&str> = range.split("-").collect();
    match &items[..] {
        [left, right] =>
            Range {
                left: left.to_string().parse::<i32>().unwrap(),
                right: right.to_string().parse::<i32>().unwrap(),
            },
        _ => panic!("Parse error!"),
    }
}

fn parse_line(line: &str) -> Option<Pair> {
    let items: Vec<&str> = line.split(",").collect();
    match &items[..] {
        [first, second] =>
            Some(Pair {
                first: parse_range(first),
                second: parse_range(second),
            }),
        _ => {
            None
        }
    }
}

fn read_input() -> Vec<Pair> {
    fs::read_to_string("input/day4.txt")
        .expect("File input.csv does not exist.")
        .split("\n")
        .filter_map(parse_line)
        .collect::<Vec<Pair>>()
}

fn part1(input: &Vec<Pair>) {
    let result: i32 = input.iter().map(|pair| pair.one_contains() as i32).sum();
    println!("Part 1: {}", result);
}

fn part2(input: &Vec<Pair>) {
    let result: i32 = input.iter().map(|pair| pair.overlaps() as i32).sum();
    println!("Part 2: {}", result);
}

fn main() {
    println!("################ Day 4 ################");
    let input: Vec<Pair> = read_input();
    part1(&input);
    part2(&input);
}
