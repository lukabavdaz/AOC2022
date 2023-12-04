use itertools::chain;
use itertools::Itertools;

fn get_input() -> Vec<Vec<i64>> {
    std::fs::read_to_string("input/input.txt")
        .unwrap()
        .lines()
        .map(|l| {
            l.split(|c: char| !c.is_numeric() && c != '-')
                .filter_map(|s| s.parse().ok())
                .collect()
        })
        .collect()
}

fn part1(input: &[Vec<i64>]) -> i64 {
    let (min, max) = input
        .iter()
        .map(|v| {
            (
                v[0],
                (v[0] - v[2]).abs() + (v[1] - v[3]).abs() - (v[1] - 2000000).abs(),
            )
        })
        .filter(|(_, len)| len >= &0)
        .map(|(mid, len)| (mid - len, mid + len))
        .reduce(|acc, i| (acc.0.min(i.0), acc.1.max(i.1)))
        .unwrap();
    let beacons = 1; //input.iter().filter(|v| v[3] == 10).count();
    max - min + 1 - beacons as i64
}

fn edge(x: i64, y: i64, d: i64) -> Vec<(i64, i64)> {
    chain!(
        (x..=x + d + 1).zip(y - d - 1..=y),
        (x..=x + d + 1).rev().zip(y..=y + d + 1),
        (x - d - 1..=x).zip(y..=y + d + 1),
        (x - d - 1..=x).rev().zip(y - d - 1..=y)
    )
    .collect()
}

fn part2(input: &[Vec<i64>]) -> i64 {
    let scanners: Vec<(i64, i64, i64)> = input
        .iter()
        .map(|v| (v[0], v[1], (v[0] - v[2]).abs() + (v[1] - v[3]).abs()))
        .collect();
    scanners
        .iter()
        .flat_map(|(a, b, c)| edge(*a, *b, *c))
        .filter(|(x, y)| x>=&0 && x <= &4000000 && y>=&0 && y <= &4000000)
        .find(|(x, y)| {
            scanners
                .iter()
                .all(|(i, j, d)| (x - i).abs() + (y - j).abs() > *d)
        })
        .map(|(x, y)| x * 4000000 + y)
        .unwrap()
}

fn main() {
    let input = get_input();
    println!("part1: {}", part1(&input));
    println!("part2: {}", part2(&input));
}
