fn get_input() -> (Vec<Vec<char>>, Vec<Vec<usize>>) {
    let a = vec![
        vec!['W','P','G','Z','V','S','B'],
        vec!['F','Z','C','B','V','J'],
        vec!['C','D','Z','N','H','M','L','V'],
        vec!['B','J','F','P','Z','M','D','L'],
        vec!['H','Q','B','J','G','C','F','V'],
        vec!['B','L','S','T','Q','F','G'],
        vec!['V','Z','C','G','L'],
        vec!['G','L','N'],
        vec!['C','H','F','J']];
    let b = std::fs::read_to_string("input/input.txt")
        .unwrap()
        .lines()
        .skip(10)
        .map(|l| {
            l.split(|c: char| !c.is_numeric())
                .filter(|s| !s.is_empty())
                .map(|s| s.parse().unwrap())
                .collect()
        })
        .collect();
    (a,b)
}

fn part1((crates,commands): &(Vec<Vec<char>>, Vec<Vec<usize>>)) {
    let mut stacks: Vec<Vec<_>> = crates.iter().map(|v| v.iter().cloned().rev().collect()).collect();

    for c in commands {
        for _ in 0..c[0] {
            let popped = stacks[c[1] - 1].pop().unwrap();
            stacks[c[2] - 1].push(popped);
        }
    }
    for i in stacks {
        print!("{}", i.last().unwrap());
    }
}

fn part2((crates,commands): &(Vec<Vec<char>>, Vec<Vec<usize>>)) {
    let mut stacks: Vec<Vec<_>> = crates.iter().map(|v| v.iter().cloned().rev().collect()).collect();

    for c in commands {
        let new_length = stacks[c[1] - 1].len()-c[0];
        let popped = stacks[c[1] - 1].split_off(new_length);
        stacks[c[2] - 1].extend(popped);
    }
    for i in stacks {
        print!("{}", i.last().unwrap());
    }
}

fn main() {
    let input = get_input();
    part1(&input);
    println!();
    part2(&input);
}