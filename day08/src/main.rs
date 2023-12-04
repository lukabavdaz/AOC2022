fn get_input() -> Vec<Vec<u8>> {
    std::fs::read_to_string("input/input.txt")
        .unwrap()
        .lines()
        .map(|l| l.into())
        .collect()
}

fn part1(input: &[Vec<u8>]) -> usize {
    let mut visible = vec![vec![false;input[0].len()];input.len()];

    for i in 0..input.len() {
        let mut largest = 0;
        for j in 0..input[0].len() {
            if input[i][j] > largest {
                visible[i][j] = true;
                largest = input[i][j];
            }
        }
    }
    for j in 0..input[0].len() {
        let mut largest = 0;
            for i in 0..input.len() {
            if input[i][j] > largest {
                visible[i][j] = true;
                largest = input[i][j];
            }
        }
    }
    for i in 0..input.len() {
        let mut largest = 0;
        for j in (0..input[0].len()).rev() {
            if input[i][j] > largest {
                visible[i][j] = true;
                largest = input[i][j];
            }
        }
    }
    for j in 0..input[0].len() {
        let mut largest = 0;
        for i in (0..input.len()).rev() {
            if input[i][j] > largest {
                visible[i][j] = true;
                largest = input[i][j];
            }
        }
    }

    visible.iter().flatten().filter(|&&b| b).count()
}

fn scenic_score((x,y): (usize,usize), input: &[Vec<u8>]) -> i64 {

    let mut largest = input[x][y];
    let mut visible_count1 = 0;
    for i in (x+1)..input.len() {
        if input[i][y] < largest {
            visible_count1 += 1;
        } else {
            visible_count1 += 1;
            break;
        }
    }
    let mut largest = input[x][y];
    let mut visible_count2 = 0;
    for j in (y+1)..input[0].len() {
        if input[x][j] < largest {
            visible_count2 += 1;
        } else{
            visible_count2 += 1;
            break;
        }
    }
    let mut largest = input[x][y];
    let mut visible_count3 = 0;
    for i in (0..x).rev() {
        if input[i][y] < largest {
            visible_count3 += 1;
        } else {
            visible_count3 += 1;
            break;
        }
    }
    let mut largest = input[x][y];
    let mut visible_count4 = 0;
    for j in (0..y).rev() {
        if input[x][j] < largest {
            visible_count4 += 1;
        } else {
            visible_count4 += 1;
            break;
        }
    }

    visible_count1 * visible_count2 * visible_count3 * visible_count4
}

fn part2(input: &[Vec<u8>]) -> i64 {
    let mut max_scenic_score = 0;
    for i in 0..input.len() {
        for j in 0..input[0].len() {
            max_scenic_score = max_scenic_score.max(scenic_score((i,j), input));
        }
    }
    max_scenic_score
}

fn main() {
    let input = get_input();
    println!("part1: {}", part1(&input));
    println!("part2: {}", part2(&input));
    // println!("scenic (2,1): {}", scenic_score((2,1),&input));
    // println!("scenic (2,3): {}", scenic_score((2,3),&input));
}
