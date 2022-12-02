fn get_input() -> Vec<(char, char)> {
    std::fs::read_to_string("input/input.txt")
        .unwrap()
        .lines()
        .map(|s| s.split_once(' ').unwrap())
        .map(|(a, b)| (a.parse().unwrap(), b.parse().unwrap()))
        .collect()
}

fn part1(input: &[(char,char)]) -> i64 {
    input.iter().map(|x| match x {
            ('A', 'X') => 3 + 1,
            ('B', 'X') => 0 + 1,
            ('C', 'X') => 6 + 1,
            ('A', 'Y') => 6 + 2,
            ('B', 'Y') => 3 + 2,
            ('C', 'Y') => 0 + 2,
            ('A', 'Z') => 0 + 3,
            ('B', 'Z') => 6 + 3,
            ('C', 'Z') => 3 + 3,
            _ => panic!()
    }).sum()
}

fn part2(input: &[(char,char)]) -> i64 {
    input.iter().map(|x| match x {
            ('A', 'X') => 0 + 3,
            ('B', 'X') => 0 + 1,
            ('C', 'X') => 0 + 2,
            ('A', 'Y') => 3 + 1,
            ('B', 'Y') => 3 + 2,
            ('C', 'Y') => 3 + 3,
            ('A', 'Z') => 6 + 2,
            ('B', 'Z') => 6 + 3,
            ('C', 'Z') => 6 + 1,
            _ => panic!()
    }).sum()
}

fn main() {
    let input = get_input();
    println!("part1: {}", part1(&input));
    println!("part2: {}", part2(&input));
}
