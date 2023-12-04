fn get_input() -> Vec<Option<i64>> {
    std::fs::read_to_string("input/input.txt")
        .unwrap()
        .lines()
        .map(|s| s.split_once(' ').map(|(_, a)| a.parse().ok()).flatten())
        .collect()
}

fn part1(input: &[Option<i64>]) -> i64 {
    let mut x = 1;
    let mut cycle = 0;
    let mut result = 0;
    for i in input {
        cycle += 1;
        if cycle%40 == 20 {
            result += x * cycle;
        }
        if let Some(a) = i {
            cycle += 1;
            if cycle%40 == 20 {
                result += x * cycle;
            }
            x += a;
        }
        if cycle > 220 {
            break
        }
    }
    result
}

fn part2(input: &[Option<i64>]) -> i64 {
    let mut x = 1;
    let mut cycle = 0;
    let mut result = 0;
    for i in input {
        cycle += 1;
        if cycle%40 > x-2 && cycle%40 < x+2 {
            print!("#");
        } else {
            print!(".");
        }
        if cycle%40 == 0 {
            println!();
        }
        if let Some(a) = i {
            cycle += 1;
            x += a;
            if cycle%40 > x-2 && cycle%40 < x+2 {
                print!("#");
            } else {
                print!(".");
            }
            if cycle%40 == 0 {
                println!();
            }

        }

    }
    result
}

fn main() {
    let input = get_input();
    println!("part1: {}", part1(&input));
    println!("{}",part2(&input));
}
