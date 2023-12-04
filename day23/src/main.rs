use std::collections::{HashMap, HashSet};

fn get_input() -> HashSet<(i64, i64)> {
    std::fs::read_to_string("input/input.txt")
        .unwrap()
        .lines()
        .enumerate()
        .flat_map(|(j, l)| {
            l.chars().enumerate().filter_map(move |(i, c)| {
                if c == '#' {
                    Some((i as i64, j as i64))
                } else {
                    None
                }
            })
        })
        .collect()
}

fn get_position(&(x, y): &(i64, i64), positions: &HashSet<(i64, i64)>, i: usize) -> (i64, i64) {
    if (y-1..=y+1).flat_map(|j| (x-1..=x+1).map(move |i| (i,j))).filter(|&(i,j)| i != x || j != y ).all(|p| !positions.contains(&p)) {
        return (x, y)
    }

    [
        ([(x - 1, y - 1), (x, y - 1), (x + 1, y - 1)], (x, y - 1)),
        ([(x - 1, y + 1), (x, y + 1), (x + 1, y + 1)], (x, y + 1)),
        ([(x - 1, y - 1), (x - 1, y), (x - 1, y + 1)], (x - 1, y)),
        ([(x + 1, y - 1), (x + 1, y), (x + 1, y + 1)], (x + 1, y)),
    ]
    .iter()
    .cycle()
    .skip(i)
    .take(4)
    .find(|(a, _)| a.iter().all(|p| !positions.contains(p)))
    .map(|&(_, p)| p)
    .unwrap_or((x, y))
}

// fn part1(input: &HashSet<(i64, i64)>) -> i64 {
//     let mut positions = input.clone();
//     for i in 0..10 {
//         // println!("positions: {:?}", positions);
//         let mut occurrences = HashMap::new();
//         for new_p in positions.iter().map(|p| get_position(p, &positions, i)) {
//             *occurrences.entry(new_p).or_insert(0) += 1;
//         }
//         // println!("occurrences: {:?}", occurrences);
//         positions = positions
//             .iter()
//             .map(|p| {
//                 let new_p = get_position(p, &positions, i);
//                 if occurrences.get(&new_p).unwrap() > &1 {
//                     *p
//                 } else {
//                     new_p
//                 }
//             })
//             .collect();
//     }
//
//     let x_min = *positions.iter().map(|(x, _)| x).min().unwrap();
//     let x_max = *positions.iter().map(|(x, _)| x).max().unwrap();
//     let y_min = *positions.iter().map(|(_, y)| y).min().unwrap();
//     let y_max = *positions.iter().map(|(_, y)| y).max().unwrap();
//
//     for y in y_min..=y_max {
//         for x in x_min..=x_max {
//             if positions.contains(&(x,y)) {
//                 print!("#");
//             } else {
//                 print!(".");
//             }
//         }
//         println!();
//     }
//
//     (x_max + 1 - x_min) * (y_max + 1 - y_min) - input.len() as i64
// }

fn part1(input: &HashSet<(i64, i64)>) -> i64 {
    let mut positions = input.clone();
    for i in 0.. {
        // println!("positions: {:?}", positions);
        let mut occurrences = HashMap::new();
        for new_p in positions.iter().map(|p| get_position(p, &positions, i)) {
            *occurrences.entry(new_p).or_insert(0) += 1;
        }
        // println!("occurrences: {:?}", occurrences);
        positions = positions
            .iter()
            .map(|p| {
                let new_p = get_position(p, &positions, i);
                if occurrences.get(&new_p).unwrap() > &1 {
                    *p
                } else {
                    new_p
                }
            })
            .collect();
    }

    let x_min = *positions.iter().map(|(x, _)| x).min().unwrap();
    let x_max = *positions.iter().map(|(x, _)| x).max().unwrap();
    let y_min = *positions.iter().map(|(_, y)| y).min().unwrap();
    let y_max = *positions.iter().map(|(_, y)| y).max().unwrap();

    for y in y_min..=y_max {
        for x in x_min..=x_max {
            if positions.contains(&(x,y)) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }

    (x_max + 1 - x_min) * (y_max + 1 - y_min) - input.len() as i64
}

fn part2(input: &HashSet<(i64, i64)>) -> usize {
    let mut positions = input.clone();
    for i in 0.. {
        // println!("positions: {:?}", positions);
        let mut occurrences = HashMap::new();
        for new_p in positions.iter().map(|p| get_position(p, &positions, i)) {
            *occurrences.entry(new_p).or_insert(0) += 1;
        }
        // println!("occurrences: {:?}", occurrences);
        let new_positions = positions
            .iter()
            .map(|p| {
                let new_p = get_position(p, &positions, i);
                if occurrences.get(&new_p).unwrap() > &1 {
                    *p
                } else {
                    new_p
                }
            })
            .collect();
        if new_positions == positions {
            return i + 1
        } else {
            positions = new_positions;
        }
    }
    panic!()
}

fn main() {
    let input = get_input();
    // println!("input: {:?}", input.len());
    // println!("part1: {}", part1(&input));
    println!("part2: {}", part2(&input));
}

// ...#..#..#.
// .....#.#...
// ..#...#...#
// ..#..#.#...
// #...#.#.#..
// ...........
// .#.#.#.##..
// ...........
// ...#..#....
//
// ......#....
// ...#.....#.
// ..#..#.#...
// ......#...#
// ..#..#.#...
// #...#.#.#..
// ...........
// .#.#.#.##..
// ...#..#....