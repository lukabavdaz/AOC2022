fn get_input() -> Vec<Vec<i64>> {
    std::fs::read_to_string("input/input.txt")
        .unwrap()
        .lines()
        .map(|l| {
            l.split(|c: char| !c.is_numeric())
                .map(|s| s.parse().unwrap())
                .collect()
        })
        .collect()
}

fn part1(input: &[Vec<i64>]) -> usize {
    input
        .iter()
        .filter(|&i| (i[0] <= i[2] && i[3] <= i[1]) || (i[2] <= i[0] && i[1] <= i[3]))
        .count()
}

fn part2(input: &[Vec<i64>]) -> usize {
    input
        .iter()
        .filter(|&i| (i[0] <= i[2] && i[2] <= i[1]) || (i[2] <= i[0] && i[0] <= i[3]))
        .count()
}

fn main() {
    let input = get_input();
    println!("part1: {}", part1(&input));
    println!("part2: {}", part2(&input));
}
