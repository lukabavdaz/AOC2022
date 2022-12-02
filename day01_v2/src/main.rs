use std::cmp::Reverse;
use std::io::BufRead;

fn main() {
    let file = std::fs::File::open("input/input.txt").unwrap();
    let mut top_3 = std::collections::BinaryHeap::from([Reverse(0); 3]);

    let last = std::io::BufReader::new(file)
        .lines()
        .map(|l| l.unwrap().parse::<u32>().ok())
        .fold(0, |acc, c| {
            if let Some(cal) = c {
                acc + cal
            } else {
                top_3.push(Reverse(acc));
                top_3.pop();
                0
            }
        });
    top_3.push(Reverse(last));
    top_3.pop();

    let sorted_top_3: Vec<_> = top_3.into_sorted_vec().iter().map(|x| x.0).collect();
    let part1 = sorted_top_3[0];
    let part2: u32 = sorted_top_3.iter().sum();

    println!("part1: {part1}");
    println!("part2: {part2}");
}
