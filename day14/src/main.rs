use itertools::Itertools;
use std::collections::HashMap;

fn get_input() -> HashMap<(i64, i64), bool> {
    std::fs::read_to_string("input/input.txt")
        .unwrap()
        .lines()
        .flat_map(|l| {
            l.split(" -> ")
                .map(|s| s.split_once(",").unwrap())
                .map(|(x, y)| (x.parse::<i64>().unwrap(), y.parse::<i64>().unwrap()))
                .tuple_windows()
                .flat_map(|((x1, y1), (x2, y2))| {
                    (x1.min(x2)..=x1.max(x2)).cartesian_product(y1.min(y2)..=y1.max(y2))
                })
                .map(|p| (p, false))
        })
        .collect()
}

fn part1(input: &HashMap<(i64, i64), bool>) -> usize {
    let mut cave = input.clone();
    let last_y = *cave.keys().map(|(_, y)| y).max().unwrap();
    loop {
        let mut s = (500,0);

        while let Some(new_x) = match [s.0, s.0 - 1, s.0 + 1].map(|x| cave.get(&(x, s.1 + 1))) {
            [None, _, _] => Some(s.0),
            [_, None, _] => Some(s.0 - 1),
            [_, _, None] => Some(s.0 + 1),
            _ => None
        } {
            s = (new_x, s.1 + 1);
            // println!("s: {s:?}, last_y: {last_y}");
            if s.1 > last_y {
                return cave.values().filter(|a| **a).count()
            }
        }
        cave.insert(s, true);
    }
}

fn part2(input: &HashMap<(i64, i64), bool>) -> usize {
    let mut cave = input.clone();
    let last_y = *cave.keys().map(|(_, y)| y).max().unwrap();
    'first: while cave.get(&(500,0)).is_none() {
        let mut s = (500,0);
        while let Some(new_x) = match [s.0, s.0 - 1, s.0 + 1].map(|x| cave.get(&(x, s.1 + 1))) {
            [None, _, _] => Some(s.0),
            [_, None, _] => Some(s.0 - 1),
            [_, _, None] => Some(s.0 + 1),
            _ => None
        } {
            s = (new_x, s.1 + 1);
            // println!("s: {s:?}, last_y: {last_y}");
            if s.1 == last_y + 1 {
                cave.insert(s, true);
                continue 'first
            }
        }
        cave.insert(s, true);
    }
    cave.values().filter(|a| **a).count()
}

fn main() {
    let input = get_input();
    println!("part1: {}", part1(&input));
    println!("part2: {}", part2(&input));
}
