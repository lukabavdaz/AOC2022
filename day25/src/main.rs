fn get_input() -> i64 {
    std::fs::read_to_string("input/input.txt")
        .unwrap()
        .lines()
        .map(|l| l.chars().map(|c| match c {
            '-' => -1 as i64,
            '=' => -2 as i64,
            n => n.to_digit(5).unwrap() as i64,
        }).fold(0,|acc,i| acc*5+i))
        .sum()
}

fn part1(input: i64) {
    let mut answer = vec![];
    let mut intermediate = input;
    while intermediate > 0 {
        answer.push((intermediate+2).rem_euclid(5) - 2);
        intermediate = (intermediate+2).div_euclid(5);
    }

    // print!("{}: ", input);
    for c in answer.iter().rev() {
        match c {
            -2 => print!("="),
            -1 => print!("-"),
            n => print!("{}", n),
        }
    }
}

fn main() {
    let input = get_input();
    part1(input);
}
