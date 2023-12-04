fn get_input() -> Vec<(char, i64)> {
    std::fs::read_to_string("input/input.txt")
        .unwrap()
        .lines()
        .map(|s| s.split_once(' ').unwrap())
        .map(|(a, b)| (a.chars().next().unwrap(), b.parse().unwrap()))
        .collect()
}

fn part1(input: &[(char, i64)]) -> usize {
    let mut visited = std::collections::HashSet::new();
    let mut head = (0i64, 0i64);
    let mut tail = (0i64, 0i64);
    visited.insert(tail);
    for c in input {
        let d = match c.0 {
            'R' => (1, 0),
            'L' => (-1, 0),
            'U' => (0, 1),
            'D' => (0, -1),
            _ => panic!(),
        };
        for _ in 0..c.1 {
            head = (head.0 + d.0, head.1 + d.1);
            if (head.0 - tail.0).abs() > 1 || (head.1 - tail.1).abs() > 1 {
                tail = (head.0 - d.0, head.1 - d.1);
                visited.insert(tail);
            }
        }
    }
    visited.len()
}

fn part2(input: &[(char, i64)], rope_len: usize) -> usize {
    let mut visited = std::collections::HashSet::new();
    let mut rope = vec![(0i64, 0i64); rope_len];
    visited.insert(*rope.last().unwrap());
    for c in input {
        let d = match c.0 {
            'R' => (1, 0),
            'L' => (-1, 0),
            'U' => (0, 1),
            'D' => (0, -1),
            _ => panic!(),
        };
        for _ in 0..c.1 {
            rope[0] = (rope[0].0 + d.0, rope[0].1 + d.1);
            for i in 1..rope.len() {
                let (x_diff, y_diff) = (rope[i - 1].0 - rope[i].0, rope[i - 1].1 - rope[i].1);
                if x_diff.abs() > 1 {
                    rope[i] = (rope[i - 1].0 - x_diff.signum(), rope[i - 1].1);
                }
                if y_diff.abs() > 1 {
                    rope[i] = (rope[i - 1].0, rope[i - 1].1 - y_diff.signum());
                }
                if x_diff.abs() > 1 && y_diff.abs() > 1 {
                    rope[i] = (
                        rope[i - 1].0 - x_diff.signum(),
                        rope[i - 1].1 - y_diff.signum(),
                    );
                }
            }
            visited.insert(*rope.last().unwrap());
        }
    }
    println!("{visited:?}");
    visited.len()
}

fn main() {
    let input = get_input();
    println!("part1: {}", part1(&input, 10));
    println!("part2: {}", part2(&input, 10));
}

// .map(|(a, b)| match a {
//     "R" => (b.parse().unwrap(), 0),
//     "L" => (-b.parse().unwrap(), 0),
//     "U" => (0, b.parse().unwrap()),
//     "D" => (0, -b.parse().unwrap()),
//     _ => panic!(),
// })

// head = (head.0 + x, head.1 + y);
// if (head.0 - tail.0).abs() > 1 || (head.1 - tail.1).abs() > 1  {
//     tail = (tail.0 + x, tail.1 + y);
//     if
// }

// head.0 += x,
// head.0 -= x,
// head.1 += y,
// head.1 -= y,
