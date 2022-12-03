use std::collections::HashSet;

use aoc_runner_derive::aoc;
use itertools::Itertools;

#[aoc(day3, part1)]
fn part1(input: &[u8]) -> u32 {
    input
        .split(|&c| c == b'\n')
        .map(|line| {
            let (a, b) = line.split_at(line.len() / 2);
            let a = a.iter().collect::<HashSet<_>>();
            let b = b.iter().collect::<HashSet<_>>();
            let c = **a.intersection(&b).nth(0).unwrap();
            get_priority(c)
        })
        .sum()
}

#[aoc(day3, part2)]
fn part2(input: &[u8]) -> u32 {
    input
        .split(|&c| c == b'\n')
        .chunks(3)
        .into_iter()
        .map(|group| {
            if let [a, b, c] = group.collect::<Vec<_>>()[..] {
                let a = a.iter().collect::<HashSet<_>>();
                let b = b.iter().collect::<HashSet<_>>();
                let c = c.iter().collect::<HashSet<_>>();
                let d = a.intersection(&b).map(|v| *v).collect::<HashSet<_>>();
                let e = d.intersection(&c).nth(0).unwrap();
                get_priority(**e)
            } else {
                0
            }
        })
        .sum()
}

fn get_priority(num: u8) -> u32 {
    if num.is_ascii_uppercase() {
        (num - 38) as u32
    } else {
        (num - 96) as u32
    }
}
