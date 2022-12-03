use aoc_runner_derive::{aoc, aoc_generator};

const MY_WINNING: [(u8, u8); 3] = [(65, 89), (66, 90), (67, 88)];
const MY_LOSS: [(u8, u8); 3] = [(65, 90), (66, 88), (67, 89)];

#[aoc_generator(day2)]
fn generate(input: &[u8]) -> Vec<(u8, u8)> {
    input
        .split(|&c| c == b'\n')
        .map(|bytes| (bytes[0], bytes[2]))
        .collect()
}

#[aoc(day2, part1)]
fn part1(input: &[(u8, u8)]) -> u32 {
    input.iter().map(|&slice| get_score(&slice) as u32).sum()
}

#[aoc(day2, part2)]
fn part2(input: &[(u8, u8)]) -> u32 {
    input
        .iter()
        .map(|&slice| get_right_score(&slice) as u32)
        .sum()
}

fn get_score(&(a, b): &(u8, u8)) -> u8 {
    let mut score = b - 87;
    if MY_WINNING.contains(&(a, b)) {
        score += 6;
    } else if !MY_LOSS.contains(&(a, b)) {
        score += 3;
    }
    score
}

fn get_right_score(&(a, b): &(u8, u8)) -> u8 {
    let mut score = 0;
    if b == 90 {
        score += 6;
        let temp = if a == 67 { 65 } else { a + 1 };
        score += temp - 64;
    } else if b == 89 {
        score += 3;
        score += a - 64;
    } else {
        let temp = if a == 65 { 67 } else { a - 1 };
        score += temp - 64;
    };
    score
}
