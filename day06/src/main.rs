fn get_input() -> Vec<u8> {
    std::fs::read_to_string("input/input.txt").unwrap().into()
}

fn find_marker(input: &[u8], n: usize) -> usize {
    input
        .windows(n)
        .position(|v| {
            let mut set = std::collections::HashSet::new();
            v.iter().all(|c| set.insert(c))
        })
        .map(|i| i + n)
        .unwrap()
}

fn main() {
    let input = get_input();
    println!("part1: {}", find_marker(&input, 4));
    println!("part2: {}", find_marker(&input, 14));
}
