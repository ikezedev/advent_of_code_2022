use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day8)]
fn generate(input: &[u8]) -> Vec<Vec<u8>> {
    input
        .split(|&c| c == b'\n')
        .map(|line| line.into_iter().map(|num| num - 48).collect())
        .collect()
}

#[aoc(day8, part1)]
fn part1(input: &[Vec<u8>]) -> usize {
    let len_x = input.len();
    let mut visible = len_x * 2 + (input[0].len() - 2) * 2;
    for row in 1..len_x - 1 {
        let len_y = input[row].len();
        for col in 1..len_y - 1 {
            if is_visible((row, col), input) {
                visible += 1;
            }
        }
    }
    visible
}

#[aoc(day8, part2)]
fn part2(input: &[Vec<u8>]) -> usize {
    input
        .iter()
        .enumerate()
        .flat_map(|(row, entries)| {
            entries
                .iter()
                .enumerate()
                .map(move |(col, _)| visible_trees((row, col), input))
        })
        .max()
        .unwrap()
}

fn is_visible((row, col): (usize, usize), input: &[Vec<u8>]) -> bool {
    let current = input[row][col];
    let right = input[row][col + 1..].iter().any(|&e| e >= current);
    if !right {
        return true;
    }
    let left = input[row][0..col].iter().any(|&e| e >= current);
    if !left {
        return true;
    }
    let down = input[row + 1..]
        .iter()
        .map(|r| r[col])
        .any(|e| e >= current);
    if !down {
        return true;
    }

    let up = input[0..row].iter().map(|r| r[col]).any(|e| e >= current);
    if !up {
        return true;
    }
    return false;
}

fn visible_trees((row, col): (usize, usize), input: &[Vec<u8>]) -> usize {
    let current = input[row][col];

    let right = {
        let entries = &input[row][col + 1..];
        let count = entries.iter().take_while(|&&e| e < current).count();
        if count < entries.len() {
            count + 1
        } else {
            count
        }
    };
    let left = {
        let entries = &input[row][0..col];
        let count = entries.iter().rev().take_while(|&&e| e < current).count();
        if count < entries.len() {
            count + 1
        } else {
            count
        }
    };
    let down = {
        let entries = &input[row + 1..];
        let count = entries
            .iter()
            .map(|r| r[col])
            .take_while(|&e| e < current)
            .count();
        if count < entries.len() {
            count + 1
        } else {
            count
        }
    };
    let up = {
        let entries = &input[0..row];
        let count = entries
            .iter()
            .map(|r| r[col])
            .rev()
            .take_while(|&e| e < current)
            .count();
        if count < entries.len() {
            count + 1
        } else {
            count
        }
    };

    return right * left * up * down;
}
