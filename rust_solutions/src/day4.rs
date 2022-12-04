use aoc_runner_derive::aoc;

#[aoc(day4, part1)]
fn part1(input: &[u8]) -> usize {
    input
        .split(|&c| c == b'\n')
        .filter(|line| {
            let mut iter = line.splitn(2, |&c| c == b',').map(get_range);
            let a = iter.next().unwrap();
            let b = iter.next().unwrap();
            contains(&a, &b)
        })
        .count()
}

#[aoc(day4, part2)]
fn part2(input: &[u8]) -> usize {
    input
        .split(|&c| c == b'\n')
        .filter(|line| {
            let mut iter = line.splitn(2, |&c| c == b',').map(get_range);
            let a = iter.next().unwrap();
            let b = iter.next().unwrap();
            overlaps(&a, &b)
        })
        .count()
}

fn get_range(slice: &[u8]) -> (u32, u32) {
    let mut iter = slice.splitn(2, |&c| c == 45).map(|chunk| make_num(chunk));
    (iter.next().unwrap(), iter.next().unwrap())
}

fn make_num(nums: &[u8]) -> u32 {
    let mut sum = 0;
    for (index, num) in nums.iter().enumerate() {
        let factor = 10_u32.pow((nums.len() - index - 1) as u32);
        sum += (num - 48) as u32 * factor;
    }
    sum
}

fn contains(a: &(u32, u32), b: &(u32, u32)) -> bool {
    let c = a.1 >= b.1 && a.0 <= b.0;
    let d = b.1 >= a.1 && b.0 <= a.0;
    c || d
}

fn overlaps(a: &(u32, u32), b: &(u32, u32)) -> bool {
    let c = a.1 >= b.0 && a.0 <= b.1;
    let d = b.1 >= a.0 && b.0 <= a.1;
    c || d
}
