use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;

#[aoc_generator(day1)]
fn generate(input: &[u8]) -> Vec<Option<u32>> {
    input
        .split(|&c| c == b'\n')
        .map(|line| (!line.is_empty()).then_some(make_num(line)))
        .collect()
}

#[aoc(day1, part1)]
fn part1(input: &[Option<u32>]) -> u32 {
    input
        .split(|opt| opt.is_none())
        .map(|entry| entry.iter().map(|o| o.unwrap()).sum())
        .max()
        .unwrap()
}

#[aoc(day1, part2)]
fn part2(input: &[Option<u32>]) -> u32 {
    input
        .split(|opt| opt.is_none())
        .map(|entry| entry.iter().map(|o| o.unwrap()).sum::<u32>())
        .sorted()
        .rev()
        .take(3)
        .sum()
}

fn make_num(nums: &[u8]) -> u32 {
    let mut sum = 0;
    for (index, num) in nums.iter().enumerate() {
        let factor = 10_u32.pow((nums.len() - index - 1) as u32);
        sum += (num - 48) as u32 * factor;
    }
    sum
}
