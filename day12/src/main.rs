fn get_input() -> Vec<Vec<u8>> {
    std::fs::read_to_string("input/input.txt")
        .unwrap()
        .lines()
        .map(|s| s.bytes().collect())
        .collect()
}

fn part1(input: &[Vec<u8>]) -> i64 {
    let mut steps_taken = vec![vec![None; input[0].len()]; input.len()];
    let mut to_visit = std::collections::VecDeque::new();
    for j in 0..input.len() {
        for i in 0..input[0].len() {
            if input[j][i] == b'S' {
                to_visit.push_back((i, j));
                steps_taken[j][i] = Some(0);
            }
        }
    }

    while let Some((x,y)) = to_visit.pop_front() {
        let current_height = match input[y][x] {
            b'S' => b'a',
            x => x
        };
        let current_steps = steps_taken[y][x].unwrap();
        for (i,j) in [(x as i64 + 1,y as i64),(x as i64 - 1,y as i64),(x as i64,y as i64 + 1),(x as i64,y as i64 - 1)] {
            if i < 0 || j < 0 || i >= input[0].len() as i64 || j >= input.len() as i64 {
                continue
            }
            let i = i as usize;
            let j = j as usize;

            if let None = steps_taken[j][i] {
                // println!("(i,j): ({i},{j}) current_height: {current_height}");
                if input[j][i] as i64 == b'E' as i64 {
                    if current_height as i64 + 1 >= b'z' as i64 {
                        return current_steps + 1
                    }
                } else if current_height as i64 + 1 >= input[j][i] as i64 {
                    steps_taken[j][i] = Some(current_steps + 1);
                    to_visit.push_back((i,j));
                }
            }
        }
    }
    // for j in 0..input.len() {
    //     for i in 0..input[0].len() {
    //         print!("{:?}\t", steps_taken[j][i]);
    //     }
    //     println!();
    // }
    panic!()
}

fn part2(input: &[Vec<u8>]) -> i64 {
    let mut steps_taken = vec![vec![None; input[0].len()]; input.len()];
    let mut to_visit = std::collections::VecDeque::new();
    for j in 0..input.len() {
        for i in 0..input[0].len() {
            if input[j][i] == b'S' || input[j][i] == b'a' {
                to_visit.push_back((i, j));
                steps_taken[j][i] = Some(0);
            }
        }
    }
    while let Some((x,y)) = to_visit.pop_front() {
        let current_height = match input[y][x] {
            b'S' => b'a',
            x => x
        };
        let current_steps = steps_taken[y][x].unwrap();
        for (i,j) in [(x as i64 + 1,y as i64),(x as i64 - 1,y as i64),(x as i64,y as i64 + 1),(x as i64,y as i64 - 1)] {
            if i < 0 || j < 0 || i >= input[0].len() as i64 || j >= input.len() as i64 {
                continue
            }
            let i = i as usize;
            let j = j as usize;

            if let None = steps_taken[j][i] {
                // println!("(i,j): ({i},{j}) current_height: {current_height}");
                if input[j][i] as i64 == b'E' as i64 {
                    if current_height as i64 + 1 >= b'z' as i64 {
                        return current_steps + 1
                    }
                } else if current_height as i64 + 1 >= input[j][i] as i64 {
                    steps_taken[j][i] = Some(current_steps + 1);
                    to_visit.push_back((i,j));
                }
            }
        }
    }
    // for j in 0..input.len() {
    //     for i in 0..input[0].len() {
    //         print!("{:?}\t", steps_taken[j][i]);
    //     }
    //     println!();
    // }
    panic!()
}

fn main() {
    let input = get_input();
    println!("part1: {}", part1(&input));
    println!("part2: {}", part2(&input));
}
