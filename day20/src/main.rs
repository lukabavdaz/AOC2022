fn get_input() -> Vec<i64> {
    std::fs::read_to_string("input/input.txt")
        .unwrap()
        .lines()
        .map(|l| l.parse().unwrap())
        .collect()
}

fn part1(input: &[i64]) -> i64 {
    let mut v = input.iter().map(|i| (i.clone(), true)).collect::<Vec<_>>();
    while let Some(i) = v.iter().position(|&(_,b)| b) {
        let mut new_i_raw = i as i64 + v[i].0;
        let mut div = new_i_raw.div_euclid(input.len() as i64);
        let new_i = (new_i_raw + div).rem_euclid(input.len() as i64) as usize;

        v[i].1 = false;
        if i < new_i {
            v[i..=new_i].rotate_left(1);
        } else if new_i < i {
            v[new_i..=i].rotate_right(1);
        }
    }
    let zero = v.iter().position(|&(a,_)| a == 0 ).unwrap();
    [1000,2000,3000].map(|i| v[(zero+i)%input.len()].0).iter().sum()
}

fn part2(input: &[i64]) -> i64 {
    let mut v = input.iter().map(|u| u*811589153).enumerate().collect::<Vec<_>>();
    for _ in 0..10 {
        for x in 0..input.len() {
            let i = v.iter().position(|&(u,_)| x == u).unwrap();
            let new_i = (i as i64 + v[i].1).rem_euclid(input.len() as i64 - 1) as usize;
            if i < new_i {
                v[i..=new_i].rotate_left(1);
            } else if new_i < i {
                v[new_i..=i].rotate_right(1);
            }
        }
    }

    let zero = v.iter().position(|&(_,a)| a == 0 ).unwrap();
    [1000,2000,3000].map(|i| v[(zero+i)%input.len()].1).iter().sum()
}

fn main() {
    let input = get_input();
    println!("part1: {}", part1(&input));
    println!("part2: {}", part2(&input));
}
