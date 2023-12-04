use std::collections::HashSet;

fn get_input() -> Vec<Vec<[bool;4]>> {
    std::fs::read_to_string("input/input.txt")
        .unwrap()
        .lines()
        .map(|l| l.chars().map(|c| match c {
            '>' => [true, false, false, false],
            'v' => [false, true, false, false],
            '<' => [false, false, true, false],
            '^' => [false, false, false, true],
            '.' => [false; 4],
            _ => [true; 4],
        }).collect())
        .collect()
}

fn part1(input: &Vec<Vec<[bool;4]>>) -> i64 {
    let mut valley = input.clone();
    let mut positions = HashSet::<(i64,i64)>::from([(1,0)]);
    let mut steps = 0;
    while !positions.contains(&(valley[0].len() as i64 - 2, valley.len() as i64 - 1)) {
        steps += 1;
        let old_valley = valley.clone();
        for j in 1..(old_valley.len()-1) {
            for i in 1..(old_valley[0].len() - 1) {
                let [right, down, left, up] = old_valley[j][i];
                valley[j][i.rem_euclid(old_valley[0].len() - 2) + 1][0] = right;
                valley[j.rem_euclid(old_valley.len() - 2) + 1][i][1] = down;
                valley[j][(i as i64 - 2).rem_euclid(old_valley[0].len() as i64 - 2) as usize + 1][2] = left;
                valley[(j as i64 - 2).rem_euclid(old_valley.len() as i64 - 2) as usize + 1][i][3] = up;
            }
        }
        positions = positions.iter().flat_map(|&(x,y)| [(x+1,y), (x-1,y), (x,y+1), (x,y-1), (x,y)].into_iter()).filter(|&(x,y)| x >= 0 && y >= 0 && (x as usize) < valley[0].len() && (y as usize) < valley.len() && valley[y as usize][x as usize] == [false; 4]).collect();
    }

    positions = HashSet::from([(valley[0].len() as i64 - 2, valley.len() as i64 - 1)]);

    while !positions.contains(&(1,0)) {
        steps += 1;
        let old_valley = valley.clone();
        for j in 1..(old_valley.len()-1) {
            for i in 1..(old_valley[0].len() - 1) {
                let [right, down, left, up] = old_valley[j][i];
                valley[j][i.rem_euclid(old_valley[0].len() - 2) + 1][0] = right;
                valley[j.rem_euclid(old_valley.len() - 2) + 1][i][1] = down;
                valley[j][(i as i64 - 2).rem_euclid(old_valley[0].len() as i64 - 2) as usize + 1][2] = left;
                valley[(j as i64 - 2).rem_euclid(old_valley.len() as i64 - 2) as usize + 1][i][3] = up;
            }
        }
        positions = positions.iter().flat_map(|&(x,y)| [(x+1,y), (x-1,y), (x,y+1), (x,y-1), (x,y)].into_iter()).filter(|&(x,y)| x >= 0 && y >= 0 && (x as usize) < valley[0].len() && (y as usize) < valley.len() && valley[y as usize][x as usize] == [false; 4]).collect();
    }

    positions = HashSet::from([(1,0)]);

    while !positions.contains(&(valley[0].len() as i64 - 2, valley.len() as i64 - 1)) {
        steps += 1;
        let old_valley = valley.clone();
        for j in 1..(old_valley.len()-1) {
            for i in 1..(old_valley[0].len() - 1) {
                let [right, down, left, up] = old_valley[j][i];
                valley[j][i.rem_euclid(old_valley[0].len() - 2) + 1][0] = right;
                valley[j.rem_euclid(old_valley.len() - 2) + 1][i][1] = down;
                valley[j][(i as i64 - 2).rem_euclid(old_valley[0].len() as i64 - 2) as usize + 1][2] = left;
                valley[(j as i64 - 2).rem_euclid(old_valley.len() as i64 - 2) as usize + 1][i][3] = up;
            }
        }
        positions = positions.iter().flat_map(|&(x,y)| [(x+1,y), (x-1,y), (x,y+1), (x,y-1), (x,y)].into_iter()).filter(|&(x,y)| x >= 0 && y >= 0 && (x as usize) < valley[0].len() && (y as usize) < valley.len() && valley[y as usize][x as usize] == [false; 4]).collect();
    }


    steps

    // for steps in 1.. {
    //     let old_valley = valley.clone();
    //     for j in 1..(old_valley.len()-1) {
    //         for i in 1..(old_valley[0].len() - 1) {
    //             let [right, down, left, up] = old_valley[j][i];
    //             valley[j][i.rem_euclid(old_valley[0].len() - 2) + 1][0] = right;
    //             valley[j.rem_euclid(old_valley.len() - 2) + 1][i][1] = down;
    //             valley[j][(i as i64 - 2).rem_euclid(old_valley[0].len() as i64 - 2) as usize + 1][2] = left;
    //             valley[(j as i64 - 2).rem_euclid(old_valley.len() as i64 - 2) as usize + 1][i][3] = up;
    //         }
    //     }
    //     positions = positions.iter().flat_map(|&(x,y)| [(x+1,y), (x-1,y), (x,y+1), (x,y-1), (x,y)].into_iter()).filter(|&(x,y)| x >= 0 && y >= 0 && (x as usize) < valley[0].len() && (y as usize) < valley.len() && valley[y as usize][x as usize] == [false; 4]).collect();
    //
    //     if positions.contains(&(valley[0].len() as i64 - 2, valley.len() as i64 - 1)) {
    //         break steps
    //     }
}

fn main() {
    let input = get_input();
    println!("part1: {}", part1(&input));
}
