use std::collections::HashSet;

fn get_input() -> Vec<Vec<u8>> {
    std::fs::read_to_string("input/input.txt")
        .unwrap()
        .lines()
        .map(|l| l.into())
        .collect()
}

fn common<'a, T>(input: T) -> i64
where
    T: IntoIterator,
    <T as IntoIterator>::Item: IntoIterator<Item = &'a u8>,
{
    let sets: Vec<_> = input
        .into_iter()
        .map(|v| HashSet::<&u8>::from_iter(v))
        .collect();
    sets[0]
        .iter()
        .find(|&&k| sets.iter().all(|s| s.contains(k)))
        .map(|&&i| match i > 96 {
            true => i as i64 - 96,
            false => i as i64 - 64 + 26,
        })
        .unwrap()
}

fn part1(input: &[Vec<u8>]) -> i64 {
    input.iter().map(|v| common(v.chunks(v.len() / 2))).sum()
}

fn part2(input: &[Vec<u8>]) -> i64 {
    input.chunks(3).map(common).sum()
}

fn main() {
    let input = get_input();
    println!("part1: {}", part1(&input));
    println!("part2: {}", part2(&input));
}
