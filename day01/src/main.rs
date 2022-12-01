fn get_input() -> Vec<Vec<i64>> {
    std::fs::read_to_string("input/input.txt")
        .unwrap()
        .split("\n\n")
        .map(|s| s.lines().map(|l| l.parse().unwrap()).collect())
        .collect()
}

fn part1(input: &[Vec<i64>]) -> i64 {
    input.iter().map(|l| l.iter().sum()).max().unwrap()
}

fn part2(input: &[Vec<i64>]) -> i64 {
    let mut totals = input.iter().map(|l| l.iter().sum()).collect::<Vec<_>>();
    totals.sort();
    totals.iter().rev().take(3).sum()
}

fn main() {
    let input = get_input();
    println!("part1: {}", part1(&input));
    println!("part2: {}", part2(&input));
}
