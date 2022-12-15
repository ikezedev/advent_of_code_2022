use aoc_runner_derive::aoc;
use itertools::Itertools;
use std::str::FromStr;

#[derive(Debug, Default)]
struct Move {
    from: usize,
    to: usize,
    by: usize,
}

// struct Day5 {
//     map: Vec<Vec<char>>,
//     steps: Vec<Move>,
// }

impl FromStr for Move {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut iter = s.split_whitespace();
        if let (Some(a), Some(b), Some(c)) = (iter.nth(1), iter.nth(1), iter.nth(1)) {
            Ok(Self {
                from: b
                    .parse()
                    .map_err(|_| format!("expected num for move from but got {b}"))?,
                to: c
                    .parse()
                    .map_err(|_| format!("expected num for move to but got {c}"))?,
                by: a
                    .parse()
                    .map_err(|_| format!("expected num for move by but got {a}"))?,
            })
        } else {
            Err(format!("invalid row: {s}"))
        }
    }
}

#[aoc(day5, part1)]
fn part1(input: &str) -> Option<String> {
    if let Some((stacks, steps)) = input.split_once("\n\n") {
        let mut stack_lines = stacks.lines().rev();
        let key_count = stack_lines
            .nth(0)
            .unwrap()
            .trim()
            .split_whitespace()
            .count();
        let mut map = Vec::<Vec<char>>::with_capacity(key_count);
        stack_lines.for_each(|line| {
            line.chars()
                .chunks(4)
                .into_iter()
                .map(|mut chunks| chunks.nth(1).unwrap())
                .enumerate()
                .for_each(|(c, i)| {
                    if !i.is_whitespace() {
                        map.get_mut(c)
                            .map(|v| v.push(i))
                            .or_else(|| Some(map.insert(c, vec![i])));
                    }
                });
        });
        let steps = steps
            .trim()
            .lines()
            .map(Move::from_str)
            .collect::<Result<Vec<_>, _>>()
            .unwrap();
        return solve(&mut map, steps);
    };
    return None;
}

#[aoc(day5, part2)]
fn part2(input: &str) -> Option<String> {
    if let Some((stacks, steps)) = input.split_once("\n\n") {
        let mut stack_lines = stacks.lines().rev();
        let key_count = stack_lines
            .nth(0)
            .unwrap()
            .trim()
            .split_whitespace()
            .count();
        let mut map = Vec::<Vec<char>>::with_capacity(key_count);
        stack_lines.for_each(|line| {
            line.chars()
                .chunks(4)
                .into_iter()
                .map(|mut chunks| chunks.nth(1).unwrap())
                .enumerate()
                .for_each(|(c, i)| {
                    if !i.is_whitespace() {
                        map.get_mut(c)
                            .map(|v| v.push(i))
                            .or_else(|| Some(map.insert(c, vec![i])));
                    }
                });
        });
        let steps = steps
            .trim()
            .lines()
            .map(Move::from_str)
            .collect::<Result<Vec<_>, _>>()
            .unwrap();
        return solve2(&mut map, steps);
    };
    return None;
}

fn solve(map: &mut Vec<Vec<char>>, steps: Vec<Move>) -> Option<String> {
    steps.iter().for_each(|&Move { by, to, from }| {
        let len = map[from - 1].len() - by;
        let to_move = map[from - 1].split_off(len);
        map[to - 1].extend(to_move.iter().rev());
    });
    map.iter().map(|vec| vec.last()).collect::<Option<String>>()
}

fn solve2(map: &mut Vec<Vec<char>>, steps: Vec<Move>) -> Option<String> {
    steps.iter().for_each(|&Move { by, to, from }| {
        let len = map[from - 1].len() - by;
        let to_move = map[from - 1].split_off(len);
        map[to - 1].extend(to_move.iter());
    });
    map.iter().map(|vec| vec.last()).collect::<Option<String>>()
}
