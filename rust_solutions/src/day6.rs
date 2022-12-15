use std::iter::{successors, zip};

use aoc_runner_derive::aoc;
use itertools::Itertools;

#[aoc(day6, part1)]
fn part1(input: &str) -> usize {
    let zips = zip(
        zip(input.chars(), input.chars().skip(1)),
        zip(input.chars().skip(2), input.chars().skip(3)),
    );

    zips.take_while(|(a, b)| ![a.0, a.1, b.0, b.1].iter().all_unique())
        .count()
        + 4
}

#[aoc(day6, part1, successors)]
fn part1_successors(input: &str) -> usize {
    let index = successors(Some((&input[0..4], 1_usize)), |&(_, num)| {
        if num + 4 > input.len() {
            None
        } else {
            Some((&input[num..(num + 4)], num + 1))
        }
    })
    .take_while(|&(chars, _)| !chars.chars().all_unique())
    .count()
        + 4;

    index
}

#[aoc(day6, part1, while_loop)]
fn part1_while(input: &str) -> usize {
    for i in 0..input.len() - 4 {
        if input[i..i + 4].chars().all_unique() {
            return i + 4;
        }
    }
    return 0;
}

#[aoc(day6, part2)]
fn part2(input: &str) -> usize {
    let index = successors(Some((&input[0..14], 1_usize)), |&(_, num)| {
        if num + 14 > input.len() {
            None
        } else {
            Some((&input[num..(num + 14)], num + 1))
        }
    })
    .take_while(|&(chars, _)| !chars.chars().all_unique())
    .count()
        + 14;

    index
}

#[aoc(day6, part2, while_loop)]
fn part2_while(input: &str) -> usize {
    for i in 0..input.len() - 14 {
        if input[i..i + 14].chars().all_unique() {
            return i + 14;
        }
    }
    return 0;
}
